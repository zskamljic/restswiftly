use anyhow::Result;
use std::mem;
use swift_generator::{
    AccessModifier, ClassBuilder, CodeBuilder, ControlType, FieldBuilder, FunctionBuilder,
    ParameterBuilder,
};
use swift_parser::{Definition, Parameter, PostfixModifier};

use self::errors::GeneratingError;

mod errors;
mod parameters;
mod path;
mod query;
#[cfg(test)]
mod test;

pub struct Generator {
    calls: Vec<FunctionBuilder>,
    definition: Option<CallDefinition>,
}

impl Generator {
    pub fn new() -> Self {
        Generator {
            calls: vec![],
            definition: None,
        }
    }

    pub fn generate_service(
        &mut self,
        name: &str,
        definitions: &[Definition],
    ) -> Result<ClassBuilder> {
        for definition in definitions {
            self.generate_definition(definition)?;
        }
        if !matches!(self.definition, None) {
            return Err(GeneratingError::GeneralError("Not all tokens were handled".into()).into());
        }

        let mut class = ClassBuilder::new(&(name.to_owned() + "Impl"));
        class
            .add_super(name)
            .add_field(FieldBuilder {
                modifier: Some(AccessModifier::Private),
                name: "baseUrl".into(),
                field_type: "String".into(),
            })
            .add_field(FieldBuilder {
                modifier: Some(AccessModifier::Private),
                name: "interceptors".into(),
                field_type: "[Interceptor]".into(),
            })
            .add_function(make_constructor())
            .add_functions(mem::take(&mut self.calls));

        Ok(class)
    }

    fn generate_definition(&mut self, definition: &Definition) -> Result<()> {
        match definition {
            Definition::Comment(comment) => self.handle_call_definition(comment)?,
            Definition::Function {
                name,
                parameters,
                modifiers,
                return_type,
            } => self.generate_function_definition(name, parameters, modifiers, return_type)?,
            value => {
                return Err(GeneratingError::GeneralError(format!(
                    "Unsupported definition: {value:?}"
                ))
                .into())
            }
        }
        Ok(())
    }

    fn handle_call_definition(&mut self, comment: &str) -> Result<()> {
        let definition = mem::take(&mut self.definition);
        let definition = if let Some(definition) = definition {
            parse_headers(definition, comment)?
        } else {
            parameters::parse_call_definition(comment)?
        };
        self.definition = Some(definition);
        Ok(())
    }

    fn generate_function_definition(
        &mut self,
        name: &str,
        parameters: &[Parameter],
        modifiers: &[PostfixModifier],
        return_type: &Option<String>,
    ) -> Result<()> {
        let definition = mem::take(&mut self.definition);
        let definition = match definition {
            Some(definition) => definition,
            None => {
                return Err(
                    GeneratingError::GeneralError("No call definition for function".into()).into(),
                )
            }
        };
        let call = self.generate_call(name, parameters, modifiers, return_type, definition)?;
        self.calls.push(call);
        Ok(())
    }

    fn generate_call(
        &mut self,
        name: &str,
        parameters: &[Parameter],
        modifiers: &[PostfixModifier],
        return_type: &Option<String>,
        definition: CallDefinition,
    ) -> Result<FunctionBuilder> {
        parameters::ensure_present(parameters, &definition)?;

        if !modifiers.contains(&PostfixModifier::Async)
            || !modifiers.contains(&PostfixModifier::Throws)
        {
            return Err(GeneratingError::GeneralError(
                "Only async throws supported at this time".into(),
            )
            .into());
        }

        let mut failure = CodeBuilder::default();
        failure.add_statement(r#"fatalError("Unable to fetch data")"#);

        let mut code = CodeBuilder::default();
        code.add_statement(&format!(
            r#"{} url = URL(string: baseUrl + {})!"#,
            if definition.query.is_empty() {
                "let"
            } else {
                "var"
            },
            path::create_template(&definition),
        ));
        if !definition.query.is_empty() {
            query::add_parameters(&mut code, definition.query);
        }
        code.add_statement("var request = URLRequest(url: url)")
            .add_statement(&format!(r#"request.httpMethod = "{}""#, definition.verb));
        add_headers(&mut code, &definition.headers);
        if has_body(&definition.verb, parameters) {
            let encoder = select_encoder(&definition.headers)?;
            code.add_statement(&format!("let encoder = {encoder}"))
                .add_statement("request.httpBody = try encoder.encode(body)");
        }
        code.add_statement(
            "let chain = Chain(using: interceptors) { URLSession.shared.data(for: request) }",
        )
        .add_statement("let (data, response) = try await chain.proceed(with: request)")
        .add_control(
            ControlType::Guard,
            "(response as? HTTPURLResponse)?.statusCode == 200",
            failure,
        );

        if let Some(return_type) = return_type {
            code.add_statement("let decoder = JSONDecoder()")
                .add_statement(&format!(
                    "return try decoder.decode({return_type}.self, from: data)"
                ));
        } else {
            code.add_statement("print(String(data: data, encoding: .utf8)!)");
        }

        let mut function = FunctionBuilder::new(name);
        // TODO: validate all parameters used and no duplicates
        parameters
            .iter()
            .map(|p| ParameterBuilder {
                label: p.label.as_ref().cloned(),
                name: p.name.clone(),
                parameter_type: p.parameter_type.clone(),
            })
            .for_each(|p| {
                function.add_parameter(p);
            });
        function.set_async(true).set_throws(true);
        if let Some(return_type) = return_type {
            function.set_return_type(return_type);
        }
        function.add_code(code);
        Ok(function)
    }
}

fn make_constructor() -> FunctionBuilder {
    let mut trim = CodeBuilder::default();
    trim.add_statement("baseUrl = String(baseUrl.removeLast())");

    let mut code = CodeBuilder::default();
    code.add_statement("var baseUrl = baseUrl")
        .add_control(ControlType::If, r#"baseUrl.hasSuffix("/")"#, trim)
        .add_statement("self.baseUrl = baseUrl")
        .add_statement("self.interceptors = interceptors");

    let mut constructor = FunctionBuilder::new("init");
    constructor
        .add_parameter(ParameterBuilder {
            label: None,
            name: "baseUrl".into(),
            parameter_type: "String".into(),
        })
        .add_parameter(ParameterBuilder {
            label: None,
            name: "interceptors".into(),
            parameter_type: "Interceptor...".into(),
        })
        .add_code(code);

    constructor
}

fn parse_headers(mut definition: CallDefinition, header: &str) -> Result<CallDefinition> {
    let mut parts = header.splitn(2, ": ");
    let name = match parts.next() {
        None => return Ok(definition),
        Some(name) => name.to_owned(),
    };
    let value = match parts.next() {
        None => "".to_owned(),
        Some(value) => value.to_owned(),
    };
    if value.starts_with('{') && value.ends_with('}') {
        definition.headers.push((
            name,
            ParameterValue::Parameter(value[1..value.len() - 1].to_string()),
        ));
    } else {
        definition
            .headers
            .push((name, ParameterValue::Value(value)));
    }

    Ok(definition)
}

fn add_headers(code: &mut CodeBuilder, headers: &Vec<(String, ParameterValue)>) {
    for (header, value) in headers {
        let value = match value {
            ParameterValue::Parameter(name) => name.into(),
            ParameterValue::Value(value) => format!(r#""{value}""#),
            ParameterValue::None => r#""""#.to_owned(),
        };
        if header.to_lowercase() == "content-type" && value == r#""multipart/form-data""# {
            code.add_statement("let boundary = UUID().uuidString");
            code.add_statement(&format!(
                r#"request.addValue("multipart/form-data; boundary=\(boundary)", forHTTPHeaderField: "{header}")"#
            ));
        } else {
            code.add_statement(&format!(
                r#"request.addValue({value}, forHTTPHeaderField: "{header}")"#
            ));
        }
    }
}

fn select_encoder(headers: &Vec<(String, ParameterValue)>) -> Result<String> {
    for (name, value) in headers {
        if name.to_lowercase() != "content-type" {
            continue;
        }
        match value {
            ParameterValue::None => {
                return Err(
                    GeneratingError::GeneralError("content type must not be empty".into()).into(),
                )
            }
            ParameterValue::Parameter(_) => {
                return Err(GeneratingError::GeneralError(
                    "content type must not be variable".into(),
                )
                .into())
            }
            ParameterValue::Value(value) => {
                return match value.as_str() {
                    "application/json" => Ok("JSONEncoder()".into()),
                    "application/x-www-form-urlencoded" => Ok("FormEncoder()".into()),
                    "multipart/form-data" => Ok("MultipartEncoder(boundary: boundary)".into()),
                    value => {
                        Err(GeneratingError::GeneralError(format!("{value} not supported")).into())
                    }
                }
            }
        }
    }

    Ok("JSONEncoder()".into())
}

struct CallDefinition {
    verb: String,
    headers: Vec<(String, ParameterValue)>,
    path: String,
    path_params: Vec<String>,
    query: Vec<(String, ParameterValue)>,
}

fn has_body(verb: &str, parameters: &[Parameter]) -> bool {
    parameters
        .iter()
        .map(|p| p.name.clone())
        .any(|p| p == "body")
        && matches!(verb, "PATCH" | "POST" | "PUT")
}

#[derive(Debug)]
enum ParameterValue {
    None,
    Parameter(String),
    Value(String),
}

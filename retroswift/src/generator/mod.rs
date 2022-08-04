use anyhow::Result;
use std::mem;
use swift_generator::{ClassBuilder, CodeBuilder, ControlType, FunctionBuilder};
use swift_parser::{Definition, PostfixModifier};

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
            panic!("There were tokens remaining");
        }

        let mut class = ClassBuilder::new(&(name.to_owned() + "Impl"));
        class.add_super(name);
        class.add_functions(mem::take(&mut self.calls));

        Ok(class)
    }

    fn generate_definition(&mut self, definition: &Definition) -> Result<()> {
        match definition {
            Definition::Comment(comment) => self.handle_call_definition(comment),
            Definition::Function {
                name,
                modifiers,
                return_type,
            } => self.generate_function_definition(name, modifiers, return_type)?,
            _ => panic!("Unsupported definition"),
        }
        Ok(())
    }

    fn handle_call_definition(&mut self, comment: &str) {
        let definition = mem::take(&mut self.definition);
        match (definition, Generator::parse_call_definition(comment)) {
            (None, Ok(value)) => self.definition = Some(value),
            (Some(_), Ok(_)) => panic!("Repeat call definition"),
            (_, Err(err)) => {
                log::warn!("Failed to get definition: {err}");
                panic!("Not handled");
            }
        }
    }

    fn generate_function_definition(
        &mut self,
        name: &str,
        modifiers: &[PostfixModifier],
        return_type: &Option<String>,
    ) -> Result<()> {
        let definition = mem::take(&mut self.definition);
        let definition = match definition {
            Some(definition) => definition,
            None => panic!("No call definition for function"),
        };
        let call = self.generate_call(name, modifiers, return_type, definition)?;
        self.calls.push(call);
        Ok(())
    }

    fn generate_call(
        &mut self,
        name: &str,
        modifiers: &[PostfixModifier],
        return_type: &Option<String>,
        definition: CallDefinition,
    ) -> Result<FunctionBuilder> {
        if !modifiers.contains(&PostfixModifier::Async)
            || !modifiers.contains(&PostfixModifier::Throws)
        {
            panic!("Only async throws supported at this time");
        }

        let mut failure = CodeBuilder::default();
        failure.add_statement(r#"fatalError("Unable to fetch data")"#);

        let mut code = CodeBuilder::default();
        code.add_statement(&format!(
            r#"let url = URL("https://httpbin.org{}")!"#,
            definition.path
        ))
        .add_statement("var request = URLRequest(url: url)")
        .add_statement(&format!(r#"request.httpMethod = "{}""#, definition.verb))
        .add_statement("let (data, response) = try await URLSession.shared.data(for: request)")
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
            let mut success = CodeBuilder::default();
            success.add_statement("print(String(data: data, encoding: .utf8)!)");
            code.add_control(ControlType::If, "let data = data", success);
        }

        let mut function = FunctionBuilder::new(name);
        function.set_async(true).set_throws(true);
        if let Some(return_type) = return_type {
            function.set_return_type(return_type);
        }
        function.add_code(code);
        Ok(function)
    }

    fn parse_call_definition(call: &str) -> Result<CallDefinition> {
        let mut parts = call.split_whitespace();
        let verb = match parts.next() {
            Some(verb) => verb.to_owned(),
            None => panic!("Call verb not present"),
        };
        let allowed_verbs = vec![
            "DELETE".to_owned(),
            "GET".to_owned(),
            "PATCH".to_owned(),
            "POST".to_owned(),
            "PUT".to_owned(),
        ];
        if !allowed_verbs.contains(&verb) {
            panic!("Invalid request verb");
        }

        let path = match parts.next() {
            Some(path) => path.to_owned(),
            None => panic!("Call path not present"),
        };
        Ok(CallDefinition { verb, path })
    }
}

struct CallDefinition {
    verb: String,
    path: String,
}

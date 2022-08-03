use core::panic;
use std::fs::File;

use anyhow::Result;
use swift_generator::{ClassBuilder, CodeBuilder, ControlType, FunctionBuilder, Options};
use swift_parser::Definition;

fn main() -> Result<()> {
    env_logger::init();

    let input_file = File::open("samples/Simple.swift")?;
    let definitions = swift_parser::read_definitions(input_file)?;

    for definition in definitions.into_iter() {
        generate_service(&definition)?;
    }

    Ok(())
}

fn generate_service(definition: &Definition) -> Result<()> {
    let (name, definitions) = match definition {
        Definition::Protocol(name, definitions) => (name, definitions),
        _ => return Ok(()),
    };

    let mut generated_calls = vec![];

    let mut call_definition = None;
    for definition in definitions {
        match definition {
            Definition::Comment(comment) => match (call_definition, parse_call_definition(comment))
            {
                (None, Ok(value)) => call_definition = Some(value),
                (Some(_), Ok(_)) => panic!("Repeat call definition"),
                (_, Err(err)) => {
                    log::warn!("Failed to get definition: {err}");
                    panic!("Not handled");
                }
            },
            Definition::Function {
                name,
                is_async,
                is_throws,
            } => {
                let definition = match call_definition {
                    Some(definition) => definition,
                    None => panic!("No call definition for function"),
                };
                call_definition = None;
                generated_calls.push(generate_call(name, *is_async, *is_throws, definition)?);
            }
            _ => panic!("Unsupported definition"),
        }
    }

    let mut class = ClassBuilder::new(&(name.to_owned() + "Impl"));
    class.add_super(name);
    class.add_functions(generated_calls);

    class.generate(&mut File::create("out.swift")?, &Options::default())
}

fn generate_call(
    name: &str,
    is_async: bool,
    is_throws: bool,
    definition: CallDefinition,
) -> Result<FunctionBuilder> {
    if !is_async || !is_throws {
        panic!("Only async throws supported at this time");
    }

    let mut failure = CodeBuilder::default();
    failure.add_statement(r#"fatalError("Unable to fetch data")"#);

    let mut success = CodeBuilder::default();
    success.add_statement("print(String(data: data, encoding: .utf8)!)");

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
    )
    .add_control(ControlType::If, "let data = data", success);

    let mut function = FunctionBuilder::new(name);
    function.set_async(true).set_throws(true).add_code(code);
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
        "HEAD".to_owned(),
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

struct CallDefinition {
    verb: String,
    path: String,
}

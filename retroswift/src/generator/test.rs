use std::{
    fs::{self, File},
    str::from_utf8,
};

use anyhow::Result;
use swift_generator::Options;
use swift_parser::Definition;

use crate::generator::Generator;

use super::errors::GeneratingError;

#[test]
fn generated_files_correct() -> Result<()> {
    for file in vec![
        "Simple",
        "AllMethods",
        "Return",
        "QueryParameter",
        "Path",
        "Body",
    ]
    .into_iter()
    {
        generated_file_correct(file)?;
        println!("File correct: {file}");
    }
    Ok(())
}

#[test]
fn file_fails_generation() -> Result<()> {
    let input_file = File::open(format!("../samples/Failing.swift"))?;
    let definitions = swift_parser::read_definitions(input_file)?;

    let (name, definitions) = match definitions.into_iter().next() {
        Some(Definition::Protocol(name, definitions)) => (name, definitions),
        _ => panic!("Invalid test definition"),
    };

    let result = Generator::new().generate_service(&name, &definitions);
    match result {
        Ok(_) => panic!("Expected failure"),
        Err(error) => {
            let error: GeneratingError = error.downcast()?;
            if let GeneratingError::UnusedParameters(parameters) = error {
                assert_eq!(vec!["query".to_string()], parameters);
            } else {
                panic!("Unexpected error variant: {error}");
            }
        }
    }

    Ok(())
}

fn generated_file_correct(name: &str) -> Result<()> {
    let input_file = File::open(format!("../samples/{name}.swift"))?;
    let definitions = swift_parser::read_definitions(input_file)?;
    assert_eq!(1, definitions.len()); // sanity check, only expect one file per test

    let (name, definitions) = match definitions.into_iter().next() {
        Some(Definition::Protocol(name, definitions)) => (name, definitions),
        _ => panic!("Invalid test definition"),
    };

    let class = Generator::new().generate_service(&name, &definitions)?;

    let mut output = vec![];
    class.generate(&mut output, &Options::default())?;

    let output = from_utf8(&output)?;
    let expected = fs::read(format!("../samples/outputs/{name}Impl.swift"))?;
    let expected = from_utf8(&expected)?;

    assert_eq!(expected, output);
    Ok(())
}

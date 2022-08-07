use std::{
    fs::{self, File},
    str::from_utf8,
};

use anyhow::Result;
use swift_generator::Options;
use swift_parser::Definition;

use crate::generator::Generator;

#[test]
fn test_generated_files() -> Result<()> {
    for file in vec!["Simple", "AllMethods", "Return", "QueryParameter"].into_iter() {
        test_generated_file(file)?;
        println!("File correct: {file}");
    }
    Ok(())
}

fn test_generated_file(name: &str) -> Result<()> {
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

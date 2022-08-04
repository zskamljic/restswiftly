use std::fs::File;

use anyhow::Result;
use args::Args;
use clap::Parser;
use generator::Generator;
use swift_generator::Options;
use swift_parser::Definition;

mod args;
mod generator;

fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();

    let input_file = File::open(&args.file_name)?;
    let definitions = swift_parser::read_definitions(input_file)?;

    for definition in definitions.into_iter() {
        generate_service(&args.output, &definition)?;
    }

    Ok(())
}

fn generate_service(out_file: &str, definition: &Definition) -> Result<()> {
    let (name, definitions) = match definition {
        Definition::Protocol(name, definitions) => (name, definitions),
        _ => return Ok(()),
    };

    let class = Generator::new().generate_service(name, definitions)?;

    class.generate(&mut File::create(out_file)?, &Options::default())
}

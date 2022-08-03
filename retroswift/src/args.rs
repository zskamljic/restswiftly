use clap::Parser;

#[derive(Parser)]
#[clap(version)]
pub struct Args {
    #[clap(short, long, value_parser)]
    pub file_name: String,
    #[clap(short, long, value_parser, default_value = "out.swift")]
    pub output: String,
}

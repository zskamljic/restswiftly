use std::io::Write;

use anyhow::Result;

use crate::{write_indent, writeln_indent, FunctionBuilder, Options};

pub struct ClassBuilder {
    name: String,
    functions: Vec<FunctionBuilder>,
    supers: Vec<String>,
}

impl ClassBuilder {
    pub fn new(name: &str) -> ClassBuilder {
        ClassBuilder {
            name: name.to_owned(),
            functions: vec![],
            supers: vec![],
        }
    }

    pub fn add_super(&mut self, super_type: &str) {
        self.supers.push(super_type.to_owned());
    }

    pub fn add_function(&mut self, function: FunctionBuilder) {
        self.functions.push(function);
    }

    pub fn add_functions(&mut self, functions: Vec<FunctionBuilder>) {
        for function in functions {
            self.add_function(function);
        }
    }

    pub fn generate(self, writer: &mut impl Write, options: &Options) -> Result<()> {
        let indent = options.indent.unwrap_or(0);

        write_indent!(writer, indent, "class {}", self.name)?;
        if !self.supers.is_empty() {
            write_indent!(writer, indent, ": ")?;
            write_indent!(writer, indent, "{}", self.supers.join(", "))?;
        }
        writeln_indent!(writer, indent, " {{")?;

        for function in self.functions {
            function.generate(writer, Options::default().indent(4))?;
            writeln!(writer)?;
        }

        writeln_indent!(writer, indent, "}}")?;

        Ok(())
    }
}

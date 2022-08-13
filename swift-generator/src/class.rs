use std::io::Write;

use anyhow::Result;

use crate::{field::FieldBuilder, write_indent, writeln_indent, FunctionBuilder, Options};

pub struct ClassBuilder {
    name: String,
    fields: Vec<FieldBuilder>,
    functions: Vec<FunctionBuilder>,
    supers: Vec<String>,
}

impl ClassBuilder {
    pub fn new(name: &str) -> ClassBuilder {
        ClassBuilder {
            fields: vec![],
            name: name.to_owned(),
            functions: vec![],
            supers: vec![],
        }
    }

    pub fn add_super(&mut self, super_type: &str) -> &mut Self {
        self.supers.push(super_type.to_owned());
        self
    }

    pub fn add_field(&mut self, field: FieldBuilder) -> &mut Self {
        self.fields.push(field);
        self
    }

    pub fn add_function(&mut self, function: FunctionBuilder) -> &mut Self {
        self.functions.push(function);
        self
    }

    pub fn add_functions(&mut self, functions: Vec<FunctionBuilder>) -> &mut Self {
        for function in functions {
            self.add_function(function);
        }
        self
    }

    pub fn generate(&self, writer: &mut impl Write, options: &Options) -> Result<()> {
        let indent = options.indent.unwrap_or(0);

        self.generate_start(writer, indent)?;
        self.generate_fields(writer, indent)?;
        if !self.fields.is_empty() {
            writeln!(writer)?;
        }
        self.generate_functions(writer, indent)?;

        writeln_indent!(writer, indent, "}}")?;

        Ok(())
    }

    fn generate_start(&self, writer: &mut impl Write, indent: u8) -> Result<()> {
        write_indent!(writer, indent, "class {}", self.name)?;
        if !self.supers.is_empty() {
            write_indent!(writer, indent, ": ")?;
            write_indent!(writer, indent, "{}", self.supers.join(", "))?;
        }
        writeln_indent!(writer, indent, " {{")?;
        Ok(())
    }

    fn generate_fields(&self, writer: &mut impl Write, indent: u8) -> Result<()> {
        for field in &self.fields {
            field.generate(writer, Options::default().indent(indent + 4))?;
        }
        Ok(())
    }

    fn generate_functions(&self, writer: &mut impl Write, indent: u8) -> Result<()> {
        for function in &self.functions {
            function.generate(writer, Options::default().indent(indent + 4))?;
            writeln!(writer)?;
        }
        Ok(())
    }
}

use std::io::Write;

use anyhow::Result;

use crate::{write_indent, writeln_indent, CodeBuilder, Options, ParameterBuilder, DEFAULT_INDENT};

pub struct FunctionBuilder {
    name: String,
    parameters: Vec<ParameterBuilder>,
    code: Vec<CodeBuilder>,
    is_async: bool,
    is_throws: bool,
    return_type: Option<String>,
}

impl FunctionBuilder {
    pub fn new(name: &str) -> FunctionBuilder {
        FunctionBuilder {
            name: name.to_owned(),
            parameters: vec![],
            code: vec![],
            is_async: false,
            is_throws: false,
            return_type: None,
        }
    }

    pub fn add_parameter(&mut self, parameter: ParameterBuilder) -> &mut Self {
        self.parameters.push(parameter);
        self
    }

    pub fn set_async(&mut self, is_async: bool) -> &mut Self {
        self.is_async = is_async;
        self
    }

    pub fn set_throws(&mut self, is_throws: bool) -> &mut Self {
        self.is_throws = is_throws;
        self
    }

    pub fn add_code(&mut self, code: CodeBuilder) -> &mut Self {
        self.code.push(code);
        self
    }

    pub fn set_return_type(&mut self, return_type: &str) -> &mut Self {
        self.return_type = Some(return_type.into());
        self
    }

    pub fn generate(&self, writer: &mut impl Write, options: &Options) -> Result<()> {
        let indent = options.indent.unwrap_or(0);

        if self.name != "init" {
            write_indent!(writer, indent, "func {}(", self.name)?;
            self.generate_parameters(writer)?;
            write!(writer, ") ")?;
        } else {
            write_indent!(writer, indent, "init(")?;
            self.generate_parameters(writer)?;
            write!(writer, ") ")?;
        }
        if self.is_async {
            write!(writer, "async ")?;
        }
        if self.is_throws {
            write!(writer, "throws ")?;
        }
        if let Some(return_type) = &self.return_type {
            write!(writer, "-> {return_type} ")?;
        }
        writeln!(writer, "{{")?;

        for statement in &self.code {
            statement.generate(writer, Options::default().indent(indent + DEFAULT_INDENT))?;
        }

        writeln_indent!(writer, indent, "}}")?;
        Ok(())
    }

    fn generate_parameters(&self, writer: &mut impl Write) -> Result<()> {
        for i in 0..self.parameters.len() {
            let parameter = &self.parameters[i];
            parameter.generate(writer)?;
            if i < self.parameters.len() - 1 {
                write!(writer, ",")?;
            }
        }
        Ok(())
    }
}

use std::io::Write;

use anyhow::Result;

use crate::{write_indent, writeln_indent, CodeBuilder, Options, DEFAULT_INDENT};

pub struct FunctionBuilder {
    name: String,
    code: Vec<CodeBuilder>,
    is_async: bool,
    is_throws: bool,
}

impl FunctionBuilder {
    pub fn new(name: &str) -> FunctionBuilder {
        FunctionBuilder {
            name: name.to_owned(),
            code: vec![],
            is_async: false,
            is_throws: false,
        }
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

    pub fn generate(self, writer: &mut impl Write, options: &Options) -> Result<()> {
        let indent = options.indent.unwrap_or(0);

        write_indent!(writer, indent, "func {}() ", self.name)?;
        if self.is_async {
            write!(writer, "async ")?;
        }
        if self.is_throws {
            write!(writer, "throws ")?;
        }
        writeln!(writer, "{{")?;

        for statement in self.code {
            statement.generate(writer, Options::default().indent(indent + DEFAULT_INDENT))?;
        }

        writeln_indent!(writer, indent, "}}")?;
        Ok(())
    }
}

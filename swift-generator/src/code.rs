use crate::Options;
use crate::DEFAULT_INDENT;

use std::io::Write;

use anyhow::Result;

use crate::writeln_indent;

#[derive(Default)]
pub struct CodeBuilder {
    lines: Vec<Code>,
}

impl CodeBuilder {
    pub fn add_control(
        &mut self,
        control_type: ControlType,
        condition: &str,
        code: CodeBuilder,
    ) -> &mut Self {
        let start = match control_type {
            ControlType::If => format!("if {condition} {{"),
            ControlType::Guard => format!("guard {condition} else {{"),
        };

        self.lines.push(Code::ControlFlow {
            start,
            end: "}".into(),
            code,
        });
        self
    }

    pub fn add_statement(&mut self, statement: &str) -> &mut Self {
        self.lines.push(Code::Line(statement.into()));
        self
    }

    pub fn generate(&self, writer: &mut impl Write, options: &Options) -> Result<()> {
        let indent = options.indent.unwrap_or(0);

        for line in &self.lines {
            match line {
                Code::Line(line) => writeln_indent!(writer, indent, "{line}")?,
                Code::ControlFlow { start, end, code } => {
                    writeln_indent!(writer, indent, "{start}")?;
                    code.generate(writer, Options::default().indent(indent + DEFAULT_INDENT))?;
                    writeln_indent!(writer, indent, "{end}")?;
                }
            }
        }
        Ok(())
    }
}

pub enum ControlType {
    If,
    Guard,
}

enum Code {
    Line(String),
    ControlFlow {
        start: String,
        end: String,
        code: CodeBuilder,
    },
}

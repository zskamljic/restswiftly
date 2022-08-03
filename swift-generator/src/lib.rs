use std::io::Write;

use anyhow::Result;

const DEFAULT_INDENT: u8 = 4;

macro_rules! write_indent {
    ($dst:expr, $indent:expr, $($arg:tt)*) => {
        {
            $dst.write_all(" ".repeat($indent as usize).as_bytes())?;
            write!($dst, $($arg)*)
        }
    };
}

macro_rules! writeln_indent {
    ($dst:expr $(,)?) => {
        $crate::write!($dst, "\n")
    };
    ($dst:expr, $indent:expr, $($arg:tt)*) => {
        {
            $dst.write_all(" ".repeat($indent as usize).as_bytes())?;
            writeln!($dst, $($arg)*)
        }
    };
}

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

    pub fn generate(self, writer: &mut impl Write, options: &Options) -> Result<()> {
        let indent = options.indent.unwrap_or(0);

        for line in self.lines.into_iter() {
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

#[derive(Default)]
pub struct Options {
    indent: Option<u8>,
}

impl Options {
    pub fn indent(&mut self, count: u8) -> &Self {
        self.indent = Some(count);
        self
    }
}

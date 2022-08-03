mod class;
pub use class::ClassBuilder;

mod code;
pub use code::CodeBuilder;
pub use code::ControlType;

mod function;
pub use function::FunctionBuilder;

mod macros;

pub(crate) const DEFAULT_INDENT: u8 = 4;

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

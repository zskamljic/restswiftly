use std::io::Write;

use anyhow::Result;

use crate::{write_indent, AccessModifier, Options, DEFAULT_INDENT};

pub struct FieldBuilder {
    pub modifier: Option<AccessModifier>,
    pub name: String,
    pub field_type: String,
}

impl FieldBuilder {
    pub fn generate(&self, writer: &mut impl Write, options: &Options) -> Result<()> {
        let indent = options.indent.unwrap_or(DEFAULT_INDENT);

        if let Some(modifier) = &self.modifier {
            let modifier = match modifier {
                AccessModifier::FilePrivate => "fileprivate",
                AccessModifier::Internal => "internal",
                AccessModifier::Private => "private",
                AccessModifier::Public => "public",
            };
            write_indent!(writer, indent, "{modifier} ")?;
        } else {
            write_indent!(writer, indent)?;
        }
        writeln!(writer, "let {}: {}", self.name, self.field_type)?;

        Ok(())
    }
}

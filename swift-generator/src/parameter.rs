use std::io::Write;

use anyhow::Result;

pub struct ParameterBuilder {
    pub label: Option<String>,
    pub name: String,
    pub parameter_type: String,
}

impl ParameterBuilder {
    pub fn generate(&self, writer: &mut impl Write) -> Result<()> {
        if let Some(label) = &self.label {
            write!(writer, "{label} ")?;
        }
        write!(writer, "{}: ", self.name)?;
        write!(writer, "{}", self.parameter_type)?;
        Ok(())
    }
}

use anyhow::Result;
use swift_parser::Parameter;

use super::{errors::GeneratingError, CallDefinition, QueryValue};

pub(super) fn ensure_present(parameters: &[Parameter], definition: &CallDefinition) -> Result<()> {
    let mut names: Vec<_> = parameters.iter().map(|p| p.name.clone()).collect();

    for (_, query) in &definition.query {
        if let QueryValue::Parameter(name) = query {
            if let Some(index) = names.iter().position(|n| n == name) {
                names.remove(index);
            } else {
                return Err(GeneratingError::MissingParameter(name.clone()).into());
            }
        };
    }
    if !names.is_empty() {
        return Err(GeneratingError::UnusedParameters(names).into());
    }
    Ok(())
}

use anyhow::Result;
use regex::Regex;
use std::fmt::Write;

use super::CallDefinition;

pub(super) fn create_template(definition: &CallDefinition) -> String {
    let mut path = format!(r#""{}""#, definition.path);

    for param in &definition.path_params {
        write!(
            path,
            r#".replacingOccurrences(of: "{{{param}}}", with: {param})"#
        )
        .expect("Unable to concat string");
    }
    path
}

pub(super) fn parse_params(path: &str) -> Result<Vec<String>> {
    let mut parameters = vec![];

    let path_matcher = Regex::new("\\{(\\w[\\w\\d]+)\\}")?;
    for caps in path_matcher.captures_iter(path) {
        if let Some(capture) = caps.get(1) {
            parameters.push(capture.as_str().to_owned());
        }
    }

    Ok(parameters)
}

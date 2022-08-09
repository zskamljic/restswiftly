use anyhow::Result;
use swift_parser::Parameter;

use super::{errors::GeneratingError, path, query, CallDefinition, QueryValue};

pub(super) fn ensure_present(parameters: &[Parameter], definition: &CallDefinition) -> Result<()> {
    let mut names: Vec<_> = parameters.iter().map(|p| p.name.clone()).collect();

    filter_query(&mut names, &definition.query)?;
    filter_path(&mut names, &definition.path_params)?;
    if !names.is_empty() {
        return Err(GeneratingError::UnusedParameters(names).into());
    }
    Ok(())
}

fn filter_query(names: &mut Vec<String>, query: &Vec<(String, QueryValue)>) -> Result<()> {
    for (_, query) in query {
        if let QueryValue::Parameter(name) = query {
            if let Some(index) = names.iter().position(|n| n == name) {
                names.remove(index);
            } else {
                return Err(GeneratingError::MissingParameter(name.clone()).into());
            }
        };
    }
    Ok(())
}

fn filter_path(names: &mut Vec<String>, path: &Vec<String>) -> Result<()> {
    for parameter in path {
        if let Some(index) = names.iter().position(|n| n == parameter) {
            names.remove(index);
        } else {
            return Err(GeneratingError::MissingParameter(parameter.clone()).into());
        }
    }
    Ok(())
}

pub(super) fn parse_call_definition(call: &str) -> Result<CallDefinition> {
    let mut parts = call.split_whitespace();
    let verb = match parts.next() {
        Some(verb) => verb.to_owned(),
        None => {
            return Err(GeneratingError::GeneralError("Call verb was not present".into()).into())
        }
    };
    let allowed_verbs = vec![
        "DELETE".to_owned(),
        "GET".to_owned(),
        "PATCH".to_owned(),
        "POST".to_owned(),
        "PUT".to_owned(),
    ];
    if !allowed_verbs.contains(&verb) {
        return Err(GeneratingError::GeneralError("Invalid request verb".into()).into());
    }

    let path = match parts.next() {
        Some(path) => path.to_owned(),
        None => {
            return Err(GeneratingError::GeneralError("Call path was not present".into()).into())
        }
    };
    if let Some(value) = parts.next() {
        return Err(GeneratingError::GeneralError(format!(
            "Call format should be in format <VERB> /path?with=query, unknown token: {value}"
        ))
        .into());
    }
    if !path.starts_with('/') {
        return Err(GeneratingError::GeneralError("Path must start with /".into()).into());
    }
    let mut path_parts = path.splitn(2, '?');
    let path = path_parts
        .next()
        .map(|p| p.to_string())
        .ok_or_else(|| GeneratingError::GeneralError("Unable to split path and query".into()))?;
    let query_params = query::parse_params(path_parts.next())?;
    let path_params = path::parse_params(&path)?;
    Ok(CallDefinition {
        verb,
        path,
        path_params,
        query: query_params,
    })
}

use std::collections::HashMap;

use anyhow::Result;
use swift_parser::Parameter;

use super::{errors::GeneratingError, path, query, CallDefinition, QueryValue};

pub(super) fn ensure_present(parameters: &[Parameter], definition: &CallDefinition) -> Result<()> {
    let mut names: HashMap<_, _> = parameters
        .iter()
        .map(|p| (p.name.clone(), p.parameter_type.clone()))
        .collect();

    if names.contains_key("body") {
        match definition.verb.as_ref() {
            "PATCH" | "POST" | "PUT" => {
                names.remove("body");
            }
            other => {
                return Err(GeneratingError::GeneralError(format!(
                    "{other} does not support sending a body"
                ))
                .into());
            }
        }
    }

    filter_query(&mut names, &definition.query)?;
    filter_path(&mut names, &definition.path_params)?;
    if !names.is_empty() {
        return Err(GeneratingError::UnusedParameters(
            names.keys().map(|s| s.to_owned()).collect(),
        )
        .into());
    }
    Ok(())
}

fn filter_query(
    parameters: &mut HashMap<String, String>,
    query: &Vec<(String, QueryValue)>,
) -> Result<()> {
    for (_, query) in query {
        if let QueryValue::Parameter(name) = query {
            remove_string_param(parameters, name)?;
        };
    }
    Ok(())
}

fn filter_path(parameters: &mut HashMap<String, String>, path: &Vec<String>) -> Result<()> {
    for parameter in path {
        remove_string_param(parameters, parameter)?;
    }
    Ok(())
}

fn remove_string_param(parameters: &mut HashMap<String, String>, parameter: &str) -> Result<()> {
    if parameters.contains_key(parameter) {
        let param = parameters.remove(parameter).unwrap();
        match param.as_str() {
            "String" => (),
            other => {
                return Err(GeneratingError::GeneralError(format!("Invalid type: {other}")).into())
            }
        }
        Ok(())
    } else {
        Err(GeneratingError::MissingParameter(parameter.to_owned()).into())
    }
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

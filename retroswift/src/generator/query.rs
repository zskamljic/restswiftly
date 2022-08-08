use anyhow::Result;
use swift_generator::CodeBuilder;

use super::{errors::GeneratingError, QueryValue};

pub(super) fn add_parameters(code: &mut CodeBuilder, query: Vec<(String, QueryValue)>) {
    code.add_statement("var urlComponents = URLComponents(string: url.absoluteString)!")
        .add_statement("var queryItems = urlComponents.queryItems ?? []");
    query.into_iter().for_each(|(name, value)| {
        let statement = match value {
            QueryValue::None => {
                format!(r#"queryItems.append(URLQueryItem(name: "{name}", value: nil))"#,)
            }
            QueryValue::Parameter(parameter) => {
                format!(r#"queryItems.append(URLQueryItem(name: "{name}", value: {parameter}))"#,)
            }
            QueryValue::Value(value) => {
                format!(r#"queryItems.append(URLQueryItem(name: "{name}", value: "{value}"))"#,)
            }
        };
        code.add_statement(&statement);
    });
    code.add_statement("urlComponents.queryItems = queryItems")
        .add_statement("url = urlComponents.url!");
}

pub(super) fn parse_params(query: Option<&str>) -> Result<Vec<(String, QueryValue)>> {
    let query = match query {
        Some(value) => value,
        None => return Ok(vec![]),
    };
    let mut query_values = vec![];
    for query_item in query.split('&') {
        let mut parts = query_item.split('=');
        let name = parts
            .next()
            .ok_or_else(|| GeneratingError::GeneralError("Query name required".into()))?
            .to_string();
        let value = match parts.next() {
            Some(value) => {
                if let Some(suffix) = value.strip_prefix(':') {
                    QueryValue::Parameter(suffix.into())
                } else {
                    QueryValue::Value(value.into())
                }
            }
            None => QueryValue::None,
        };

        query_values.push((name, value))
    }

    Ok(query_values)
}

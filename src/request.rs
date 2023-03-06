use std::collections::HashMap;

use tiny_http::{HTTPVersion, Header, Method};

/// Wrapping request type, this should usually be constructed from a tiny_http::Request internally
/// by calling Into<Request>

pub struct Request<'a> {
    url_values: HashMap<String, String>,

    pub url: &'a str,
    pub secure: bool,
    pub method: &'a Method,
    pub http_version: &'a HTTPVersion,
    pub headers: &'a [Header],
    pub body_length: Option<usize>,
}

impl<'a> Request<'a> {
    pub(crate) fn from_request( request: &'a tiny_http::Request, url_values: HashMap<String, String>) -> Self {
        Self {
            url_values: url_values,

            url: request.url(),

            secure: request.secure(),

            method: request.method(),

            http_version: request.http_version(),

            headers: request.headers(),

            body_length: request.body_length(),
        }
    }

    pub(crate) fn insert_url_value(
        &mut self,
        field: impl Into<String>,
        value: impl Into<String>,
    ) -> Option<String> {
        self.url_values.insert(field.into(), value.into())
    }

    pub fn get_url_value(&self, field: &str) -> Option<&String> {
        self.url_values.get(field)
    }
}

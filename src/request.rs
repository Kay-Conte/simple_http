use std::collections::HashMap;

use tiny_http::{HTTPVersion, Header, Method};

/// Wrapping request type, this should only be constructed from a tiny_http::Request internally.
/// This is passed to all systems in an application.

pub struct Request<'a> {
    url_values: HashMap<String, String>,

    /// Valid url of request not including domain
    pub url: &'a str,

    /// Whether or not the connection is secure
    pub secure: bool,

    /// Http method specified i.e. "Post" or "Get"
    pub method: &'a Method,

    /// Http version specified
    pub http_version: &'a HTTPVersion,

    /// Slice containing all headers
    pub headers: &'a [Header],

    /// Length of the body
    pub body_length: Option<usize>,
}

impl<'a> Request<'a> {
    pub(crate) fn from_request(
        request: &'a tiny_http::Request,
        url_values: HashMap<String, String>,
    ) -> Self {
        Self {
            url_values,

            url: request.url(),

            secure: request.secure(),

            method: request.method(),

            http_version: request.http_version(),

            headers: request.headers(),

            body_length: request.body_length(),
        }
    }

    /// Get a url value from the inner map. See the `param` field at `Service#param`
    pub fn get_url_value(&self, field: &str) -> Option<&String> {
        self.url_values.get(field)
    }
}

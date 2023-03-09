use std::{collections::HashMap, io::Read};

use tiny_http::{Header};

/// Wrapping request type, this should only be constructed from a tiny_http::Request internally.
/// This is passed to all systems in an application.

pub struct Request<'a> {
    url_values: HashMap<String, String>,

    inner: &'a mut tiny_http::Request,
}

impl<'a> Request<'a> {
    pub(crate) fn from_request(
        request: &'a mut tiny_http::Request,
        url_values: HashMap<String, String>,
    ) -> Self {
        Self {
            url_values,

            inner: request
        }
    }

    pub fn url(&self) -> &str {
        self.inner.url()
    }

    pub fn headers(&self) -> &[Header] {
        self.inner.headers()
    }

    pub fn as_reader(&mut self) -> &mut dyn Read {
        self.inner.as_reader()
    }

    pub fn body_to_string(&mut self) -> std::io::Result<String> {
        let mut body_buf = String::new();
        
        self.as_reader().read_to_string(&mut body_buf)?;

        Ok(body_buf)
    }

    pub fn body_length(&self) -> Option<usize> {
        self.inner.body_length()
    }

    /// Get a url value from the inner map. See the `param` field at `Service#param`
    pub fn get_url_value(&self, field: &str) -> Option<&String> {
        self.url_values.get(field)
    }
}

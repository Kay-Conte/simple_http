use std::{fs::File, io::Read, sync::mpsc::Receiver};

use tiny_http::{Header, StatusCode};

/// Wrapping response body intended to be returned by services.
pub struct Response(tiny_http::ResponseBox);

impl From<Response> for tiny_http::ResponseBox {
    fn from(value: Response) -> Self {
        value.0
    }
}

impl Response {
    /// General response constructor
    pub fn new<R: Read + Send + 'static>(
        status_code: StatusCode,
        headers: Vec<Header>,
        data: R,
        data_length: Option<usize>,
        additional_headers: Option<Receiver<Header>>,
    ) -> Self {
        Self(
            tiny_http::Response::new(status_code, headers, data, data_length, additional_headers)
                .boxed(),
        )
    }

    /// Status code 200 with file in the body
    pub fn file(file: File) -> Self {
        Self(tiny_http::Response::from_file(file).boxed())
    }

    /// Empty response used to send status codes
    pub fn empty(status_code: StatusCode) -> Self {
        Self(tiny_http::Response::empty(status_code).boxed())
    }

    /// Insert a header to the underlying `Response` object
    pub fn with_header<H>(mut self, header: H) -> Self
    where
        H: Into<Header>,
    {
        self.0 = self.0.with_header(header);

        self
    }
}

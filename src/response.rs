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
        Response(
            tiny_http::Response::new(status_code, headers, data, data_length, additional_headers)
                .boxed(),
        )
    }

    /// Empty response used to send status codes
    pub fn empty(status_code: StatusCode) -> Self {
        Response(tiny_http::Response::empty(status_code).boxed())
    }
}

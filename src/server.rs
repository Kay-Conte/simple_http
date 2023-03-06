use std::{
    pin::Pin,
    task::{Context, Poll},
};

use tiny_http::Request;
use tokio_stream::Stream;

/// Stream wrapper provding easy conversion to `ServerStream`
pub struct Server(pub tiny_http::Server);

impl Server {
    pub fn new(addr: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        Ok(Self(tiny_http::Server::http(addr)?))
    }

    pub fn into_stream(self) -> ServerStream {
        ServerStream::from(self)
    }

    fn try_recv(&self) -> Result<Option<Request>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(self.0.try_recv()?)
    }
}

/// Provides async execution over http server
pub struct ServerStream {
    server: Server,
}

impl Stream for ServerStream {
    type Item = tiny_http::Request;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        match self.server.try_recv() {
            Ok(Some(item)) => Poll::Ready(Some(item)),
            Ok(None) => Poll::Pending,
            Err(_) => Poll::Ready(None),
        }
    }
}

impl From<Server> for ServerStream {
    fn from(value: Server) -> Self {
        Self { server: value }
    }
}

use std::{sync::Arc, io::Read};

use crate::{Context, Response};
use tiny_http::ReadWrite;

/// Boxed ReadWrite trait. This is equivalent to the type reaturned by
/// `tiny_http::Request::upgrade()`
type ReadWriteBoxed = Box<dyn ReadWrite + Send>;

/// Websocket poll function. This is called repeatedly in an owned thread in
/// `WebsocketService::run`
type WebsocketFn<T> = fn(&mut Websocket, &Context, T) -> Option<T>;

/// Describes websocket frame type
/// This may not be necessary
pub enum FrameDescriptor {
    Prefixed,
    FixedLength(u64),
    Custom(fn(&mut ReadWriteBoxed) -> Vec<u8>)
}

/// Websocket abstraction responsible for framing and masking data
pub struct Websocket {
    inner: ReadWriteBoxed,
    frame: FrameDescriptor,
}

impl Websocket {
    /// Construct a new Websocket from a `ReadWrite` trait
    pub fn new(inner: ReadWriteBoxed, frame: FrameDescriptor) -> Self {
        Self {
            inner,
            frame,
        }
    }
    
    /// Wait for next frame, blocking
    pub fn next_frame(&mut self) -> Option<Vec<u8>> {
        let mut buf = Vec::new();

        match self.frame {
            FrameDescriptor::FixedLength(len) => {
                match Read::by_ref(&mut self.inner).take(len).read_to_end(&mut buf) {
                    Ok(n) if n >= 1 => {}
                    Ok(_) => return None,
                    Err(_) => return None,
                };
            }
            _ => {
                todo!("Implement Prefixed frame lengths")
            }
        }

        return Some(buf);
    }
}

/// This struct is used to describe how to interact with a Websocket.
pub struct WebsocketDescriptor<T = ()>
where
    T: Sized,
{
    initial_response: Option<Response>,
    initial_data: Option<T>,
    poll_fn: WebsocketFn<T>,
}

impl<T> WebsocketDescriptor<T> {
    /// Constructs a new WebsocketService
    pub fn new(initial_response: Response, initial_data: T, poll_fn: WebsocketFn<T>) -> Self {
        Self {
            initial_response: Some(initial_response),
            initial_data: Some(initial_data),
            poll_fn,
        }
    }
}

/// A trait providing a run method to run WebsocketDescriptors with any generic type on their own thread.
pub trait WebsocketService<T>: Sized
where
    T: Sized + Sync + Send + 'static,
{
    /// Expected to return a Response, this should only be called once.
    fn take_initial_response(&mut self) -> Option<Response>;
    
    /// Expected to return a valid instance of `T` this should only be called once.
    fn take_initial_data(&mut self) -> Option<T>;

    /// Expected to return a pointer to a `WebsocketFn<T>`
    fn poll_fn(&self) -> WebsocketFn<T>;

    /// Repeatedly calls the `WebsocketFn` associated with the descriptor on its own thread. 
    fn run(mut self, context: Arc<Context>, ws: Box<dyn ReadWrite + Send>) {
        let initial_data = self.take_initial_data().expect("Data already taken");
        let poll_fn = self.poll_fn();

        drop(self);

        std::thread::spawn(move || {
            let mut ws = Websocket::new(ws, FrameDescriptor::FixedLength(20));

            let mut ret = initial_data;
            loop {
                ret = if let Some(ret) = poll_fn(&mut ws, context.as_ref(), ret) {
                    ret
                } else {
                    break;
                }
            }
        });
    }
}

impl<T> WebsocketService<T> for WebsocketDescriptor<T>
where
    T: Sized + Sync + Send + 'static,
{
    fn take_initial_response(&mut self) -> Option<Response> {
        self.initial_response.take()
    }

    fn take_initial_data(&mut self) -> Option<T> {
        self.initial_data.take()
    }

    fn poll_fn(&self) -> WebsocketFn<T> {
        self.poll_fn.clone()
    }
}

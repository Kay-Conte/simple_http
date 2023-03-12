use std::{io::Read, sync::Arc};

use tiny_http::ReadWrite;

/// Boxed ReadWrite trait. This is equivalent to the type reaturned by
/// `tiny_http::Request::upgrade()`
type ReadWriteBoxed = Box<dyn ReadWrite + Send>;

/// Websocket poll function. This is called repeatedly in an owned thread in
/// `WebsocketService::run`
type WebsocketFn<Data, State> = fn(&mut Websocket, &Data, State) -> Option<State>;


enum Frame {
    Text {
        payload: String,
        continuation: bool,
        fin: bool
    },
    Binary {
        payload: Vec<u8>,
        continuation: bool,
        fin: bool,
    },
    Ping {
        payload: Option<Vec<u8>>,
    },
    Pong {
        payload: Option<Vec<u8>>,
    },
    Close {
        payload: Option<(u16, String)>
    },
}

/// Websocket abstraction responsible for framing and masking data
pub struct Websocket {
    inner: ReadWriteBoxed,
}

impl Websocket {
    /// Construct a new Websocket from a `ReadWrite` trait
    pub fn new(inner: ReadWriteBoxed) -> Self {
        Self { inner }
    }

    fn read_u8(&mut self) -> Option<u8> {
        self.inner.bytes().next().map(|inner| inner.expect("Error"))
    }

    fn read_u16(&mut self) -> Option<u16> {
        Some(self.inner.bytes().take(2).fold(0_u16, |u, v| u + v.expect("") as u16))
    }

    /// Wait for next frame, blocking
    pub fn next_frame(&mut self) -> Option<Frame> {

        let fin_and_opcode = self.read_u8()?;

        let fin: bool = fin_and_opcode & 0b10000000_u8 != 0;
        let opcode: u8 = fin_and_opcode & 0b00001111_u8;

        let mask_and_payload_len = self.read_u8()?;

        let masked = mask_and_payload_len & 0b10000000_u8 != 0;
        if masked {
            todo!("Handle masked payloads");
        }

        let payload_len = mask_and_payload_len & 0b01111111_u8;
        let payload_len = match payload_len {
           0..=125 => payload_len as usize,
            126 => {

            }

        }

        return Some(buf);
    }
}

pub trait WebsocketService<Data>
where
    Self: Sized,
{
    type State;

    /// Returns the initial state of the `WebsocketService`
    fn initial_state(&self) -> (Self, Self::State);

    fn poll_fn(self) -> (Self, Self::State);

    /// Runs the WebsocketService continuously in a loop
    fn run(self, data: Arc<Data>) {
        let initial_state = self.initial_state();

        std::thread::spawn(move || {});
    }
}

/// Used to allow Boxing of `WebsocketService`, there may be a better method to allow storing of
/// multiple `State` associated types
pub(crate) trait WebsocketServiceExport<Data> {
    /// Expected to run the run method of `WebsocketService`
    fn run(self, data: Arc<Data>);
}

impl<T, Data, State> WebsocketServiceExport<Data> for T
where
    T: Sized + Sync + Send + WebsocketService<Data, State = State>,
{
    #[inline]
    fn run(self, data: Arc<Data>) {
        WebsocketService::run(self, data)
    }
}

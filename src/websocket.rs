use std::{io::Read, sync::{Arc, mpsc}};

use tiny_http::ReadWrite;

/// Boxed ReadWrite trait. This is equivalent to the type reaturned by
/// `tiny_http::Request::upgrade()`
type ReadWriteBoxed = Box<dyn ReadWrite + Send>;

/// Websocket poll function. This is called repeatedly in an owned thread in
/// `WebsocketService::run`

/// A safe unoptimized mask application.
#[inline]
fn apply_mask_fallback(buf: &mut [u8], mask: &[u8]) {
    for (i, byte) in buf.iter_mut().enumerate() {
        *byte ^= mask[i & 3];
    }
}

pub enum Error {
    InvalidFrame,
    FrameNotMasked,
    Io(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Io(value)
    }
}

/// Describes the type of frame of a RawFrame
#[derive(Debug, Clone)]
pub enum FrameKind {
    Text,
    Binary,
    Ping,
    Pong,
    Close,
    Continuation,
}

impl FrameKind {
    /// Constructs a FrameKind given an op_code
    pub fn from_op(op: u8) -> Result<Self, Error> {
        match op {
            0x0 => Ok(Self::Continuation),
            0x1 => Ok(Self::Text),
            0x2 => Ok(Self::Binary),
            0x3..=0x7 => Err(Error::InvalidFrame),
            0x8 => Ok(Self::Close),
            0x9 => Ok(Self::Ping),
            0xA => Ok(Self::Pong),
            _ => Err(Error::InvalidFrame),
        }
    }
}

/// Raw data representation of a frame
#[derive(Debug, Clone)]
pub struct RawFrame {
    payload: Vec<u8>,
    fin: bool,
    kind: FrameKind,
}

impl RawFrame {
    pub fn from_raw(payload: Vec<u8>, fin: bool, op: u8) -> Result<Self, Error> {
        let kind = FrameKind::from_op(op)?;

        Ok(Self { payload, fin, kind })
    }

    pub fn payload(&self) -> &Vec<u8> {
        &self.payload
    }

    pub fn fin(&self) -> bool {
        self.fin
    }

    pub fn kind(&self) -> &FrameKind {
        &self.kind
    }
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

    pub fn read_exact(&mut self, buf: &mut Vec<u8>) -> Result<(), std::io::Error> {
        self.inner.read_exact(buf)
    }

    pub fn read_u8(&mut self) -> u8 {
        let mut buf = vec![0; 1];
        let _ = self.read_exact(&mut buf);

        buf[0]
    }

    pub fn read_u16(&mut self) -> u16 {
        let mut buf = vec![0; 2];
        let _ = self.read_exact(&mut buf);

        buf.iter().fold(0, |m, v| m + *v as u16)
    }


    pub fn read_u32(&mut self) -> u32{
        let mut buf = vec![0; 4];
        let _ = self.read_exact(&mut buf);

        buf.iter().fold(0, |m, v| m + *v as u32)
    }

    pub fn read_u64(&mut self) -> u64 {
        let mut buf = vec![0; 8];
        let _ = self.read_exact(&mut buf);

        buf.iter().fold(0, |m, v| m + *v as u64)
    }

    pub fn send(&mut self, _frame: RawFrame) {
        todo!("Implement send")
    }

    /// Wait for next frame, blocking
    pub fn next_frame(&mut self) -> Result<RawFrame, Error> {
        let fin_and_op = self.read_u8();

        let fin: bool = fin_and_op & 0b10000000_u8 != 0;
        let op: u8 = fin_and_op & 0b00001111_u8;

        let mask_and_payload_len = self.read_u8();

        let masked = mask_and_payload_len & 0b10000000_u8 != 0;
        if !masked {
            return Err(Error::FrameNotMasked);
        }

        let payload_len = mask_and_payload_len & 0b01111111_u8;
        let payload_len = match payload_len {
            0..=125 => payload_len as usize,
            126 => self.read_u16() as usize,
            127 => self.read_u64() as usize,
            _ => unreachable!(),
        };

        let mut masking_key = vec![0; 4];
        self.read_exact(&mut masking_key)?;

        let mut payload = vec![0; payload_len];
        self.read_exact(&mut payload)?;

        apply_mask_fallback(&mut payload, &masking_key);

        Ok(RawFrame::from_raw(payload, fin, op)?)
    }
}

pub type WebsocketFn<Data, State> = fn(&mut Websocket, &Data, State) -> Option<State>;

pub trait WebsocketService<Data>
where
    Self: Sized + 'static,
    Data: Sync + Send + 'static,
{
    type State: Sync + Send + 'static;

    /// Returns the initial state of the `WebsocketService`
    fn initial_state(&self) -> Self::State;

    fn poll_fn(ws: &mut Websocket, data: &Data, state: Self::State) -> Option<Self::State>;

    /// Runs the WebsocketService continuously in a loop
    fn run(self, data: Arc<Data>, mut ws: Websocket) {
        let mut state = self.initial_state();
        let callback = Self::poll_fn;

        std::thread::spawn(move || loop {
            state = if let Some(state) = callback(&mut ws, data.as_ref(), state) {
                state
            } else {
                break;
            }
        });
    }
}

/// Used to allow Boxing of `WebsocketService`, there may be a better method to allow storing of
/// multiple `State` associated types
pub trait WebsocketServiceExport<Data>
where
    Self: Sync + Send,
{
    /// Expected to run the run method of `WebsocketService`
    fn run(self: Box<Self>, data: Arc<Data>, ws: Websocket);
}

impl<T, Data, State> WebsocketServiceExport<Data> for T
where
    T: Sync + Send + WebsocketService<Data, State = State>,
    Data: Sync + Send + 'static,
    State: Sync + Send + 'static,
{
    fn run(self: Box<Self>, data: Arc<Data>, ws: Websocket) {
        WebsocketService::run(*self, data, ws)
    }
}

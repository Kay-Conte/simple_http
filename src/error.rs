#[derive(Debug)]
pub enum Error {
    FailedToInitializeRuntime,
    ServerClosed,
    Io(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;

        match self {
            FailedToInitializeRuntime => write!(f, "Failed to initialize runtime"),
            ServerClosed => write!(f, "Server closed"),
            Io(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

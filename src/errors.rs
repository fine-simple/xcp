use core::result;
use failure::Fail;
use std::io::{Error as IOError, ErrorKind as IOKind};

#[derive(Debug, Fail)]
pub enum XcpError {
    #[fail(display = "Failed to find filename.")]
    UnknownFilename,

    #[fail(display = "Invalid source: {}", msg)]
    InvalidSource { msg: &'static str },

    #[fail(display = "Invalid destination: {}", msg)]
    InvalidDestination { msg: &'static str },
}

pub fn io_err(kind: IOKind, desc: &str) -> Error {
    IOError::new(kind, desc).into()
}

pub use failure::Error;
pub type Result<T> = result::Result<T, Error>;

//! Possible SLIP encoding and decoding errors

use core::fmt;

/// Type alias for handling SLIP errors.
pub type Result<T> = core::result::Result<T, self::Error>;
/// Type alias for handling SLIP buffer errors.
pub type BufferResult<T> = core::result::Result<T, self::BufferError>;

/// Errors encountered by SLIP.
#[derive(Debug)]
pub enum Error {
    // Encoder errors
    /// The encoder does not have enough space to write the SLIP header.
    NoOutputSpaceForHeader,
    /// The encoder does not have enough space to write the final SLIP end byte.
    NoOutputSpaceForEndByte,

    // Decoder errors
    /// The decoder cannot process the SLIP header.
    BadHeaderDecode,
    /// The decoder cannot process the SLIP escape sequence.
    BadEscapeSequenceDecode,

    // Buffer errors
    /// The buffer is full.
    BufferFull
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Error::NoOutputSpaceForHeader => "insufficient space in output buffer for header",
            Error::NoOutputSpaceForEndByte => "insufficient space in output buffer for end byte",
            Error::BadHeaderDecode => "malformed header",
            Error::BadEscapeSequenceDecode => "malformed escape sequence",
            Error::BufferFull => "buffer full",
        })
    }
}

#[derive(Debug)]
pub struct BufferError {
    pub pos: usize,
    pub code: Error,
}

impl From<Error> for BufferError {
    fn from(err: Error) -> Self {
        Self {pos: 0, code: err}
    }
}

use buffer_basic::BasicBuffer;
use error::{BufferError, BufferResult};

use super::*;

/// Decoder buffer allow to decode a stream piece by piece
/// 
pub struct DecoderBuffer<const CAPACITY: usize> {
    buf: BasicBuffer<CAPACITY>,
    is_escaping: bool
}


impl<const CAPACITY: usize> DecoderBuffer<CAPACITY> {

    /// Create a new buffer context for SLIP decoding
    /// 
    pub fn new() -> Self {
        Self {
            buf: BasicBuffer::new(),
            is_escaping: false
        }
    }

    /// Reset the buffer
    /// 
    pub fn reset(&mut self) {
        self.is_escaping = false;
        self.buf.reset();
    }

    /// Get the slice of the buffer
    /// 
    pub fn slice(&self) -> &[u8] {
        self.buf.slice()
    }

    /// Feed the buffer with input data
    /// 
    pub fn feed(&mut self, input: &[u8]) -> BufferResult<(usize, bool)> {
        let mut i = 0;

        while i < input.len() {
            let c = input[i]; // Consume 1 char from input buffer
            i += 1;           // Increment counter
            
            if self.is_escaping {
                self.is_escaping = false;
                match c {
                    ESC_END => {
                        match self.buf.put(END) {
                            Ok(_) => {},
                            Err(code) => {return Err(BufferError{ pos: i, code: code});}
                        }
                    }

                    ESC_ESC => {
                        match self.buf.put(ESC) {
                            Ok(_) => {},
                            Err(code) => {return Err(BufferError{ pos: i, code: code});}
                        }
                    }

                    _ => {return Err(BufferError{ pos: i, code: Error::BadEscapeSequenceDecode});}
                }
            }

            else {
                match c {
                    END => {
                        return Ok((i, true));
                    }

                    ESC => {
                        self.is_escaping = true;
                    }

                    // otherwise, put stuff in buffer
                    _ => {
                        match self.buf.put(c) {
                            Ok(_) => {},
                            Err(code) => {return Err(BufferError{ pos: i, code: code});}
                        }
                    }
                }
            }
        }

        // Input buffer processed, but no packet end yet detected
        Ok((i, false))
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decoder_buffer() {
        let mut buf = DecoderBuffer::<32>::new();
        assert_eq!(buf.slice(), &[]);

        let input = [0x01, 0x02, 0x03, 0x04, 0x05];
        let (bytes_processed, is_end_of_packet) = buf.feed(&input).unwrap();
        assert_eq!(bytes_processed, 5);
        assert_eq!(is_end_of_packet, false);
        assert_eq!(buf.slice(), &input);

        let input = [0x01, 0x02, 0x03, 0x04, 0x05, END];
        let (bytes_processed, is_end_of_packet) = buf.feed(&input).unwrap();
        assert_eq!(bytes_processed, 6);
        assert_eq!(is_end_of_packet, true);
        assert_eq!(buf.slice(), &input[..5]);

        buf.reset();
        assert_eq!(buf.slice(), &[]);
    }
}
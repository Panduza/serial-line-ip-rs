use buffer_basic::BasicBuffer;
use error::{BufferError, BufferResult};

use super::*;


/// Encoder buffer allow to encode a stream piece by piece
/// 
pub struct EncoderBuffer<const CAPACITY: usize> {
    buf: BasicBuffer<CAPACITY>,
}


impl<const CAPACITY: usize> EncoderBuffer<CAPACITY> {

    /// Create a new buffer context for SLIP encoding
    /// 
    pub fn new() -> Self {
        Self {
            buf: BasicBuffer::new(),
        }
    }

    /// Reset the buffer
    /// 
    pub fn reset(&mut self) {
        self.buf.reset();
    }

    /// Get the slice of the buffer
    /// 
    pub fn slice(&self) -> &[u8] {
        self.buf.slice()
    }

    /// Feed the buffer with input data
    /// 
    pub fn feed(&mut self, input: &[u8]) -> BufferResult<usize>{
        let mut i = 0;
        while i < input.len() {
            let c = input[i];
            i += 1;

            match c {
                END => {
                    match self.buf.put(ESC) {
                        Ok(_) => {},
                        Err(code) => {return Err(BufferError{pos: i, code: code})}
                    }

                    match self.buf.put(ESC_END) {
                        Ok(_) => {},
                        Err(code) => {return Err(BufferError{pos: i, code: code})}
                    }
                },

                ESC => {
                    match self.buf.put(ESC) {
                        Ok(_) => {},
                        Err(code) => {return Err(BufferError{pos: i, code: code})}
                    }

                    match self.buf.put(ESC_ESC) {
                        Ok(_) => {},
                        Err(code) => {return Err(BufferError{pos: i, code: code})}
                    }
                },

                other => {
                    match self.buf.put(other) {
                        Ok(_)     => {}
                        Err(code) => {return Err(BufferError{pos: i, code: code})}
                    }
                }
            }
        }

        Ok(i)
    }

    /// Finish the encoding
    /// 
    pub fn finish(&mut self) -> BufferResult<()>{
        Ok(self.buf.put(END)?)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoder_buffer() {
        let mut buf = EncoderBuffer::<32>::new();
        assert_eq!(buf.slice(), &[]);

        buf.feed(&[0x01, 0x02, 0x03]).unwrap();
        assert_eq!(buf.slice(), &[0x01, 0x02, 0x03]);

        buf.feed(&[0xC0]).unwrap();
        assert_eq!(buf.slice(), &[0xDB, 0xDC]);

        buf.feed(&[0x04, 0x05, 0x06]).unwrap();
        assert_eq!(buf.slice(), &[0xDB, 0xDC, 0x04, 0x05, 0x06]);

        buf.finish().unwrap();
        assert_eq!(buf.slice(), &[0xDB, 0xDC, 0x04, 0x05, 0x06, 0xC0]);
    }
}
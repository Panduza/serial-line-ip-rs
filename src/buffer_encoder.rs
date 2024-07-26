use buffer_basic::BasicBuffer;
use error::{BufferError, BufferResult};

use super::*;


/// Encoder buffer allow to encode a stream piece by piece
/// 
pub struct EncoderBuffer<const CAPACITY: usize> {
    buf: BasicBuffer<CAPACITY>,
    header_written: bool,
    skip_header: bool,
}


impl<const CAPACITY: usize> EncoderBuffer<CAPACITY> {

    /// Create a new buffer context for SLIP encoding
    /// 
    pub fn new() -> Self {
        Self {
            buf: BasicBuffer::new(),
            header_written: false,
            skip_header: false,
        }
    }

    /// Reset the buffer
    /// 
    pub fn reset(&mut self) -> BufferResult<()> {
        self.buf.reset();
        Ok(())
    }

    /// Get the slice of the buffer
    /// 
    pub fn slice(&self) -> &[u8] {
        self.buf.slice()
    }

    /// Feed the buffer with input data
    /// 
    pub fn feed(&mut self, input: &[u8]) -> BufferResult<usize>{

        if !self.skip_header {
            if !self.header_written {
                self.buf.put(END)?;
                self.header_written = true;  
            }
        }

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

        if !self.skip_header {
            if !self.header_written {
                self.buf.put(END)?;
                self.header_written = true;  
            }
        }

        Ok(self.buf.put(END)?)
    }

    /// Skip writing the header byte (because it is optional)
    /// 
    pub fn skip_header(mut self) -> Self {
        self.skip_header = true;
        self
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoder_buffer_empty() {
        let mut buf = EncoderBuffer::<32>::new();
        buf.finish().unwrap();
        assert_eq!(buf.slice(), &[END, END]);
    }

    #[test]
    fn test_encoder_buffer() {
        let mut buf = EncoderBuffer::<32>::new();
        
        buf.feed(&[0x01, 0x02, 0x03]).unwrap();
        assert_eq!(buf.slice(), &[END, 0x01, 0x02, 0x03]);

        buf.feed(&[0xC0]).unwrap();
        assert_eq!(buf.slice(), &[END, 0x01, 0x02, 0x03, 0xDB, 0xDC]);

        buf.finish().unwrap();
        assert_eq!(buf.slice(), &[END, 0x01, 0x02, 0x03, 0xDB, 0xDC, END]);
    }

    
    #[test]
    fn test_encoder_buffer_with_header() {
        let mut buf = EncoderBuffer::<32>::new()
            .skip_header();
        assert_eq!(buf.slice(), &[]);

        buf.feed(&[0x01, 0x02, 0x03]).unwrap();
        assert_eq!(buf.slice(), &[0x01, 0x02, 0x03]);

        buf.feed(&[0xC0]).unwrap();
        assert_eq!(buf.slice(), &[0x01, 0x02, 0x03, 0xDB, 0xDC]);

        buf.finish().unwrap();
        assert_eq!(buf.slice(), &[0x01, 0x02, 0x03, 0xDB, 0xDC, END]);

    }
}
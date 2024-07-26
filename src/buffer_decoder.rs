use buffer_basic::BasicBuffer;
use error::{BufferError, BufferResult};

use super::*;

/// Decoder buffer allow to decode a stream piece by piece
/// 
pub struct DecoderBuffer<const CAPACITY: usize> {

    buf: BasicBuffer<CAPACITY>,
    is_escaping: bool,
    header_found: bool,
    search_for_header: bool,
}


impl<const CAPACITY: usize> DecoderBuffer<CAPACITY> {

    /// Create a new buffer context for SLIP decoding
    /// 
    pub fn new() -> Self {
        Self {
            buf: BasicBuffer::new(),
            is_escaping: false,
            header_found: false,
            search_for_header: true,
        }
    }

    /// Reset the buffer
    /// 
    pub fn reset(&mut self) {
        self.is_escaping = false;
        self.header_found = false;
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

        // If we are at the beginning of the buffer, we need to check for the header
        if self.buf.is_first_byte() {
            if self.search_for_header {
                if !self.header_found {
                    if input.len() < 1 {
                        return Err(BufferError{ pos: 0, code: Error::BadHeaderDecode});
                    }

                    if input[0] != END {
                        return Err(BufferError{ pos: 0, code: Error::BadHeaderDecode});
                    }

                    i += 1;

                    self.header_found = true;
                }
            }
        }


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

    /// Skip reading the header byte (because it is optional)
    /// 
    pub fn do_not_search_header(mut self) -> Self {
        self.search_for_header = false;
        self
    }

}



#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]
    fn test_decoder_buffer() {

        let mut buf = DecoderBuffer::<32>::new();
        assert_eq!(buf.slice(), &[]);


        let r = buf.feed(&[0x01, 0x02, 0x03]);
        assert_eq!(r.is_err(), true);

        let (processed, found) = buf.feed(&[END, 0x01, 0x02, 0x03]).unwrap();
        assert_eq!(processed, 4);
        assert_eq!(found, false);

        let (processed, found) = buf.feed(&[0x01, END]).unwrap();
        assert_eq!(processed, 2);
        assert_eq!(found, true);

        assert_eq!(buf.slice(), &[0x01, 0x02, 0x03, 0x01]);
    }

    #[test]
    fn test_decoder_buffer_without_header() {

        // let [END, 0x01, 0x02, 0x03, 0xDB, 0xDC, END];

        let mut buf = DecoderBuffer::<32>::new().do_not_search_header();
        assert_eq!(buf.slice(), &[]);

        let (processed, found) = buf.feed(&[0x01, 0x02, 0x03]).unwrap();
        assert_eq!(processed, 3);
        assert_eq!(found, false);

        let (processed, found) = buf.feed(&[0x01, 0x02, 0x03]).unwrap();
        assert_eq!(processed, 3);
        assert_eq!(found, false);

        let (processed, found) = buf.feed(&[0x01, END]).unwrap();
        assert_eq!(processed, 2);
        assert_eq!(found, true);

        assert_eq!(buf.slice(), &[0x01, 0x02, 0x03, 0x01, 0x02, 0x03, 0x01]);
        buf.reset();
        assert_eq!(buf.slice(), &[]);

    }
}

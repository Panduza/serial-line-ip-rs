use super::*;

/// A basic buffer that can hold a fixed number of bytes.
/// 
pub struct BasicBuffer<const CAPACITY: usize> {
    idx: usize,
    buf: [u8; CAPACITY],
}

impl<const CAPACITY: usize> BasicBuffer<CAPACITY> {
    pub fn new() -> Self {
        Self {idx: 0, buf: [0u8; CAPACITY] }
    }

    pub fn reset(&mut self) {
        self.idx = 0;
    }

    pub fn slice(&self) -> &[u8] {
        &self.buf[..self.idx]
    }

    pub fn put(&mut self, c: u8)-> Result<()> {
        if self.idx >= CAPACITY {
            Err(Error::BufferFull)
        }

        else {
            self.buf[self.idx] = c;
            self.idx+=1;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_basic_buffer() {
        let mut buf = BasicBuffer::<32>::new();
        assert_eq!(buf.slice(), &[]);

        buf.put(0x01).unwrap();
        assert_eq!(buf.slice(), &[0x01]);

        buf.put(0x02).unwrap();
        assert_eq!(buf.slice(), &[0x01, 0x02]);

        buf.put(0x03).unwrap();
        assert_eq!(buf.slice(), &[0x01, 0x02, 0x03]);

        buf.reset();
        assert_eq!(buf.slice(), &[]);

        buf.put(0x04).unwrap();
        assert_eq!(buf.slice(), &[0x04]);
    }

}
use crate::ParseError;

pub(crate) struct Cursor<'a> {
    it: std::iter::Copied<std::slice::Iter<'a, u8>>,
}

impl<'a> Cursor<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            it: data.iter().copied(),
        }
    }

    fn take<const N: usize>(it: &mut impl Iterator<Item = u8>) -> Option<[u8; N]> {
        let mut buf = [0u8; N];
        for b in &mut buf {
            *b = it.next()?;
        }
        Some(buf)
    }

    pub fn u32(&mut self) -> Result<u32, ParseError> {
        Cursor::take::<4>(&mut self.it)
            .map(u32::from_be_bytes)
            .ok_or(ParseError::UnexpectedEof)
    }

    pub fn u16(&mut self) -> Result<u16, ParseError> {
        Cursor::take::<2>(&mut self.it)
            .map(u16::from_be_bytes)
            .ok_or(ParseError::UnexpectedEof)
    }

    pub fn u8(&mut self) -> Result<u8, ParseError> {
        self.it.next().ok_or(ParseError::UnexpectedEof)
    }

    pub fn bytes(&mut self, n: usize) -> Result<Vec<u8>, ParseError> {
        let mut v = Vec::with_capacity(n);
        for _ in 0..n {
            v.push(self.it.next().ok_or(ParseError::UnexpectedEof)?);
        }
        Ok(v)
    }

    pub fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), ParseError> {
        for byte in buf {
            *byte = self.it.next().ok_or(ParseError::UnexpectedEof)?;
        }
        Ok(())
    }
}

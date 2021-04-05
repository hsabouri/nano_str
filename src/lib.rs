use std::error::Error;
use std::fmt;
use std::ops::Deref;

#[derive(Debug)]
pub struct OversizedError(u8);

impl fmt::Display for OversizedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Oversized text, length shoud be < 23 (got: {})", self.0)
    }
}

impl Error for OversizedError {}

#[derive(Debug, Clone, Copy)]
pub struct NanoStr {
    buf: [u8; 23],
    len: u8,
}

impl NanoStr {
    fn __new(s: &str, len: usize) -> Self {
        let s = s.bytes().take(len).collect::<Vec<u8>>();

        let mut buf = [0; 23];

        buf[..len].copy_from_slice(s.as_slice());

        Self {
            buf,
            len: len as u8,
        }
    }

    pub unsafe fn new_unchecked<T: AsRef<str>>(text: T) -> Self {
        let s = text.as_ref();
        let len = if s.len() >= 23 { 23 } else { s.len() };

        Self::__new(s, len)
    }

    pub fn new_truncated<T: AsRef<str>>(text: T) -> Result<Self, OversizedError> {
        let s = text.as_ref();
        let size = s.len();
        let len = s.chars().count();

        if size > 23 && len != size {
            return Err(OversizedError(size as u8));
        } else if len > 23 {
            Ok(Self::__new(s, 23))
        } else {
            Ok(Self::__new(s, len))
        }
    }

    pub fn new<T: AsRef<str>>(text: T) -> Result<Self, OversizedError> {
        let s = text.as_ref();
        let len = s.len();

        if len > 23 {
            return Err(OversizedError(len as u8));
        } else {
            Ok(Self::__new(s, len))
        }
    }
}

impl Deref for NanoStr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        let len = self.len as usize;
        let buf = &self.buf[..len];

        unsafe { std::str::from_utf8_unchecked(buf) }
    }
}

impl fmt::Display for NanoStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &*self)
    }
}

#[cfg(test)]
mod tests {
    use super::NanoStr;

    #[test]
    fn ascii() {
        let s = "Hello World !";
        let ns = NanoStr::new(s).unwrap();

        assert_eq!(&*ns, s);
    }

    #[test]
    fn utf8() {
        let s = "👋🌍";
        let ns = NanoStr::new(s).unwrap();

        assert_eq!(&*ns, s);
    }

    #[test]
    fn new_truncated_ascii() {
        let s = "Hello World !!!!!!!!!!!!!!!!!!!!!!!!!!!!!";
        let ns = NanoStr::new_truncated(s).unwrap();

        assert_ne!(&*ns, s);
    }

    #[test]
    fn new_truncated_utf8() {
        let s = "👋🌍👋👋🌍🌍";
        let ns = NanoStr::new_truncated(s);

        assert_eq!(ns.is_err(), true);
    }
}

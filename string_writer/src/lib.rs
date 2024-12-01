#![no_std]

use core::cmp::min;
use core::fmt;
use core::str::from_utf8;

/// A struct representing a writer that appends formatted data to a byte buffer.
pub struct StringWriter<const N: usize> {
    buf: [u8; N],
    len: usize,
}

impl<const N: usize> StringWriter<N> {
    /// Constructs a new `WriteTo` instance wrapping the provided byte buffer.
    pub fn new() -> Self {
        StringWriter {
            buf: [0u8; N],
            len: N,
        }
    }

    /// Converts the written portion of the buffer into a string slice, if possible.
    pub fn as_str<'a>(&'a self) -> Option<&'a str> {
        if self.len <= self.buf.len() {
            from_utf8(self.buf.as_ref()).ok()
        } else {
            None
        }
    }

    /// Get the number of bytes written to buffer, unless there where errors.
    pub fn len(&self) -> Option<usize> {
        if self.len <= self.buf.len() {
            Some(self.len)
        } else {
            None
        }
    }

    /// Returns true if self has a length of zero bytes, unless there where errors.
    pub fn is_empty(&self) -> Option<bool> {
        if self.len <= self.buf.len() {
            Some(self.len == 0)
        } else {
            None
        }
    }
}

impl<const N: usize> fmt::Write for StringWriter<N> {
    /// Writes a string slice into the buffer, updating the length accordingly.
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if self.len > self.buf.len() {
            return Err(fmt::Error);
        }

        let rem = &mut self.buf[self.len..];
        let raw_s = s.as_bytes();
        let num = min(raw_s.len(), rem.len());

        rem[..num].copy_from_slice(&raw_s[..num]);
        self.len += raw_s.len();

        if num < raw_s.len() {
            Err(fmt::Error)
        } else {
            Ok(())
        }
    }
}

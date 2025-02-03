use std::io::{self, BufRead, ErrorKind, Read};
use std::string::{FromUtf8Error, IntoChars};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Utf8(FromUtf8Error),
}

/// Not the best optimization.
/// Better use one buffer, instead of creating a new one each time.
pub struct ReadCharsChunksIter<R: BufRead> {
    reader: R,
    read_until: u8,
    chars: IntoChars,
}

impl<R: Read + BufRead> Iterator for ReadCharsChunksIter<R> {
    type Item = Result<char, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ch) = self.chars.next() {
                return Some(Ok(ch));
            }

            let mut buffer = Vec::<u8>::new();
            return match self.reader.read_until(self.read_until, &mut buffer) {
                Ok(0) => None,
                Ok(_) => {
                    let text = match String::from_utf8(buffer) {
                        Ok(t) => t,
                        Err(e) => return Some(Err(Error::Utf8(e))),
                    };
                    self.chars = text.into_chars();

                    continue;
                }
                Err(e) if e.kind() == ErrorKind::UnexpectedEof => None,
                Err(e) => Some(Err(Error::Io(e))),
            };
        }
    }
}

pub trait ReadCharsChunks {
    type Output;

    fn chars_chunks(self, read_until: u8) -> Self::Output;
}

impl<R: Read + BufRead> ReadCharsChunks for R {
    type Output = ReadCharsChunksIter<R>;

    fn chars_chunks(self, read_until: u8) -> Self::Output {
        ReadCharsChunksIter {
            reader: self,
            read_until,
            chars: String::new().into_chars(),
        }
    }
}

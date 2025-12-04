use ::std::{
    io::{self, BufRead, ErrorKind, Read},
    string::FromUtf8Error,
    sync::LazyLock,
};
use regex::Regex;

static SKIP_PARAMS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"strong\s?=\s?["″][^"″]*["″]"#).unwrap()); // skips `strong="val"`

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Utf8(FromUtf8Error),
}

/// Not the best optimization.
/// Better use one buffer, instead of creating a new one each time.
pub struct ReadChunksIter<R: BufRead> {
    reader: R,
    read_until: u8,
}

impl<R: Read + BufRead> Iterator for ReadChunksIter<R> {
    type Item = Result<String, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = Vec::<u8>::new();
        match self.reader.read_until(self.read_until, &mut buffer) {
            Ok(0) => None,
            Ok(_) => Some(match String::from_utf8(buffer) {
                Ok(t) => Ok(SKIP_PARAMS.replace_all(&t, "").into_owned()),
                Err(e) => Err(Error::Utf8(e)),
            }),
            Err(e) if e.kind() == ErrorKind::UnexpectedEof => None,
            Err(e) => Some(Err(Error::Io(e))),
        }
    }
}

pub trait ReadChunks: Sized {
    type Output;

    fn chunks(self, read_until: u8) -> Self::Output;
}

impl<R: Read + BufRead> ReadChunks for R {
    type Output = ReadChunksIter<R>;

    fn chunks(self, read_until: u8) -> Self::Output {
        ReadChunksIter {
            reader: self,
            read_until,
        }
    }
}

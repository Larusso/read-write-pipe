use std::io::{ErrorKind, Read, Result, Write};

/// A trait for objects implementing [`Write`], to write all content from a [`Read`] object.
///
/// This trait adds one method to the writers implementing it.
/// * `write_reader`
/// This method allows to read a whole reader object into the writer.
/// There is no garantee about the state of both reader and writer in case of an error.
/// ```
/// use std::io;
/// use read_write_pipe::*;
/// let _ = io::stdout().write_reader(io::stdin()).unwrap();
/// ```
///
/// [`Read`]:         https://doc.rust-lang.org/std/io/trait.Read.html
/// [`Write`]:         https://doc.rust-lang.org/std/io/trait.Read.html
pub trait ReadWritePipe: Write {
    fn write_reader<R: Read>(&mut self, mut r: R) -> Result<usize> {
        let ret;
        let mut bytes_written = 0;
        let mut buffer = [0; 1024];

        loop {
            match r.read(&mut buffer[..]) {
                Ok(0) => {
                    ret = Ok(bytes_written);
                    break;
                }
                Ok(n) => {
                    self.write_all(&buffer[0..n])?;
                    bytes_written += n;
                }
                Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
                Err(e) => {
                    ret = Err(e);
                    break;
                }
            }
        }
        ret
    }
}

impl<W: Write> ReadWritePipe for W {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_all_bytes_from_reader_into_writer() {
        let reader = b"a small little text";
        let mut writer: Vec<u8> = Vec::new();
        let size = writer.write_reader(reader as &[u8]).unwrap();
        assert_eq!(size, 19);
        let mut expected_result: Vec<u8> = Vec::new();
        expected_result.write_all(b"a small little text").unwrap();
        assert_eq!(writer, expected_result);
    }
}

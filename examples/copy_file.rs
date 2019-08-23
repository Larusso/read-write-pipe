use read_write_pipe::*;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::Write;

fn prepare_input() -> io::Result<()> {
    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("a.txt")?;
    f.write_all(b"some content")
}

fn main() -> io::Result<()> {
    prepare_input()?;
    let input = File::open("a.txt")?;
    let mut output = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("b.txt")?;
    let _ = output.write_reader(input)?;
    Ok(())
}

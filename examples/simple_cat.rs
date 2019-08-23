use read_write_pipe::*;
use std::io;

fn main() -> io::Result<()> {
    let _ = io::stdout().write_reader(io::stdin())?;
    Ok(())
}

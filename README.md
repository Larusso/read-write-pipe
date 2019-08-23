read-write-pipe
===============

A trait for objects implementing [`Write`], to write all content from a [`Read`] object.

[![Build Status](https://travis-ci.org/Larusso/read-write-pipe.svg?branch=master)](https://travis-ci.org/Larusso/read-write-pipe)
[![Crates.io](https://img.shields.io/crates/v/read-write-pipe.svg)](https://crates.io/crates/read-write-pipe)

`read-write-pipe` is a library for Rust that contains a single utility `Trait` to write a generic [`Read`] object into a writer.
The `Trait` in this library are not production ready. It serves as a utility for quick and dirty copies of read objects into files or `stdout`.

Example
-------

```rust
  use read_write_pipe::*;
  use std::fs::{File, OpenOptions};
  use std::io;
  use std::io::Write;

  fn main() -> io::Result<()> {
      let input = File::open("a.txt")?;
      let mut output = OpenOptions::new()
          .read(true)
          .write(true)
          .create(true)
          .open("b.txt")?;
      let _ = output.write_reader(input)?;
      Ok(())
  }
```

Documentation: https://docs.rs/read-write-pipe

License
=======

[Apache License 2.0](LICENSE)

[`Read`]:         https://doc.rust-lang.org/std/io/trait.Read.html
[`Write`]:         https://doc.rust-lang.org/std/io/trait.Read.html
[`IntoIterator`]: https://doc.rust-lang.org/std/iter/trait.IntoIterator.html  

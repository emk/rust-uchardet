[![Build Status](https://travis-ci.org/emk/rust-uchardet.svg?branch=master)](https://travis-ci.org/emk/rust-uchardet)

Attempts to detect the character encoding of raw text using the `uchardet`
library.

To add it to your project, add the following lines to your `Cargo.toml`
file:

```
[dependencies.uchardet]
git = "git://github.com/emk/rust-uchardet"
```

To run it:

```rust
// At the top of the file.
extern crate uchardet;
use uchardet::EncodingDetector;

// Inside a function.
assert_eq!(Some("UTF-8".to_string()),
           detect_encoding_name("français".as_bytes()).unwrap());
```

[API documentation is available][apidoc].

_Are you looking for a Rust wrapper for cld2 for detecting languages?  I'm
currently working on one and hope to publish it shortly._

[apidoc]: http://www.rust-ci.org/emk/rust-uchardet/doc/uchardet/

### Getting uchardet (usually optional)

If you wish, you may install `uchardet` using your system package manager.
For example, under Ubuntu, you can run:

```sh
sudo apt-get install libuchardet-dev
```

If you skip this step, Cargo will attempt to compile `uchardet` from the
bundled source code instead.  This will probably only work on Linux
machines with CMake involved, but pull requests to improve this are
welcomed eagerly.

### License

New code in the `rust-uchardet` library is released into the public domain,
as described in the `UNLICENSE` file.  However, several pre-existing pieces
have their own licenses:

- The [`uchardet` C++ library][cxx] in `uchardet-sys/uchardet` is
  distributed under the Mozilla Public License 1.1.
- The file `uchardet-sys/src/build.rs` contains several short snippets of
  code based on Alex Crichton's [git2-rs][] library, which is described as
  being licenses "under the terms of both the MIT license and the Apache
  License (Version 2.0), with portions covered by various BSD-like
  licenses."  However, this file is only run at build time, not linked into
  the resulting executable.

[cxx]: https://code.google.com/p/uchardet/
[git2-rs]: https://github.com/alexcrichton/git2-rs/

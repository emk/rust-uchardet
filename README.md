[![Latest version](https://img.shields.io/crates/v/uchardet.svg)](https://crates.io/crates/uchardet) [![License](https://img.shields.io/crates/l/uchardet.svg)](http://unlicense.org/) [![Build Status](https://travis-ci.org/emk/rust-uchardet.svg?branch=master)](https://travis-ci.org/emk/rust-uchardet) [![Build status](https://ci.appveyor.com/api/projects/status/5qas95gi8935jtn8?svg=true)](https://ci.appveyor.com/project/emk/rust-uchardet) [![Documentation](https://img.shields.io/badge/documentation-docs.rs-yellow.svg)](https://docs.rs/uchardet/)

_**Deprecated in favor of [chardet][],** which is pure Rust. If you have
use-case for this code, please feel free to open an issue. Simple PRs will
still be read, and possibly accepted._

[chardet]: https://crates.io/crates/chardet

Attempts to detect the character encoding of raw text using the `uchardet`
library.

Example:

```rust
// At the top of the file.
extern crate uchardet;
use uchardet::detect_encoding_name;

// Inside a function.
assert_eq!("UTF-8",
           detect_encoding_name(""©français"".as_bytes()).unwrap());
```

If you also would also like to detect the language used in the decoded
text, see [rust-cld2](https://github.com/emk/rust-cld2).

[cld2]: https://github.com/emk/rust-cld2

### Getting uchardet (usually optional)

If you wish, you may install `uchardet` using your system package manager.
For example, under Ubuntu, you can run:

```sh
sudo apt-get install libuchardet-dev
```

If you skip this step, Cargo will attempt to compile `uchardet` from the
bundled source code instead.  This should work if you have an appropriate
`g++` (or MSVC) compiler installed, as well as `cmake`.  We test this build
on Linux, OS X and Windows (both MinGW and MSVC) using Travis CI.

### Contributing

As always, pull requests are welcome!  Please keep any patches as simple as
possible and include unit tests; that makes it much easier for me to merge
them.

If you want to get the C/C++ code building on another platform, please see
`uchardef-sys/build.rb` and [this build script guide][build-script].
You'll probably need to adjust some compiler options.  Please don't
hesitate to ask questions; I'd love for this library to support more
platforms.

[build-script]: http://doc.crates.io/build-script.html

In your first commit message, please include the following statement:

> I dedicate any and all copyright interest in my contributions to this
> project to the public domain. I make this dedication for the benefit of
> the public at large and to the detriment of my heirs and successors. I
> intend this dedication to be an overt act of relinquishment in perpetuity
> of all present and future rights to this software under copyright law.

This allows us to keep the library legally unencumbered, and free for
everyone to use.

Contributors include:

- Boris-Chengbiao Zhou.  Support for newer upstream `uchardet` libraries
  and Microsoft Windows, plus the error-chain conversion.
- Wesley Moore. Fixes for Rust beta 2 and OS X.

Thank you very much for your contributions!

### License

New code in the `rust-uchardet` library is released into the public domain,
as described in the `UNLICENSE` file.  However, several pre-existing pieces
have their own licenses:

- The [`uchardet` C++ library][cxx] include in `uchardet-sys/uchardet` via
  a git submodule is distributed under the Mozilla Public License 1.1.

[cxx]: https://code.google.com/p/uchardet/
[git2-rs]: https://github.com/alexcrichton/git2-rs/

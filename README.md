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
use uchardet::detect_encoding_name;

// Inside a function.
assert_eq!(Some("UTF-8".to_string()),
           detect_encoding_name("français".as_bytes()).unwrap());
```

[API documentation is available][apidoc].

If you also would also like to detect the language used in the decoded
text, see [rust-cld2](https://github.com/emk/rust-cld2).

[apidoc]: http://www.rust-ci.org/emk/rust-uchardet/doc/uchardet/
[cld2]: https://github.com/emk/rust-cld2

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

### Contributing

As always, pull requests are welcome!  Please keep any patches as simple as
possible and include unit tests; that makes it much easier for me to merge
them.

If you want to get the C/C++ code building on another platform, please see
`uchardef-sys/build.rb`, [this build script guide][build-script], and the
[`build.rs` file from `git2-rs`][git2].  You'll probably need to adjust
some compiler options.  Please don't hesitate to ask questions; I'd love
for this library to be cross platform.

[build-script]: http://doc.crates.io/build-script.html
[git2]: https://github.com/alexcrichton/git2-rs/blob/master/libgit2-sys/build.rs

In your first commit message, please include the following statement:

> I dedicate any and all copyright interest in my contributions to this
project to the public domain. I make this dedication for the benefit of the
public at large and to the detriment of my heirs and successors. I intend
this dedication to be an overt act of relinquishment in perpetuity of all
present and future rights to this software under copyright law.

This allows us to keep the library legally unencumbered, and free for
everyone to use.

### License

New code in the `rust-uchardet` library is released into the public domain,
as described in the `UNLICENSE` file.  However, several pre-existing pieces
have their own licenses:

- The [`uchardet` C++ library][cxx] include in `uchardet-sys/uchardet` via
  a git submodule is distributed under the Mozilla Public License 1.1.
- The file `uchardet-sys/src/build.rs` contains several short snippets of
  code based on Alex Crichton's [git2-rs][] library, which is described as
  being licensed "under the terms of both the MIT license and the Apache
  License (Version 2.0), with portions covered by various BSD-like
  licenses."  However, this file is only run at build time, not linked into
  the resulting executable.

[cxx]: https://code.google.com/p/uchardet/
[git2-rs]: https://github.com/alexcrichton/git2-rs/

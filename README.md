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
           EncodingDetector::detect("français".as_bytes()).unwrap());
```

[API documentation is available][apidoc].

[apidoc]: http://www.rust-ci.org/emk/rust-uchardet/doc/uchardet/

### Getting uchardet

For now, you need to install `uchardet` using your system package manager.
For example, under Ubuntu, you can run:

```sh
sudo apt-get install libuchardet-dev
```

As of right now, we do not support building a local copy of uchardet if
none is available on the system.  But pull requests to do so are eagerly
welcome.

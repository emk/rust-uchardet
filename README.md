Attempts to detect the character encoding of raw text using the `uchardet`
library.

To run it:

```rust
assert_eq!(Some("UTF-8".to_string()),
           EncodingDetector::detect("français".as_bytes()).unwrap());
```

### Getting uchardet

For now, you need to install `uchardet` using your system package manager.
For example, under Ubuntu, you can run:

```sh
sudo apt-get install libuchardet-dev
```

As of right now, we do not support building a local copy of uchardet if
none is available on the system.  But pull requests to do so are eagerly
welcome.

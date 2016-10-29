//! A wrapper around the uchardet library. Detects character encodings.
//!
//! Note that the underlying implemention is written in C and C++, and I'm
//! not aware of any security audits which have been performed against it.
//!
//! ```
//! use uchardet::detect_encoding_name;
//!
//! assert_eq!("WINDOWS-1252",
//!            detect_encoding_name(&[0x46, 0x93, 0x72, 0x61, 0x6e, 0xe7, 0x6f,
//!                0x69, 0x73, 0xe9, 0x94]).unwrap());
//! ```
//!
//! For more information, see [this project on
//! GitHub](https://github.com/emk/rust-uchardet).

// Increase the compiler's recursion limit for the `error_chain` crate.
#![recursion_limit = "1024"]
#![deny(missing_docs)]

#[macro_use]
extern crate error_chain;
extern crate libc;
extern crate uchardet_sys as ffi;

use libc::size_t;
use std::ffi::CStr;
use std::str::from_utf8;

pub use errors::*;

#[allow(missing_docs)]
mod errors {
    error_chain! {
        errors {
            UnrecognizableCharset {
                description("unrecognizable charset")
                display("uchardet was unable to recognize a charset")
            }
            OutOfMemory {
                description("out of memory error")
                display("uchardet ran out of memory")
            }
            Other(int: i32) {
                description("unknown error")
                display("uchardet returned unknown error {}", int)
            }
        }
    }

    impl ErrorKind {
        pub fn from_nsresult(nsresult: ::ffi::nsresult) -> ErrorKind {
            assert!(nsresult != 0);
            match nsresult {
                1 => ErrorKind::OutOfMemory,
                int => ErrorKind::Other(int),
            }
        }

    }
}

/// Detects the encoding of text using the uchardet library.
///
/// EXPERIMENTAL: This may be replaced by a better API soon.
struct EncodingDetector {
    ptr: ffi::uchardet_t
}

/// Return the name of the charset used in `data` or an error if uchardet
/// was unable to detect a charset.
///
/// ```
/// use uchardet::detect_encoding_name;
///
/// assert_eq!("ASCII",
///            detect_encoding_name("ascii".as_bytes()).unwrap());
/// assert_eq!("UTF-8",
///            detect_encoding_name("©français".as_bytes()).unwrap());
/// assert_eq!("WINDOWS-1252",
///            detect_encoding_name(&[0x46, 0x93, 0x72, 0x61, 0x6e, 0xe7, 0x6f,
///                0x69, 0x73, 0xe9, 0x94]).unwrap());
/// ```
pub fn detect_encoding_name(data: &[u8]) -> Result<String> {
    let mut detector = EncodingDetector::new();
    try!(detector.handle_data(data));
    detector.data_end();
    detector.charset()
}

impl EncodingDetector {
    /// Create a new EncodingDetector.
    fn new() -> EncodingDetector {
        let ptr = unsafe { ffi::uchardet_new() };
        assert!(!ptr.is_null());
        EncodingDetector{ptr: ptr}
    }

    /// Pass a chunk of raw bytes to the detector. This is a no-op if a
    /// charset has been detected.
    fn handle_data(&mut self, data: &[u8]) -> Result<()> {
        let nsresult = unsafe {
            ffi::uchardet_handle_data(self.ptr, data.as_ptr() as *const i8,
                                      data.len() as size_t)
        };
        match nsresult {
            0 => Ok(()),
            int => {
                Err(ErrorKind::from_nsresult(int).into())
            }
        }
    }

    /// Notify the detector that we're done calling `handle_data`, and that
    /// we want it to make a guess as to our encoding. This is a no-op if
    /// no data has been passed yet, or if an encoding has been detected
    /// for certain. From reading the code, it appears that you can safely
    /// call `handle_data` after calling this, but I'm not certain.
    fn data_end(&mut self) {
        unsafe { ffi::uchardet_data_end(self.ptr); }
    }

    /// Get the decoder's current best guess as to the encoding. May return
    /// an error if uchardet was unable to detect an encoding.
    fn charset(&self) -> Result<String> {
        unsafe {
            let internal_str = ffi::uchardet_get_charset(self.ptr);
            assert!(!internal_str.is_null());
            let bytes = CStr::from_ptr(internal_str).to_bytes();
            let charset = from_utf8(bytes);
            match charset {
                Err(_) =>
                    panic!("uchardet_get_charset returned a charset name \
                            containing invalid characters"),
                Ok("") => Err(ErrorKind::UnrecognizableCharset.into()),
                Ok(encoding) => Ok(encoding.to_string())
            }
        }
    }
}

impl Drop for EncodingDetector {
    fn drop(&mut self) {
        unsafe { ffi::uchardet_delete(self.ptr) };
    }
}

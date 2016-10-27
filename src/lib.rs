//! A wrapper around the uchardet library. Detects character encodings.
//!
//! Note that the underlying implemention is written in C and C++, and I'm
//! not aware of any security audits which have been performed against it.
//!
//! ```
//! use uchardet::detect_encoding_name;
//!
//! assert_eq!("ISO-8859-1".to_string(),
//!            detect_encoding_name(&[0x46, 0x72, 0x61, 0x6e, 0xe7, 0x6f,
//!                0x69, 0x73, 0xe9]).unwrap());
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

use errors::*;

mod errors {
    error_chain! {
        errors {
            UnrecognizableCharset {
                description("unrecognizable charset")
                display("uchardet was unable to recognize a charset")
            }
            DataHandlingError {
                description("data handling error")
                display("uchardet failed to handle the supplied data (out of memory?)")
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
/// assert_eq!("ASCII".to_string(),
///            detect_encoding_name("ascii".as_bytes()).unwrap());
/// assert_eq!("UTF-8".to_string(),
///            detect_encoding_name("©français".as_bytes()).unwrap());
/// assert_eq!("ISO-8859-1".to_string(),
///            detect_encoding_name(&[0x46, 0x72, 0x61, 0x6e, 0xe7, 0x6f,
///                0x69, 0x73, 0xe9]).unwrap());
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
        let result = unsafe {
            ffi::uchardet_handle_data(self.ptr, data.as_ptr() as *const i8,
                                      data.len() as size_t)
        };
        match result {
            0 => Ok(()),
            _ => {
                Err(ErrorKind::DataHandlingError.into())
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

    /// Reset the detector's internal state.
    // fn reset(&mut self) {
    //    unsafe { ffi::uchardet_reset(self.ptr); }
    // }

    /// Get the decoder's current best guess as to the encoding. May return
    /// an error if uchardet was unable to detect an encoding
    fn charset(&self) -> Result<String> {
        unsafe {
            let internal_str = ffi::uchardet_get_charset(self.ptr);
            assert!(!internal_str.is_null());
            let bytes = CStr::from_ptr(internal_str).to_bytes();
            let charset = from_utf8(bytes);
            match charset {
                Err(_) =>
                    panic!("uchardet_get_charset returned invalid value"),
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

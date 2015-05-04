//! A wrapper around the uchardet library.  Detects character encodings.
//!
//! Note that the underlying implemention is written in C and C++, and I'm
//! not aware of any security audits which have been performed against it.
//!
//! ```
//! use uchardet::detect_encoding_name;
//!
//! assert_eq!(Some("windows-1252".to_string()),
//!            detect_encoding_name(&[0x66u8, 0x72, 0x61, 0x6e, 0xe7,
//!                                 0x61, 0x69, 0x73]).unwrap());
//! ```
//!
//! For more information, see [this project on
//! GitHub](https://github.com/emk/rust-uchardet).

#![deny(missing_docs)]

extern crate libc;
extern crate uchardet_sys as ffi;

use libc::size_t;
use std::error::Error;
use std::fmt;
use std::result::Result;
use std::ffi::CStr;
use std::str::from_utf8;

/// An error occurred while trying to detect the character encoding.
#[derive(Debug)]
pub struct EncodingDetectorError {
    message: String
}

impl Error for EncodingDetectorError {
    fn description(&self) -> &str { "encoding detector error" }
    fn cause(&self) -> Option<&Error> { None }
}

impl fmt::Display for EncodingDetectorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.message)
    }
}

/// Either a return value, or an encoding detection error.
pub type EncodingDetectorResult<T> = Result<T, EncodingDetectorError>;

/// Detects the encoding of text using the uchardet library.
///
/// EXPERIMENTAL: This may be replaced by a better API soon.
pub struct EncodingDetector {
    ptr: ffi::uchardet_t
}

/// Return the name of the charset used in `data`, or `None` if the
/// charset is ASCII or if the encoding can't be detected.  This is
/// the value returned by the underlying `uchardet` library, with
/// the empty string mapped to `None`.
///
/// ```
/// use uchardet::detect_encoding_name;
///
/// assert_eq!(None, detect_encoding_name("ascii".as_bytes()).unwrap());
/// assert_eq!(Some("UTF-8".to_string()),
///            detect_encoding_name("français".as_bytes()).unwrap());
/// assert_eq!(Some("windows-1252".to_string()),
///            detect_encoding_name(&[0x66u8, 0x72, 0x61, 0x6e, 0xe7,
///                                 0x61, 0x69, 0x73]).unwrap());
/// ```
pub fn detect_encoding_name(data: &[u8]) ->
    EncodingDetectorResult<Option<String>>
{
    let mut detector = EncodingDetector::new();
    try!(detector.handle_data(data));
    detector.data_end();
    Ok(detector.charset())
}

impl EncodingDetector {
    /// Deprecated API for detecting encoding names.
    /// Use uchardet::detect_encoding_name instead
    pub fn detect(data: &[u8]) -> EncodingDetectorResult<Option<String>> {
        detect_encoding_name(data)
    }

    /// Create a new EncodingDetector.
    pub fn new() -> EncodingDetector {
        let ptr = unsafe { ffi::uchardet_new() };
        assert!(!ptr.is_null());
        EncodingDetector{ptr: ptr}
    }

    /// Pass a chunk of raw bytes to the detector.  This is a no-op if a
    /// charset has been detected.
    pub fn handle_data(&mut self, data: &[u8]) -> EncodingDetectorResult<()> {
        let result = unsafe {
            ffi::uchardet_handle_data(self.ptr, data.as_ptr() as *const i8,
                                      data.len() as size_t)
        };
        match result {
            0 => Ok(()),
            _ => {
                let msg =  "Error handling data".to_string();
                Err(EncodingDetectorError{message: msg})
            }
        }
    }

    /// Notify the detector that we're done calling `handle_data`, and that
    /// we want it to make a guess as to our encoding.  This is a no-op if
    /// no data has been passed yet, or if an encoding has been detected
    /// for certain.  From reading the code, it appears that you can safely
    /// call `handle_data` after calling this, but I'm not certain.
    pub fn data_end(&mut self) {
        unsafe { ffi::uchardet_data_end(self.ptr); }
    }

    /// Reset the detector's internal state.
    pub fn reset(&mut self) {
        unsafe { ffi::uchardet_reset(self.ptr); }
    }

    /// Get the decoder's current best guess as to the encoding.  Returns
    /// `None` on error, or if the data appears to be ASCII.
    pub fn charset(&self) -> Option<String> {
        unsafe {
            let internal_str = ffi::uchardet_get_charset(self.ptr);
            assert!(!internal_str.is_null());
            let bytes = CStr::from_ptr(internal_str).to_bytes();
            let charset = from_utf8(bytes);
            match charset {
                Err(_) =>
                    panic!("uchardet_get_charset returned invalid value"),
                Ok("") => None,
                Ok(encoding) => Some(encoding.to_string())
            }
        }
    }
}

impl Drop for EncodingDetector {
    fn drop(&mut self) {
        unsafe { ffi::uchardet_delete(self.ptr) };
    }
}

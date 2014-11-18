//! A wrapper around the uchardet library.  Detects character encodings.
//! Note that the underlying implemention is written in C and C++, and I'm
//! not aware of any security audits which have been performed against it.

#![license = "Public domain (Unlicense)"]
#![unstable]
#![deny(missing_docs)]

extern crate libc;

use std::error::Error;
use std::result::Result;
use std::c_str::CString;
use libc::{c_char, c_int, c_void, size_t};

#[allow(non_camel_case_types)]
type uchardet_t = *mut c_void;

#[link(name = "uchardet")]
extern {
    fn uchardet_new() -> uchardet_t;
    fn uchardet_delete(ud: uchardet_t);
    fn uchardet_handle_data(ud: uchardet_t, data: *const c_char, len: size_t) ->
        c_int;
    fn uchardet_data_end(ud: uchardet_t);
    fn uchardet_reset(ud: uchardet_t);
    fn uchardet_get_charset(ud: uchardet_t) -> *const c_char;
}

/// An error occurred while trying to detect the character encoding.
#[stable]
#[deriving(Show)]
pub struct EncodingDetectionError {
    message: String
}

#[stable]
impl Error for EncodingDetectionError {
    fn description(&self) -> &str { "encoding detection error" }
    fn detail(&self) -> Option<String> { Some(self.message.clone()) }
    fn cause(&self) -> Option<&Error> { None }
}

/// Either a return value, or an encoding detection error.
#[stable]
pub type EncodingDetectionResult<T> = Result<T, EncodingDetectionError>;

/// Detects the encoding of text using the uchardet library.
#[stable]
pub struct EncodingDetector {
    ptr: uchardet_t
}

impl EncodingDetector {
    /// Return the name of the charset used in `data`, or `None` if the
    /// charset is ASCII or if the encoding can't be detected. (Yes, this
    /// API is weird, and we may offer a better alternative in the future.)
    ///
    /// ```
    /// use uchardet::EncodingDetector;
    ///
    /// assert_eq!(None, EncodingDetector::detect("ascii".as_bytes()).unwrap());
    /// assert_eq!(Some("UTF-8".to_string()),
    ///            EncodingDetector::detect("français".as_bytes()).unwrap());
    /// assert_eq!(Some("windows-1252".to_string()),
    ///            EncodingDetector::detect(&[0x66u8, 0x72, 0x61, 0x6e, 0xe7,
    ///                                       0x61, 0x69, 0x73]).unwrap());
    /// ```
    #[stable]
    pub fn detect(data: &[u8]) -> EncodingDetectionResult<Option<String>> {
        let mut detector = EncodingDetector::new();
        try!(detector.handle_data(data));
        detector.data_end();
        Ok(detector.charset())
    }

    /// Create a new EncodingDetector.
    #[stable]
    pub fn new() -> EncodingDetector {
        let ptr = unsafe { uchardet_new() };
        assert!(ptr.is_not_null());
        EncodingDetector{ptr: ptr}
    }

    /// Pass a chunk of raw bytes to the detector.  This is a no-op if a
    /// charset has been detected.
    #[unstable = "The underlying API is poorly documented."]
    pub fn handle_data(&mut self, data: &[u8]) -> EncodingDetectionResult<()> {
        let result = unsafe {
            uchardet_handle_data(self.ptr, data.as_ptr() as *const i8,
                                 data.len() as size_t)
        };
        match result {
            0 => Ok(()),
            _ => {
                let msg =  "Error handling data".to_string();
                Err(EncodingDetectionError{message: msg})
            }
        }
    }

    /// Notify the detector that we're done calling `handle_data`, and that
    /// we want it to make a guess as to our encoding.  This is a no-op if
    /// no data has been passed yet, or if an encoding has been detected
    /// for certain.  From reading the code, it appears that you can safely
    /// call `handle_data` after calling this, but I'm not certain.
    #[unstable = "The underlying API is poorly documented."]
    pub fn data_end(&mut self) {
        unsafe { uchardet_data_end(self.ptr); }
    }

    /// Reset the detector's internal state.
    #[unstable = "The underlying API is poorly documented."]
    pub fn reset(&mut self) {
        unsafe { uchardet_reset(self.ptr); }
    }

    /// Get the decoder's current best guess as to the encoding.  Returns
    /// `None` on error, or if the data appears to be ASCII.
    #[unstable = "The underlying API is poorly documented."]
    pub fn charset(&self) -> Option<String> {
        unsafe {
            let internal_str = uchardet_get_charset(self.ptr);
            assert!(internal_str.is_not_null());
            let c_str = CString::new(internal_str, false);
            match c_str.as_str() {
                None =>
                    panic!("uchardet_get_charset returned invalid value"),
                Some("") => None,
                Some(encoding) => Some(encoding.to_string())
            }
        }
    }
}

#[stable]
impl Drop for EncodingDetector {
    fn drop(&mut self) {
        unsafe { uchardet_delete(self.ptr) };
    }
}

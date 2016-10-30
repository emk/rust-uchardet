//! Support for detecting `encoding_rs` encodings, with or without
//! `uchardet`.
//!
//! Note that we do _not_ promise to always return the same output for any
//! given input, even in stable versions of this API.  Our contract is that
//! we try to get the right answer.

#![warn(missing_docs)]

extern crate encoding_rs;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
#[cfg(feature="uchardet")]
extern crate uchardet;

use encoding_rs::Encoding;
use std::borrow::Cow;

pub use errors::*;
#[cfg(feature="uchardet")]
pub use uchardet_detector::*;

mod errors;
#[cfg(feature="uchardet")]
mod uchardet_detector;

/// How accurate was our decoding?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Accuracy {
    /// We didn't need to replace any malformed sequences with replacement
    /// characters.
    Lossless,
    /// We needed to replace malformed sequences with replacement
    /// characters.
    Lossy,
}

impl Accuracy {
    /// Build an `Accuracy` from an `Encoding` decode result.
    fn from_decode_result(lossy: bool) -> Accuracy {
        if lossy {
            Accuracy::Lossy
        } else {
            Accuracy::Lossless
        }
    }
}

/// This trait is implemented by our various encoding detectors.
pub trait EncodingDetector {
    /// Detect the encoding of `data`, and return a matching
    /// `encoding_rs::Encoding` object if possible.  The `hint` value will
    /// be used as a default unless we're not reasonably sure what
    /// character set is used by `data.
    fn detect_encoding(data: &[u8], hint: &'static Encoding)
                       -> Result<&'static Encoding>;

    /// Decode `data` to UTF-8 bytes **which may not be fully valid UTF-8**
    /// in some cases.  This method should only be used if you need high
    /// performance.  If the input data looks like UTF-8, it will be
    /// returned without copying.
    ///
    /// Note that this should only be called on complete input data, not on
    /// chunks of streaming data.
    fn decode_as_utf8_bytes<'a>(data: &'a [u8], hint: &'static Encoding)
                            -> Result<(Cow<'a, [u8]>, Accuracy)>
    {
        let encoding: &Encoding = try!(Self::detect_encoding(data, hint));
        if encoding == encoding_rs::UTF_8 {
            // Our results might be invalid UTF-8 if we guessed the wrong
            // encoding, but they're lossless.
            Ok((Cow::Borrowed(data), Accuracy::Lossless))
        } else {
            let (decoded, _, lossy) = encoding.decode(data);
            Ok((Cow::Owned(decoded.into_bytes()),
                Accuracy::from_decode_result(lossy)))
        }
    }

    /// Decode `data` to a valid Rust UTF-8 string.  If the input data is
    /// valid UTF-8, it will be returned without copying.
    ///
    /// Note that this should only be called on complete input data, not on
    /// chunks of streaming data.
    fn decode<'a>(data: &'a [u8], hint: &'static Encoding)
              -> Result<(Cow<'a, str>, Accuracy)>
    {
        let encoding: &Encoding = try!(Self::detect_encoding(data, hint));
        if encoding == encoding_rs::UTF_8 {
            // Use Rust's built-in decoder, which can operate in place if
            // we have valid UTF-8.
            let decoded: Cow<_> = String::from_utf8_lossy(data);
            // If `from_utf8_lossy` borrows the input string, it means we
            // didn't have any invalid code sequences that needed to be
            // replaced.
            let accuracy = match decoded {
                Cow::Borrowed(_) => Accuracy::Lossless,
                Cow::Owned(_) => Accuracy::Lossy,
            };
            Ok((decoded, accuracy))
        } else {
            let (decoded, _, lossy) = encoding.decode(data);
            Ok((Cow::Owned(decoded), Accuracy::from_decode_result(lossy)))
        }
    }
}

/// This detector recognizes Unicode byte order marks (BOM) and handles
/// them appropriately, but otherwise assumes the input is in the character
/// set specified by the `hint` argument to the detection or decoding
/// function.
///
/// This is intended to be very fast, and to give good results on a mix of
/// UTF-8 and UTF-16 files if called with `Hint::Utf8`.
pub struct BomDetector;

impl EncodingDetector for BomDetector {
    fn detect_encoding(data: &[u8], hint: &'static Encoding)
                       -> Result<&'static Encoding> {
        Ok(Encoding::for_bom(data).unwrap_or(hint))
    }
}

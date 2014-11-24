#![feature(macro_rules)] 
#![feature(globs)]

extern crate encoding;
extern crate uchardet;

use encoding::EncodingRef;
use encoding::label::encoding_from_whatwg_label;

/*
Here's a list of all the places I could find in the uchardet source which
returned a charset name.  Note that I may have missed some, and that other
versions of uchardet might add or remove character sets.

src/nsBig5Prober.h:  const char* GetCharSetName() {return "Big5";}
src/nsEUCJPProber.h:  const char* GetCharSetName() {return "EUC-JP";}
src/nsEUCKRProber.h:  const char* GetCharSetName() {return "EUC-KR";}
src/nsEUCTWProber.h:  const char* GetCharSetName() {return "x-euc-tw";}
src/nsGB2312Prober.h:  const char* GetCharSetName() {return "gb18030";}
src/nsLatin1Prober.h:  const char* GetCharSetName() {return "windows-1252";}
src/nsSJISProber.h:  const char* GetCharSetName() {return "Shift_JIS";}
src/nsUTF8Prober.h:  const char* GetCharSetName() {return "UTF-8";}

extern const SMModel UTF8SMModel; "UTF-8",
extern const SMModel Big5SMModel; "Big5",
extern const SMModel EUCJPSMModel; "EUC-JP",
extern const SMModel EUCKRSMModel; "EUC-KR",
extern const SMModel EUCTWSMModel; "x-euc-tw",
extern const SMModel GB18030SMModel; "GB18030",
extern const SMModel SJISSMModel; "Shift_JIS",
extern const SMModel HZSMModel; "HZ-GB-2312"
extern const SMModel ISO2022CNSMModel; "ISO-2022-CN"
extern const SMModel ISO2022JPSMModel; "ISO-2022-JP"
extern const SMModel ISO2022KRSMModel; "ISO-2022-KR"

#define VISUAL_HEBREW_NAME ("ISO-8859-8")
#define LOGICAL_HEBREW_NAME ("windows-1255")

mDetectedCharset = "UTF-8";
mDetectedCharset = "UTF-16";

From README.md (this is different from what the code seems to suggest):

  * Unicode
    * UTF-8
    * UTF-16BE / UTF-16LE
    * UTF-32BE / UTF-32LE / X-ISO-10646-UCS-4-34121 / X-ISO-10646-UCS-4-21431
  * Chinese
    * ISO-2022-CN
    * BIG5
    * EUC-TW
    * GB18030
    * HZ-GB-23121
  * Japanese
    * ISO-2022-JP
    * SHIFT_JIS
    * EUC-JP
  * Korean
    * ISO-2022-KR
    * EUC-KR
  * Cyrillic
    * ISO-8859-5
    * KOI8-R
    * WINDOWS-1251
    * MACCYRILLIC
    * IBM866
    * IBM855
  * Greek
    * ISO-8859-7
    * WINDOWS-1253
  * Hebrew
    * ISO-8859-8
    * WINDOWS-1255
  * Others
    * WINDOWS-1252
*/

/// Map the output of uchardet::detect_encoding_name to an `Encoding`
/// object.
pub fn encoding_from_uchardet_name(name: &str) -> Option<EncodingRef> {
    match name {
        // uchardet uses this for both UTF-16LE and UTF-16LE, so we
        // can't return an unambiguous and correct answer here.
        "UTF-16" => None,
        _ => {
            let encoding =  encoding_from_whatwg_label(name);
            match encoding {
                // SECURITY: Is this reasonable?  Rather than replacing
                // the entire input with a single replacement character,
                // and the output with UTF-8, refuse to handle the
                // encoding at all.
                Some(e) if e.name() == "encoder-only-utf-8" => None,
                _ => encoding
            }
        }
    }
}

#[cfg(test)]
mod test {
    use encoding::types::Encoding;
    use encoding::all::*;
    use self::super::*;

    macro_rules! assert_encoding_for_name {
        ($encoding:expr, $name:expr) => ({
            let expected = $encoding;
            let actual = encoding_from_uchardet_name($name);
            assert!(actual.is_some());
            assert_eq!(expected.name(), actual.unwrap().name());
        })
    }

    #[test]
    fn test_encoding_from_uchardet_name() {
        assert_encoding_for_name!(BIG5_2003, "Big5");
        assert_encoding_for_name!(EUC_JP, "EUC-JP");
        assert_encoding_for_name!(WINDOWS_949, "EUC-KR");
        assert_encoding_for_name!(GB18030, "gb18030");
        assert_encoding_for_name!(GB18030, "GB18030");
        assert_encoding_for_name!(HZ, "HZ-GB-2312");
        assert_encoding_for_name!(ISO_2022_JP, "ISO-2022-JP");
        // These encodings are allegedly used mostly for XXS.  See:
        // http://www.cvedetails.com/cve/2012-0477
        // https://encoding.spec.whatwg.org/#names-and-labels
        // https://encoding.spec.whatwg.org/#replacement
        assert!(encoding_from_uchardet_name("ISO-2022-CN").is_none());
        assert!(encoding_from_uchardet_name("ISO-2022-KR").is_none());
        assert_encoding_for_name!(ISO_8859_8, "ISO-8859-8");
        assert_encoding_for_name!(WINDOWS_31J, "Shift_JIS");
        assert_encoding_for_name!(UTF_8, "UTF-8");
        // Problem: Both BE and LE map to UTF-16.
        assert!(encoding_from_uchardet_name("UTF-16").is_none());
        assert_encoding_for_name!(WINDOWS_1252, "windows-1252");
        assert_encoding_for_name!(WINDOWS_1255, "windows-1255");
        //assert_encoding_for_name!(, "x-euc-tw");
    }
}

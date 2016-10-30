//! Our interface to `uchardet`, which may be disabled at compile time.

use encoding_rs::Encoding;
use uchardet::detect_encoding_name;

use EncodingDetector;
use errors::*;

/// This detector uses `uchardet`, which scans the input data and makes a
/// statistical guess as to what character set is most likely in use.
pub struct UChardetDetector;

impl EncodingDetector for UChardetDetector {
    fn detect_encoding(data: &[u8], hint: &'static Encoding)
                       -> Result<&'static Encoding> {
        let name = try!(detect_encoding_name(data)
          // TODO: Should we return `hint` in this case?
          .chain_err(|| ErrorKind::CouldNotDetectEncoding));
        if name == "ASCII" && hint.is_ascii_compatible() {
            Ok(hint)
        } else {
            Encoding::for_label_no_replacement(name.as_bytes())
                .ok_or_else(|| {
                    ErrorKind::UnsupportedEncoding(name.into()).into()
                })
        }
    }
}


macro_rules! assert_encoding_for_name {
    ($encoding:ident, $name:expr) => ({
        let expected = $crate::encoding_rs::$encoding;
        let actual = Encoding::for_label($name.as_bytes())
            .expect("Expected an encoding");
        assert_eq!(expected.name(), actual.name());
    })
}

macro_rules! assert_no_encoding_for_name {
    ($name:expr) => ({
        assert_eq!(Encoding::for_label($name.as_bytes()), None);
    })
}

/// This is really a test that `Encoding::for_label` behaves the way we
/// expect.  We reserve the right to override some of these mappings at
/// higher levels.
#[test]
fn test_encoding_for_label_handles_uchardet_names_as_expected() {
    // Note that we may not want to map "ASCII" to 1252 at higher levels.
    assert_encoding_for_name!(WINDOWS_1252, "ASCII");
    assert_encoding_for_name!(BIG5, "BIG5");
    assert_encoding_for_name!(EUC_JP, "EUC-JP");
    assert_encoding_for_name!(EUC_KR, "EUC-KR");
    // This does not appear to be supported by the Encoding Standard.
    assert_no_encoding_for_name!("EUC-TW");
    assert_encoding_for_name!(GB18030, "GB18030");
    assert_encoding_for_name!(REPLACEMENT, "HZ-GB-2312");
    assert_encoding_for_name!(REPLACEMENT, "ISO-2022-CN");
    assert_encoding_for_name!(ISO_2022_JP, "ISO-2022-JP");
    assert_encoding_for_name!(REPLACEMENT, "ISO-2022-KR");
    assert_encoding_for_name!(WINDOWS_1252, "ISO-8859-1");
    assert_encoding_for_name!(ISO_8859_8, "ISO-8859-8");
    assert_encoding_for_name!(SHIFT_JIS, "SHIFT_JIS");
    // This may be related to EUC-KR, but we need feedback from somebody
    // who understands this a lot better than we do.
    assert_no_encoding_for_name!("UHC");
    // This maps to UTF_16LE because that's the most common, but the
    // decoder can actually decode either in the default mode by detecting
    // byte order marks, which is the only way `uchardet` can detect
    // either, so it's not a problem.
    assert_encoding_for_name!(UTF_16LE, "UTF-16");
    // This is not supported by the encoding standard, because it appears
    // to be rare in the wild.
    assert_no_encoding_for_name!("UTF-32");
    assert_encoding_for_name!(UTF_8, "UTF-8");
    assert_encoding_for_name!(WINDOWS_1252, "WINDOWS-1252");
    assert_encoding_for_name!(WINDOWS_1255, "WINDOWS-1255");
}

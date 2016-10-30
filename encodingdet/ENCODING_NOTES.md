# Comparing uchardet encodings to Encoding Standard encodings

The `encoding_rs` library supports all the encodings in
the [Encoding Standard][], which is what we want to target.  The `uchardet`
library targets a roughly similar, but not identical set, and some
massaging is necessary to get everything to work correctly.

[Encoding Standard]: https://www.w3.org/TR/encoding/

## Encoding Standard encodings provided by `encoding_rs`

This is a fixed list that is not expected to ever change.  It contains the
known legacy encodings that are required to deal with web pages found in
the wild.

- BIG5
- EUC_JP
- EUC_KR
- GB18030
- GBK
- IBM866
- ISO_2022_JP
- ISO_8859_10
- ISO_8859_13
- ISO_8859_14
- ISO_8859_15
- ISO_8859_16
- ISO_8859_2
- ISO_8859_3
- ISO_8859_4
- ISO_8859_5
- ISO_8859_6
- ISO_8859_7
- ISO_8859_8
- ISO_8859_8_I
- KOI8_R
- KOI8_U
- MACINTOSH
- REPLACEMENT
- SHIFT_JIS
- UTF_16BE
- UTF_16LE
- UTF_8
- WINDOWS_1250
- WINDOWS_1251
- WINDOWS_1252
- WINDOWS_1253
- WINDOWS_1254
- WINDOWS_1255
- WINDOWS_1256
- WINDOWS_1257
- WINDOWS_1258
- WINDOWS_874
- X_MAC_CYRILLIC
- X_USER_DEFINED

## Encodings provided by `uchardet`

There's a list of encodings in the `uchardet` documentation, but it has
historically not corresponded to the `uchardet` source code.

Here's a list of all the places I could find in the `uchardet` source which
returned a charset name.  Note that I may have missed some, and that other
versions of uchardet might add or remove character sets.

```
src/nsSJISProber.h:60:  const char* GetCharSetName() {return "SHIFT_JIS";}
src/nsBig5Prober.h:53:  const char* GetCharSetName() {return "BIG5";}
src/nsEUCTWProber.h:53:  const char* GetCharSetName() {return "EUC-TW";}
src/nsGB2312Prober.h:55:  const char* GetCharSetName() {return "GB18030";}
src/nsEscCharsetProber.h:51:  const char* GetCharSetName() {return mDetectedCharset;}
src/nsLatin1Prober.h:51:  const char* GetCharSetName() {return "WINDOWS-1252";}
src/nsEUCKRProber.h:59:  const char* GetCharSetName() {return "UHC";}
src/nsUTF8Prober.h:51:  const char* GetCharSetName() {return "UTF-8";}
src/nsEUCJPProber.h:59:  const char* GetCharSetName() {return "EUC-JP";}

src/nsCodingStateMachine.h:89:extern const SMModel UTF8SMModel; "UTF-8"
src/nsCodingStateMachine.h:90:extern const SMModel Big5SMModel; "BIG5"
src/nsCodingStateMachine.h:91:extern const SMModel EUCJPSMModel; "EUC-JP"
src/nsCodingStateMachine.h:92:extern const SMModel EUCKRSMModel; "EUC-KR"
src/nsCodingStateMachine.h:93:extern const SMModel EUCTWSMModel; "EUC-TW"
src/nsCodingStateMachine.h:94:extern const SMModel GB18030SMModel; "GB18030"
src/nsCodingStateMachine.h:95:extern const SMModel SJISSMModel; "SHIFT_JIS"
src/nsCodingStateMachine.h:98:extern const SMModel HZSMModel; "HZ-GB-2312"
src/nsCodingStateMachine.h:99:extern const SMModel ISO2022CNSMModel; "ISO-2022-CN"
src/nsCodingStateMachine.h:100:extern const SMModel ISO2022JPSMModel; "ISO-2022-JP"
src/nsCodingStateMachine.h:101:extern const SMModel ISO2022KRSMModel; "ISO-2022-KR"

src/nsHebrewProber.cpp:61:#define VISUAL_HEBREW_NAME ("ISO-8859-8")
src/nsHebrewProber.cpp:62:#define LOGICAL_HEBREW_NAME ("WINDOWS-1255")

src/nsUniversalDetector.cpp:122: mDetectedCharset = "UTF-8";
src/nsUniversalDetector.cpp:127: mDetectedCharset = "UTF-16";
src/nsUniversalDetector.cpp:137: mDetectedCharset = "UTF-32";
src/nsUniversalDetector.cpp:247: mDetectedCharset = "ISO-8859-1";
src/nsUniversalDetector.cpp:253: mDetectedCharset = "ASCII";
```

If we sort this and remove duplicates, we get:

- ASCII
- BIG5
- EUC-JP
- EUC-KR
- EUC-TW
- GB18030
- HZ-GB-2312
- ISO-2022-CN
- ISO-2022-JP
- ISO-2022-KR
- ISO-8859-1
- ISO-8859-8
- SHIFT_JIS
- UHC
- UTF-16
- UTF-32
- UTF-8
- WINDOWS-1252
- WINDOWS-1255

## `uchardet` encodings unknown to `encoding_rs`

The following encodings may be detected by `uchardet`, but `encoding_rs`
doesn't know what to do with them:

- `ASCII`.  This could be plausibly mapped to `UTF_8`, but if it appears as
  an actual, written-out encoding name, the Encoding Standard normally maps
  it to `WINDOWS-1252` .
- `EUC_TW`.  This does not appear to be supported by the Encoding Standard.
- `HZ_GB_2312`.  Mapped to `REPLACEMENT` by the Encoding Standard and
  `encoding_rs`.
- `ISO_2022_CN`.  Mapped to `REPLACEMENT`.
- `ISO_2022_KR`.  Mapped to `REPLACEMENT`.
- `ISO_8859_1`.  Mapped to `WINDOWS-1252` by the Encoding Standard and
  `encoding_rs`.
- `UHC`.  This _might_ correspond to `EUC-KR`, but we need input from
  somebody who's familiar with this encoding and the Encoding Standard.
- `UTF_32`.  This is not supported by the Encoding Standard, probably
  because it almost never appears in the wild.

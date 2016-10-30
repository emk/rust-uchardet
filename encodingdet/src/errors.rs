//! Error handling.

// Regrettably necessary because of limitations in `error-chain`.
#![allow(missing_docs)]

error_chain! {
    // TODO: We'd like to declare `links` here for `uchardet`, but we're
    // not guaranteed to have it available, and `error_chain` doesn't
    // support conditional links.

    errors {
        CouldNotDetectEncoding {
            description("could not detect encoding used by text")
            display("could not detect encoding used by text")
        }
        UnsupportedEncoding(name: String) {
            description("unsupported encoding")
            display("unsupported encoding: '{}'", &name)
        }
    }
}

#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(unused_extern_crates)]
#![cfg_attr(all(test, feature = "nightly"), feature(test))]

//! # hyper*x*
//!
//! Hyper is the low-level HTTP implementation for Rust. Hyper*x* is an
//! e*x*traction of the hyper 0.11 typed header module, with minimized
//! dependencies, for continued use with hyper 0.12 or later, where
//! this module was removed in preference to the byte-oriented `http::header`
//! module.
//!
//! See the [*header*](header/index.html) module for more details.

extern crate base64;
extern crate bytes;
extern crate http;
extern crate httpdate;
extern crate language_tags;
pub extern crate mime;
extern crate percent_encoding;
extern crate unicase;

#[cfg(all(test, feature = "nightly"))]
extern crate test;

pub use error::{Error, Result};

#[cfg(feature = "headers")]
pub use header::Headers;

pub use method::Method;

mod error;
pub mod header;
mod method;

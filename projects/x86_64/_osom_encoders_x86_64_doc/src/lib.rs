//! This crate contains the documentation for the `X86_64` architecture,
//! as a Rust struct.
//!
//! Internally it reads the `x86.yaml` file and parses it into a Rust struct.
//!
//! This crate is not intended to be used directly or published. The point is
//! to generate code in `osom_encoders_x86_64` from it.
#![deny(warnings)]
#![allow(unused_features)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::inline_always)]

mod custom_deserializers;
mod validation;

mod x86_doc;
pub use x86_doc::*;

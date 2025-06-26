//! This crate provides a set of functions to encode x86-64 instructions.
//!
//! Most of the encoders here are unsafe, e.g. they don't validate the operands.
//! It is up to the caller to ensure that the operands are valid.
//!
//! The crate tries to follow x86 Intel's manual conventions as close
//! as possible.
#![cfg_attr(not(debug_assertions), deny(warnings))]
#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::needless_return,
    clippy::redundant_field_names,
    clippy::unreadable_literal,
    clippy::inline_always,
    clippy::module_name_repetitions,
    clippy::len_without_is_empty,
    clippy::should_implement_trait,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc
)]
#![no_std]

pub mod encoders;
pub mod models;

mod invariants;

// #![warn(missing_docs)]
//! Sequence features in your browser!
//!
//! Provides a selection of RNA appropriate sequence features
//! which can be extracted to use for ML, or anything you like
//!
//! The library is intended to compile to WASM for client-side
//! extraction, but there's nothing to stop you compiling for
//! host execution if you want to!
//!

#[macro_use]
extern crate lazy_static;

pub mod coding;
pub mod stats;
pub mod utils;

#[cfg(test)]
mod tests;

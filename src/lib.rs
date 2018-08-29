//! Exports the 0.11 error chain types.
//!
//! This is useful for systems that want to work with the error chain traits
//! and types even if they are from different versions of error chain.

extern crate error_chain;

pub use error_chain::{Backtrace, DisplayChain, Iter, ChainedError, ExitCode};

//! Schemaful serialisation of self-describing data types.
//!
//! This crate is essentially divided into two parts:
//!
//! 1. The [`encoding`] module, which is format agnostic.  This provides the machinery for types to
//!    describe their schema via trait implementations that ensure compile-time coherence with the
//!    serialised data, through Rust's type system.  A derive macro is provided by the `cu29_derive`
//!    crate to ease implementation in regular cases.
//!
//! 2. Implementations of particular serialisation formats.  A skeleton of the ROS 2.0 message format
//!    is currently implemented, showcasing in broad terms how the library can be extended to support
//!    other serialisation formats.

extern crate self as cu29_encode;

#[macro_use]
mod encoding;
mod ros2msg;

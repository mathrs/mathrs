/*

    MathRS ─ Scientific, numeric and symbolic mathematical crate for computing with Rust.
    Copyright (c) 2020 mathrs team

*/

// Crate name for https://crates.io
#![crate_name = "mathrs"]

// Deny missing documentation and code examples!
#![deny(missing_docs)]
#![deny(missing_doc_code_examples)]

// Documentation cute logo.png and favicon.ico for docs.rs (ᵔᴥᵔ)
#![doc(html_logo_url = "https://raw.githubusercontent.com/mathrs/mathrs/master/logo.png")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/mathrs/mathrs/master/favicon.ico")]
#![doc(issue_tracker_base_url = "https://github.com/mathrs/mathrs/issues/")]

//! <img src="https://raw.githubusercontent.com/mathrs/mathrs/master/logo.png" width="256px" height="256px">
//!
//! <img src="https://img.shields.io/crates/v/mathrs?style=flat-square"> <img src="https://img.shields.io/crates/d/mathrs?style=flat-square">
//!
//!## About
//!
//! **Important!** This project is still in early development! All of its features implemented over the minor versions are fully working, but prone to changes.
//!
//! **mathrs** is a scientific, numeric and symbolic mathematical crate for computing with [Rust](https://rust-lang.org).
//!
//! - **Website** ─ https://mathrs.github.io
//! - **Reference** ─ https://mathrs.github.io/reference
//! - **Documentation** ─ https://docs.rs/mathrs
//!
//! ## Installation
//! You can install mathrs on your project by inserting the following on your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! mathrs = "*"
//! ```

// src/point.rs
mod constants;
pub use constants::*;

// src/point.rs
mod point;
pub use point::*;

// src/vector.rs
mod vector;
pub use vector::*;

// src/vector3d.rs
mod vector3d;
pub use vector3d::*;

// src/tests.rs
mod tests;
pub use tests::*;

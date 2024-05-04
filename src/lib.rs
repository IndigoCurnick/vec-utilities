//! # Vec Utilities
//!
//! `vec-utilites` is a collection of tools which make working with `Vec` of
//! floats easier
//!
//! ## Motivation
//!
//! In Rust, since `f32` and `f64` strictly adhere to the float definition
//! standards, and Rust avoids all unexpected behaviour, working with Vecs of
//! floats can be challenging. For instance, since floats do not implement
//! `Ord` in Rust (due to `NaN`) then there is no built in way to get the largest
//! value in a `Vec` of floats.
//!
//! This crate offers some "common sense" implementation of common operations
//!
//! ## How This Crate is Organised
//!
//! The crate has the following modules
//!
//! - `filters`
//! - `generation`
//! - `maths`
//! - `running`
//!
//! ### `filters`
//!
//! This is where you can find some ready made filters, for example, removing NaN
//! from the Vec
//!
//! ### `generation`
//!
//! `generation` focuses on generating Vecs of floats, typically for the purpose
//! of iteration. For instance, you can find an `arange` function here
//!
//! ### `maths`
//!
//! `maths` contains statistical properties of Vecs - largest and smallest values,
//! means, medians and so on
//!
//! ### `running`
//!
//! `running` contains iterators, like running means and running sums

pub mod filters;
pub mod generation;
pub mod maths;
pub mod running;

// TODO: https://stackoverflow.com/questions/68449860/how-to-make-a-general-function-that-groups-an-generic-intoiterators-items-in?rq=3

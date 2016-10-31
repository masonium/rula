//! # rula
//!
//! `rula` is a linear algebra package on top of `ndarray`.
#![macro_use]

#[macro_use]
extern crate ndarray;
extern crate lapack;
extern crate num_traits;

pub mod util;

pub mod eigenvalues;
pub mod svd;
pub mod solve_linear;
pub mod least_squares;
pub mod types;

#[macro_use]
pub mod prelude;

mod impl_prelude;

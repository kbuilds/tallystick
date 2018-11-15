#![warn(rust_2018_idioms)]
#![warn(missing_docs)]
#![feature(crate_visibility_modifier)]
#![feature(nll)]


#[allow(unused_imports)]
#[macro_use] extern crate indexmap;
#[macro_use] extern crate derive_more;

extern crate hashbrown;
extern crate petgraph;
extern crate num_traits;

pub mod plurality;
pub mod stv;
pub mod condorcet;
pub mod result;
pub mod approval;
pub mod score;

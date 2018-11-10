#![warn(rust_2018_idioms)]
#![warn(missing_docs)]

#[allow(unused_imports)]
#[macro_use] extern crate indexmap;

extern crate hashbrown;
extern crate petgraph;

pub mod plurality;
pub mod stv;
pub mod condorcet;
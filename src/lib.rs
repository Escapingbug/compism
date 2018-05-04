extern crate libloading as lib;
#[macro_use] extern crate failure;
#[macro_use] extern crate lazy_static;

pub mod plugin;
pub mod route;
pub mod compism;
pub mod error;
pub mod external;

pub use compism::*;

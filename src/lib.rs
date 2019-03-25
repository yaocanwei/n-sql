// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
#![feature(const_fn)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

extern crate cfg_if;
extern crate core;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;

use cfg_if::cfg_if;
mod ast;
mod catalog;
mod grammar;
mod lexer;
mod optimizer;
mod version;
pub mod generator;
pub mod parser;

pub use ast::*;
pub use grammar::{
    ExpressionEntryParser as ExpressionParser, PredicateEntryParser as PredicateParser,
    StatementEntryParser as StatementParser,
};
pub use lexer::Lexer;
pub use optimizer::Optimizer;


#[cfg(target_arch="wasm32")]
extern crate wasm_bindgen;
#[cfg(target_arch="wasm32")]
mod wasm;






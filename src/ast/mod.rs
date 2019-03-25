// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt::{Debug, Error, Formatter};

mod data_type;
mod expression;
mod identifier;
mod sorting_direction;
mod statement;

pub use self::data_type::*;
pub use self::expression::*;
pub use self::identifier::*;
pub use self::sorting_direction::*;
pub use self::statement::*;

#[derive(Clone, Debug)]
pub enum Ast {
    Statement(Statement),
    Expression(Expression),
}

#[derive(Clone, Debug)]
pub struct Column {
    pub schema: Option<Identifier>,
    pub table: Option<Identifier>,
    pub name: Identifier,
}

impl Column {
    pub fn new(name: Identifier, table: Option<Identifier>, schema: Option<Identifier>) -> Column {
        Column {
            name,
            table,
            schema,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Table {
    pub database: Option<Identifier>,
    pub schema: Option<Identifier>,
    pub name: Identifier,
}

impl Table {
    pub fn new(
        name: Identifier,
        schema: Option<Identifier>,
        database: Option<Identifier>,
    ) -> Table {
        Table {
            name,
            schema,
            database,
        }
    }
}

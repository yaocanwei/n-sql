// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod join_node;
mod pagination_statement;
mod select_statement;
mod set_statement;
mod table_view;

pub use self::join_node::*;
pub use self::pagination_statement::*;
pub use self::select_statement::*;
pub use self::set_statement::*;
pub use self::table_view::*;

#[derive(Clone, Debug)]
pub enum Statement {
    Set(Box<SetStatement>),
}

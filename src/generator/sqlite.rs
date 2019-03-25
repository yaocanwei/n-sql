// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::Visitor;
use ast::*;
use optimizer::Optimizer;
use std::fmt::{Error, Result, Write};
use std::result;

type Formatter = String;

pub trait SQLiterGenerator<T> {
    fn to_sqlite(&self) -> result::Result<String, Error>;
}

struct InternalGenerator;

impl Visitor for InternalGenerator {
    fn visit_nvl_fn(&self, function: &Box<NvlFn>, f: &mut Formatter) -> Result {
        f.write_str("ifnull")?;
        f.write_char('(')?;
        self.visit_expression(&function.expr, f)?;
        f.write_str(", ")?;
        self.visit_expression(&function.default, f)?;
        f.write_char(')')
    }
    fn visit_cast_fn(&self, function: &Box<CastFn>, f: &mut Formatter) -> Result {
        let lower_tp = function.data_type.data_type.to_lowercase();
        let tp = lower_tp.as_str();

        match tp {
            "timestamp" | "datetime" | "date" | "time" => {
                f.write_str(if tp == "timestamp" {
                    "datetime"
                } else {
                    &function.data_type.data_type
                })?;
                f.write_char('(')?;
                self.visit_expression(&function.expr, f)?;
                f.write_char(')')
            }
            _ => {
                f.write_str("cast")?;
                f.write_char('(')?;
                self.visit_expression(&function.expr, f)?;
                f.write_str(" as ")?;
                f.write_str(&function.data_type.data_type)?;
                f.write_char(')')
            }
        }
    }
}

impl SQLiterGenerator<Expression> for Expression {
    fn to_sqlite(&self) -> result::Result<String, Error> {
        let mut s = String::new();
        InternalGenerator.visit_expression(self, &mut s)?;
        Ok(s)
    }
}

impl SQLiterGenerator<PredicateExpression> for PredicateExpression {
    fn to_sqlite(&self) -> result::Result<String, Error> {
        let mut s = String::new();
        InternalGenerator.visit_predicate(self, &mut s)?;
        Ok(s)
    }
}

impl SQLiterGenerator<Statement> for Statement {
    fn to_sqlite(&self) -> result::Result<String, Error> {
        let mut s = String::new();
        InternalGenerator.visit_statement(self, &mut s)?;
        Ok(s)
    }
}

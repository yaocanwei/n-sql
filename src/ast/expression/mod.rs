// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt::{Debug, Error, Formatter};

mod predicate_expression;
mod scalar_expression;
mod vector_expression;

pub use self::predicate_expression::*;
pub use self::scalar_expression::*;
pub use self::vector_expression::*;

#[derive(Clone, Debug)]
pub enum Expression {
    Vector(VectorExpression),
    Scalar(ScalarExpression),
}

impl From<ScalarExpression> for Expression {
    fn from(v: ScalarExpression) -> Self {
        Expression::Scalar(v)
    }
}

impl From<ConstantValue> for Expression {
    fn from(v: ConstantValue) -> Self {
        Expression::Scalar(v.into())
    }
}

impl From<CastFn> for Expression {
    fn from(v: CastFn) -> Self {
        Expression::Scalar(ScalarExpression::Function(Function::Cast(v.into()).into()))
    }
}

impl From<i32> for Expression {
    fn from(value: i32) -> Self {
        ConstantValue::from(value).into()
    }
}

impl Expression {
    pub fn constant_numeric(&self) -> Option<NumericValue> {
        match self {
            Expression::Scalar(s) => match s {
                ScalarExpression::Constant(t) => match t {
                    ConstantValue::Numeric(n) => Some(n.clone()),
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        }
    }
}

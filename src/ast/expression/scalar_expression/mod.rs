// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod arithmetic_expression;
mod case_when_expression;
mod constant;
mod function;
mod unary_expression;
mod variable;

pub use self::arithmetic_expression::*;
pub use self::case_when_expression::*;
pub use self::constant::*;
pub use self::function::*;
pub use self::unary_expression::*;
pub use self::variable::*;

use ast::Column;

#[derive(Clone, Debug)]
pub enum ScalarExpression {
    Constant(ConstantValue),
    Column(Column),
    Variable(Variable),

    Unary(UnaryExpression),
    Arithmetic(ArithmeticExpression),
    CaseWhen(CaseWhenExpression),
    Function(Function),
}

impl From<ConstantValue> for ScalarExpression {
    fn from(v: ConstantValue) -> Self {
        ScalarExpression::Constant(v)
    }
}

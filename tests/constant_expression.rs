// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate n_sql;

use n_sql::generator::*;
use n_sql::ConstantValue;
use n_sql::Expression;
use n_sql::ExpressionParser;
use n_sql::Lexer;
use n_sql::NumericValue;
use n_sql::Optimizer;
use n_sql::ScalarExpression;

#[macro_use]
mod common;

test_init!();

#[test]
fn test() {
    let left: NumericValue = 1i32.into();
    let right: NumericValue = 2.2.into();
    let result: Box<Expression> = Expression::Scalar(ScalarExpression::Constant(
        ConstantValue::Numeric(left + right),
    ))
    .into();
    assert_eq!("3.2", result.to_sql().unwrap());
}

#[test]
fn test_integer() {
    test_expression(NSQL, "3", "3");
}

#[test]
fn test_integer_minus() {
    test_expression(NSQL, "-3", "-3");
}

#[test]
fn test_string() {
    test_expression(NSQL, " 'abc' ", "'abc'");
}

#[test]
fn test_float() {
    test_expression(NSQL, "3.2", "3.2");
}
#[test]
fn test_float1() {
    test_expression(NSQL, ".2", "0.2")
}

#[test]
fn test_float_minus() {
    test_expression(NSQL, "-.2", "-0.2");
}

#[test]
fn test_optimizer() {
    let expr = ExpressionParser::new()
        .parse(Lexer::new("2+4.7").tokenizer())
        .unwrap();
    assert_eq!("6.7", Box::new(expr.optimize()).to_sql().unwrap());
}

#[test]
fn test_optimizer1() {
    let expr = ExpressionParser::new()
        .parse(Lexer::new("2+4.7*10").tokenizer())
        .unwrap();
    assert_eq!("49", Box::new(expr.optimize()).to_sql().unwrap())
}
#[test]
fn test_optimizer2() {
    let expr = ExpressionParser::new()
        .parse(Lexer::new("(2+4.7)*10").tokenizer())
        .unwrap();
    assert_eq!("67", Box::new(expr.optimize()).to_sql().unwrap())
}

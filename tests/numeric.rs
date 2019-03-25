// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
mod common;

test_init!();

#[theory]
#[test]
#[case("ceil(-6.8)", NSQL, "ceil(-6.8)")]
#[case("ceil(-6.8)", PostgreSQL, "ceil(-6.8)")]
#[case("ceil(-6.8)", Oracle, "ceil(-6.8)")]
#[case("ceil(-6.8)", MySQL, "ceil(-6.8)")]
//#[case("ceil(-6.8)", SqlServer, "ceiling(-6.8)")]
//#[case("ceil(-6.8)", SQLite, "ceil(-6.8)")]
fn test_ceil(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("cos(6.8)", NSQL, "cos(6.8)")]
#[case("cos(6.8)", PostgreSQL, "cos(6.8)")]
#[case("cos(6.8)", Oracle, "cos(6.8)")]
#[case("cos(6.8)", MySQL, "cos(6.8)")]
//#[case("cos(6.8)", SQLite, "cos(6.8)")]
fn test_cos(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("floor(-6.8)", NSQL, "floor(-6.8)")]
fn test_floor(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("sqrt(6.855)", NSQL, "sqrt(6.855)")]
fn test_sqrt(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("tan(6.855)", NSQL, "tan(6.855)")]
fn test_tan(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("pow(6.8, 2)", NSQL, "pow(6.8, 2)")]
#[case("power(6.8, 2)", NSQL, "pow(6.8, 2)")]
fn test_pow(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("log(6.8)", NSQL, "log(6.8)")]
#[case("log(6.8, 2)", NSQL, "log(6.8, 2)")]
#[case("log(2, 3)", PostgreSQL, "log(2, 3)")]
#[case("log(2, 3)", Oracle, "log(2, 3)")]
#[case("log(2, 3)", MySQL, "log(2, 3)")]
//#[case("log(2, 3)", SqlServer, "log(3, 2)")]
#[case("log10(6.8)", NSQL, "log10(6.8)")]
//#[case("log10(2)", NSQL, "log10(2)")]
//#[case("log10(2)", PostgreSQL, "log(10, 2)")]
//#[case("log10(2)", Oracle, "log(10, 2)")]
//#[case("log10(2)", MySQL, "log10(2)")]
//#[case("log10(2)", SqlServer, "log10(2)")]

//#[case("log(10, 2)", NSQL, "log10(2)")]
//#[case("log(10, 2)", PostgreSQL, "log(10, 2)")]
//#[case("log(10, 2)", Oracle, "log(10, 2)")]
//#[case("log(10, 2)", MySQL, "log10(2)")]
//#[case("log(10, 2)", SqlServer, "log10(2)")]

//#[case("log(2)", NSQL, "log10(2)")]
#[case("log(2)", PostgreSQL, "log(2)")]
#[case("log(2)", Oracle, "log(2)")]
#[case("log(2)", MySQL, "log(2)")]
//#[case("log(2)", SqlServer, "log(2)")]
fn test_log(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("sin(6.855)", NSQL, "sin(6.855)")]
fn test_sin(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("sign(-6.855)", NSQL, "sign(-6.855)")]
fn test_sign(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("round(6.855, 2)", NSQL, "round(6.855, 2)")]
#[case("round(6.855, 2 + b)", NSQL, "round(6.855, 2 + b)")]
fn test_round(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("abs(-6)", NSQL, "abs(-6)")]
#[case("abs(9)", NSQL, "abs(9)")]
fn test_abs(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
//#[case("rank(a)", NSQL, "rank(a)")]
//#[case("rank(a  , asc)", NSQL, "rank(a, asc)")]
//#[case("rank(a  , desc)", NSQL, "rank(a, desc)")]
//#[case("rank(count(a))", NSQL, "rank(count(a))")]
//#[case("rank(count(a), asc)", NSQL, "rank(count(a), asc)")]
//#[case("rank(count(a), desc)", NSQL, "rank(count(a), desc)")]
//
//#[case("rank() over (order by a)", NSQL, "rank(a)")]
//#[case("rank() over (order by a asc)", NSQL, "rank(a, asc)")]
//#[case("rank() over (order by a desc)", NSQL, "rank(a, desc)")]
//
//#[case("dense_rank(a)", NSQL, "dense_rank(a)")]
//#[case("dense_rank(a  , asc)", NSQL, "dense_rank(a, asc)")]
//#[case("dense_rank(a  , desc)", NSQL, "dense_rank(a, desc)")]
//
//#[case("dense_rank() over (order by a)", NSQL, "dense_rank(a)")]
//#[case("dense_rank() over (order by a asc)", NSQL, "dense_rank(a, asc)")]
//#[case("dense_rank() over (order by a desc)", NSQL, "dense_rank(a, desc)")]
//
//
//
//
//
//#[case("rank(a)", PostgreSQL, "rank() over (order by a)")]
//#[case("rank(a  , asc)", PostgreSQL, "rank() over (order by a asc)")]
//#[case("rank(a  , desc)", PostgreSQL, "rank() over (order by a desc)")]
//#[case("rank(count(a))", PostgreSQL, "rank() over (order by count(a))")]
//#[case("rank(count(a), asc)", PostgreSQL, "rank() over (order by count(a) asc)")]
//#[case("rank(count(a), desc)", PostgreSQL, "rank() over (order by count(a) desc)")]
//
//#[case("rank() over (order by a)", PostgreSQL, "rank() over (order by a)")]
//#[case("rank() over (order by a asc)", PostgreSQL, "rank() over (order by a asc)")]
//#[case("rank() over (order by a desc)", PostgreSQL, "rank() over (order by a desc)")]
//
//#[case("dense_rank(a)", PostgreSQL, "dense_rank() over (order by a)")]
//#[case("dense_rank(a  , asc)", PostgreSQL, "dense_rank() over (order by a asc)")]
//#[case("dense_rank(a  , desc)", PostgreSQL, "dense_rank() over (order by a desc)")]
//
//#[case("dense_rank() over (order by a)", PostgreSQL, "dense_rank() over (order by a)")]
//#[case("dense_rank() over (order by a asc)", PostgreSQL, "dense_rank() over (order by a asc)")]
//#[case("dense_rank() over (order by a desc)", PostgreSQL, "dense_rank() over (order by a desc)")]
//
//
//
//
//#[case("rank(a)", Oracle, "rank() over (order by a)")]
//#[case("rank(a  , asc)", Oracle, "rank() over (order by a asc)")]
//#[case("rank(a  , desc)", Oracle, "rank() over (order by a desc)")]
//#[case("rank(count(a))", Oracle, "rank() over (order by count(a))")]
//#[case("rank(count(a), asc)", Oracle, "rank() over (order by count(a) asc)")]
//#[case("rank(count(a), desc)", Oracle, "rank() over (order by count(a) desc)")]
//
//#[case("rank() over (order by a)", Oracle, "rank() over (order by a)")]
//#[case("rank() over (order by a asc)", Oracle, "rank() over (order by a asc)")]
//#[case("rank() over (order by a desc)", Oracle, "rank() over (order by a desc)")]
//
//#[case("dense_rank(a)", Oracle, "dense_rank() over (order by a)")]
//#[case("dense_rank(a  , asc)", Oracle, "dense_rank() over (order by a asc)")]
//#[case("dense_rank(a  , desc)", Oracle, "dense_rank() over (order by a desc)")]
//
//#[case("dense_rank() over (order by a)", Oracle, "dense_rank() over (order by a)")]
//#[case("dense_rank() over (order by a asc)", Oracle, "dense_rank() over (order by a asc)")]
//#[case("dense_rank() over (order by a desc)", Oracle, "dense_rank() over (order by a desc)")]
//
//
//
//
//#[case("rank(a)", MySQL, "rank() over (order by a)")]
//#[case("rank(a  , asc)", MySQL, "rank() over (order by a asc)")]
//#[case("rank(a  , desc)", MySQL, "rank() over (order by a desc)")]
//#[case("rank(count(a))", MySQL, "rank() over (order by count(a))")]
//#[case("rank(count(a), asc)", MySQL, "rank() over (order by count(a) asc)")]
//#[case("rank(count(a), desc)", MySQL, "rank() over (order by count(a) desc)")]
//
//#[case("rank() over (order by a)", MySQL, "rank() over (order by a)")]
//#[case("rank() over (order by a asc)", MySQL, "rank() over (order by a asc)")]
//#[case("rank() over (order by a desc)", MySQL, "rank() over (order by a desc)")]
//
//#[case("dense_rank(a)", MySQL, "dense_rank() over (order by a)")]
//#[case("dense_rank(a  , asc)", MySQL, "dense_rank() over (order by a asc)")]
//#[case("dense_rank(a  , desc)", MySQL, "dense_rank() over (order by a desc)")]
//
//#[case("dense_rank() over (order by a)", MySQL, "dense_rank() over (order by a)")]
//#[case("dense_rank() over (order by a asc)", MySQL, "dense_rank() over (order by a asc)")]
//#[case("dense_rank() over (order by a desc)", MySQL, "dense_rank() over (order by a desc)")]
//
//
//
//
//#[case("rank(a)", SqlServer, "rank() over (order by a)")]
//#[case("rank(a  , asc)", SqlServer, "rank() over (order by a asc)")]
//#[case("rank(a  , desc)", SqlServer, "rank() over (order by a desc)")]
//#[case("rank(count(a))", SqlServer, "rank() over (order by count(a))")]
//#[case("rank(count(a), asc)", SqlServer, "rank() over (order by count(a) asc)")]
//#[case("rank(count(a), desc)", SqlServer, "rank() over (order by count(a) desc)")]
//
//#[case("rank() over (order by a)", SqlServer, "rank() over (order by a)")]
//#[case("rank() over (order by a asc)", SqlServer, "rank() over (order by a asc)")]
//#[case("rank() over (order by a desc)", SqlServer, "rank() over (order by a desc)")]
//
//#[case("dense_rank(a)", SqlServer, "dense_rank() over (order by a)")]
//#[case("dense_rank(a  , asc)", SqlServer, "dense_rank() over (order by a asc)")]
//#[case("dense_rank(a  , desc)", SqlServer, "dense_rank() over (order by a desc)")]
//
//#[case("dense_rank() over (order by a)", SqlServer, "dense_rank() over (order by a)")]
//#[case("dense_rank() over (order by a asc)", SqlServer, "dense_rank() over (order by a asc)")]
//#[case("dense_rank() over (order by a desc)", SqlServer, "dense_rank() over (order by a desc)")]

fn test_rank(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

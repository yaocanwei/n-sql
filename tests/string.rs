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
#[case("concat(a, b ,c)", NSQL, "concat(a, b, c)")]
#[case("concat('a', 'b' ,'c')", NSQL, "concat('a', 'b', 'c')")]
fn test_concat(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("UPPER(a)", NSQL, "upper(a)")]
#[case("Upper('abc')", NSQL, "upper('abc')")]
fn test_upper(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("lower(a)", NSQL, "lower(a)")]
#[case("lower('abc')", NSQL, "lower('abc')")]
fn test_lower(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("LENGTH(a)", NSQL, "length(a)")]
#[case("Length('abc')", NSQL, "length('abc')")]
fn test_length(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("pad_left('abc', 3)", NSQL, "pad_left('abc', 3)")]
#[case("pad_left('abc', 3, 'a')", NSQL, "pad_left('abc', 3, 'a')")]
#[case("lpad('abc', 3, 'a')", NSQL, "pad_left('abc', 3, 'a')")]
#[case("pad_right('abc', 3)", NSQL, "pad_right('abc', 3)")]
#[case("pad_right('abc', 3, 'a')", NSQL, "pad_right('abc', 3, 'a')")]
#[case("rpad('abc', 3, 'a')", NSQL, "pad_right('abc', 3, 'a')")]
fn test_pad(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("substr('abc', 2)", NSQL, "substr('abc', 2)")]
#[case("substr('abc', 1,1)", NSQL, "substr('abc', 1, 1)")]
fn test_substr(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
//#[case("replace('123nj', '1', 'pp')", NSQL, "replace('123nj', '1', 'pp')")]
fn test_replace(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("trim_end('abc ')", NSQL, "trim_end('abc ')")]
#[case("trim_end('abc ', 'd')", NSQL, "trim_end('abc ', 'd')")]
#[case("rtrim('abc', 'a')", NSQL, "trim_end('abc', 'a')")]
#[case("trim(trailing  'a' from 'abc')", NSQL, "trim_end('abc', 'a')")]
#[case("trim(trailing from 'abc ')", NSQL, "trim_end('abc ')")]
#[case("trim('abc ')", NSQL, "trim('abc ')")]
#[case("trim('abc ', 'a')", NSQL, "trim('abc ', 'a')")]
#[case("btrim('abc ')", NSQL, "trim('abc ')")]
#[case("btrim('abc ', 'a')", NSQL, "trim('abc ', 'a')")]
#[case("trim_start(' abc ')", NSQL, "trim_start(' abc ')")]
#[case("trim_start(' abc ', 'a')", NSQL, "trim_start(' abc ', 'a')")]
#[case("ltrim(' abc ', 'a')", NSQL, "trim_start(' abc ', 'a')")]
#[case("ltrim(' abc ')", NSQL, "trim_start(' abc ')")]
#[case("trim(leading from ' abc ')", NSQL, "trim_start(' abc ')")]
#[case("trim(leading 'a' from ' abc ')", NSQL, "trim_start(' abc ', 'a')")]
fn test_trim(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("reverse('123nj')", NSQL, "reverse('123nj')")]
#[case("reverse('123nj')", PostgreSQL, "reverse('123nj')")]
#[case("reverse('123nj')", Oracle, "reverse('123nj')")]
#[case("reverse('123nj')", MySQL, "reverse('123nj')")]
//#[case("reverse('123nj')", SqlServer, "reverse('123nj')")]
//#[case("reverse('123nj')", SQLite, "reverse('123nj')")]
#[case("reverse('abc')", NSQL, "reverse('abc')")]
fn test_reverse(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("left(a, 3)", NSQL, "left(a, 3)")]
#[case("left(a,2)", NSQL, "left(a, 2)")]
#[case("left(a,2)", PostgreSQL, "left(a, 2)")]
//#[case("left(a,2)", Oracle, "substr(a, 1, 2)")]
#[case("left(a,2)", MySQL, "left(a, 2)")]
//#[case("left(a,2)", SQLite, "substr(a, 1, 2)")]
fn test_left(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("right(a,2)", NSQL, "right(a, 2)")]
#[case("right(a,2)", PostgreSQL, "right(a, 2)")]
#[case("right('abc',2)", PostgreSQL, "right('abc', 2)")]

//#[case("right(a,2)", Oracle, "substr(a, length(a) - 1, 2)")]
//#[case("right('abc',2)", Oracle, "substr('abc', length('abc') - 1, 2)")]
//#[case("right('abc',1)", Oracle, "substr('abc', length('abc'), 1)")]

//#[case("right(a,2)", SqlServer, "right(a, 2)")]
//#[case("right('abc',2)", SqlServer, "right('abc', 2)")]

//#[case("right('abc',2)", SQLite, "substr('abc', length('abc') - 1, 2)")]
//#[case("right('abc',1)", SQLite, "substr('abc', length('abc'), 1)")]
fn test_right(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

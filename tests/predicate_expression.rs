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
//#[case("a between   3 and 10", NSQL, "a between 3 and 10")]
fn test(left: &str, database_type: DatabaseType, right: &str) {
    test_predicate(database_type, left, right)
}

#[theory]
#[test]
#[case("a = 3 And c is null", NSQL, "a = 3 and c is null")]
#[case("a = 3 or c < 5", NSQL, "a = 3 or c < 5")]
#[case("a =3   and (c < 5 or c= 7)", NSQL, "a = 3 and (c < 5 or c = 7)")]
#[case("(a =3 )  and ((c < 5 or c= 7))", NSQL, "a = 3 and (c < 5 or c = 7)")]
#[case("(a =3 )  and (((c < 5 or c= 7)))", NSQL, "a = 3 and (c < 5 or c = 7)")]
#[case("(a = 3 or c < 5) and d = 8", NSQL, "(a = 3 or c < 5) and d = 8")]
#[case("a = 3 and (c < 5 or d = 8)", NSQL, "a = 3 and (c < 5 or d = 8)")]

//#[case("a between   3 and 10", NSQL, "a between 3 and 10")]
fn test_logical(left: &str, database_type: DatabaseType, right: &str) {
    test_predicate(database_type, left, right)
}

#[theory]
#[test]
#[case("a in ('x',     'y',      'z')", NSQL, "a in ('x', 'y', 'z')")]
#[case("a in (1,     3,      5)", NSQL, "a in (1, 3, 5)")]
#[case("a in ('2')", NSQL, "a in ('2')")]
#[case(
    "a in (select name from student)",
    NSQL,
    "a in (select name from student)"
)]
#[case("a not in ('x',     'y',      'z')", NSQL, "a not in ('x', 'y', 'z')")]

fn test_in(left: &str, database_type: DatabaseType, right: &str) {
    test_predicate(database_type, left, right)
}

#[theory]
#[test]
#[case("nOt(a = 'abc')", NSQL, "not(a = 'abc')")]
#[case("nOt(a = 'abc' and age <3)", NSQL, "not(a = 'abc' and age < 3)")]
//#[case("not a = 'abc'", NSQL, "not a = 'abc'")]
//#[case("not a > 3 or a < 7", NSQL, "not a > 3 or a < 7")]
//#[case("not (a > 3 or a < 7)", NSQL, "not(a > 3 or a < 7)")]

//#[case("a between   3 and 10", NSQL, "a between 3 and 10")]
fn test_not(left: &str, database_type: DatabaseType, right: &str) {
    test_predicate(database_type, left, right)
}

#[theory]
#[test]
#[case("a is null", NSQL, "a is null")]
#[case("a is not null", NSQL, "a is not null")]
fn test_null(left: &str, database_type: DatabaseType, right: &str) {
    test_predicate(database_type, left, right)
}

#[theory]
#[test]
#[case("a = c", NSQL, "a = c")]
#[case("a = 'abc'", NSQL, "a = 'abc'")]
#[case("a != 'abc'", NSQL, "a <> 'abc'")]
#[case("a ~= 'abc'", NSQL, "a <> 'abc'")]
#[case("a ^= 'abc'", NSQL, "a <> 'abc'")]
#[case("a <> 'abc'", NSQL, "a <> 'abc'")]
#[case("a <3", NSQL, "a < 3")]
#[case("a <-3", NSQL, "a < -3")]
#[case("a < 36.266", NSQL, "a < 36.266")]
#[case("a>3", NSQL, "a > 3")]
#[case("a>:abc", NSQL, "a > :abc")]
#[case("我的字段>3", NSQL, "我的字段 > 3")]
fn test_comparison(left: &str, database_type: DatabaseType, right: &str) {
    test_predicate(database_type, left, right)
}

#[theory]
#[test]
//#[case("a like '%c'", NSQL, "a like '%c'")]
fn test_like(left: &str, database_type: DatabaseType, right: &str) {
    test_predicate(database_type, left, right)
}

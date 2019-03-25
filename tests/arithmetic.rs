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
#[case("a     +3", NSQL, "a + 3")]
#[case("a  - 3", NSQL, "a - 3")]
#[case("a* 3", NSQL, "a * 3")]
#[case("a/ 3", NSQL, "a / 3")]
#[case("a/ (5+b)", NSQL, "a / (5 + b)")]
#[case("(a*2) / (3 + b)", NSQL, "a * 2 / (3 + b)")]
#[case("a/ 3 + 3", NSQL, "a / 3 + 3")]
#[case("a/ (3 + 3)", NSQL, "a / (3 + 3)")]
#[case("(a)/ ((3 + 3))", NSQL, "a / (3 + 3)")]
#[case("(a)/ ((m + n))", NSQL, "a / (m + n)")]
#[case("((a * 15))/ (((5+b)))", NSQL, "a * 15 / (5 + b)")]
#[case("(a*15)/(5+b)+6*7", NSQL, "a * 15 / (5 + b) + 6 * 7")]
#[case("length(name) - 7 + 5", NSQL, "length(name) - 7 + 5")]
fn test(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("x = 3+4", NSQL, "x = 3 + 4")]
fn test1(left: &str, database_type: DatabaseType, right: &str) {
    test_predicate(database_type, left, right);
}

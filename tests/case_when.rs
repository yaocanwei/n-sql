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
#[case(
    "case   a when 1 then '甲' when 2 then '乙' End",
    NSQL,
    "case a when 1 then '甲' when 2 then '乙' end"
)]
#[case(
    "case a when 1 then '甲' when 2 then '乙' ELSE '其他' End",
    NSQL,
    "case a when 1 then '甲' when 2 then '乙' else '其他' end"
)]
#[case("CASE   when 100 >= score and score > 85 then '优' when 85 >= score and score > 70 then '良' when score < 60 then '不及格' else '未知' End", NSQL, "case when 100 >= score and score > 85 then '优' when 85 >= score and score > 70 then '良' when score < 60 then '不及格' else '未知' end")]
#[case("CASE   when 100 >= score and score > 85 then '优' when 85 >= score and score > 70 then '良' when score < 60 then '不及格' End", NSQL, "case when 100 >= score and score > 85 then '优' when 85 >= score and score > 70 then '良' when score < 60 then '不及格' end")]
fn test(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

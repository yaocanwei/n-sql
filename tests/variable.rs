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

#[test]
fn test1() {
    test_predicate(NSQL, "a = :  m", "a = :m");
}

#[test]
fn test2() {
    test_statement(
        NSQL,
        "select * from student where a in (:a,:b,:c)",
        "select * from student where a in (:a, :b, :c)",
    );
}

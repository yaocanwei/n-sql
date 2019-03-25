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
    "select * from student skip 1 limit 2",
    NSQL,
    "select * from student skip 1 limit 2"
)]
#[case("select * from student skip 1", NSQL, "select * from student skip 1")]
#[case("select * from student limit 2", NSQL, "select * from student limit 2")]
#[case(
    "select * from student skip 1 limit 2",
    MySQL,
    "select * from student limit 2, 1"
)]
#[case(
    "select * from student skip 1 limit 2",
    Oracle,
    "select * from student offset 1 row fetch first 2 rows only"
)]
#[case(
    "select * from student skip 2 limit 1",
    Oracle,
    "select * from student offset 2 rows fetch first 1 row only"
)]
#[case(
    "select * from student skip 1 limit 2",
    PostgreSQL,
    "select * from student offset 1 limit 2"
)]
#[case(
    "select * from student skip 1",
    PostgreSQL,
    "select * from student offset 1"
)]
#[case(
    "select * from student limit 2",
    PostgreSQL,
    "select * from student limit 2"
)]
fn pagination(left: &str, database_type: DatabaseType, right: &str) {
    test_statement(database_type, left, right);
}

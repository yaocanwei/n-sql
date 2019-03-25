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
//#[case("variance(a)", NSQL, "variance(a)")]

fn test(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("stddev(a)", NSQL, "stddev(a)")]
#[case("STddev(a)", NSQL, "stddev(a)")]
#[case("stddevif(gender = '男',a)", NSQL, "stddevif(gender = '男', a)")]
#[case(
    "stddevif(gender = '男',a)",
    PostgreSQL,
    "stddev(case when gender = '男' then a else null end)"
)]

fn test_stddev(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("median(abc)", NSQL, "median(abc)")]
//#[case("median(abc)", PostgreSQL, "percentile_cont(0.5) within group (order by abc)")]
#[case("median(abc)", Oracle, "median(abc)")]
//#[case("median(abc)", MySQL, "median(abc)")]
//#[case("median(abc)", SqlServer, "percentile_cont(0.5) within group (order by abc) over (partition by 0)")]
//#[case("median(abc)", SQLite, "median(abc)")]
fn test_median(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("sum(all a)", NSQL, "sum(all a)")]
#[case("sum(a)", NSQL, "sum(a)")]
#[case("sumif(gender = '男',all a)", NSQL, "sumif(gender = '男', all a)")]
#[case("sumif(gender = '男',a)", NSQL, "sumif(gender = '男', a)")]
#[case(
    "sumif(gender = '男',all a)",
    PostgreSQL,
    "sum(all case when gender = '男' then a else null end)"
)]
#[case(
    "sumif(gender = '男',a)",
    PostgreSQL,
    "sum(case when gender = '男' then a else null end)"
)]

fn test_sum(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("count(a)", NSQL, "count(a)")]
#[case("count(unique a)", NSQL, "count(unique a)")]
#[case("count(all a)", NSQL, "count(all a)")]
#[case("countif(gender = '男',a)", NSQL, "countif(gender = '男', a)")]
#[case(
    "countif(gender = '男',unique a)",
    NSQL,
    "countif(gender = '男', unique a)"
)]
#[case(
    "countif(gender = '男',a)",
    PostgreSQL,
    "count(case when gender = '男' then a else null end)"
)]
#[case(
    "countif(gender = '男',unique a)",
    PostgreSQL,
    "count(unique case when gender = '男' then a else null end)"
)]
fn test_count(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("avg(a)", NSQL, "avg(a)")]
#[case("avg(distinct a)", NSQL, "avg(distinct a)")]
#[case("avgif(gender = '男', a)", NSQL, "avgif(gender = '男', a)")]
#[case(
    "avgif(gender = '男',distinct a)",
    NSQL,
    "avgif(gender = '男', distinct a)"
)]
#[case(
    "avgif(gender = '男', a)",
    PostgreSQL,
    "avg(case when gender = '男' then a else null end)"
)]
#[case(
    "avgif(gender = '男',distinct a)",
    PostgreSQL,
    "avg(distinct case when gender = '男' then a else null end)"
)]
fn test_avg(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("percentile(age, 0.25)", NSQL, "percentile(age, 0.25)")]
#[case("percentile(age, 0.25, asc)", NSQL, "percentile(age, 0.25, asc)")]
#[case("percentile(age, 0.25, desc)", NSQL, "percentile(age, 0.25, desc)")]
#[case("percent(age, 0.25)", NSQL, "percentile(age, 0.25)")]
#[case("percent(age, 0.25, asc)", NSQL, "percentile(age, 0.25, asc)")]
#[case("percent(age, 0.25, desc)", NSQL, "percentile(age, 0.25, desc)")]
#[case("percentile_cont(age, 0.25)", NSQL, "percentile(age, 0.25)")]
#[case("percentile_cont(age, 0.25, asc)", NSQL, "percentile(age, 0.25, asc)")]
#[case(
    "percentile_cont(age, 0.25, desc)",
    NSQL,
    "percentile(age, 0.25, desc)"
)]
#[case(
    "percentile_cont(0.25) within group (order by age)",
    NSQL,
    "percentile(age, 0.25)"
)]
#[case(
    "percentile_cont(0.25) within group (order by age asc)",
    NSQL,
    "percentile(age, 0.25, asc)"
)]
#[case(
    "percentile_cont(0.25) within group (order by age desc)",
    NSQL,
    "percentile(age, 0.25, desc)"
)]
#[case("percentile_disc(age, 0.25)", NSQL, "percentile_disc(age, 0.25)")]
#[case(
    "percentile_disc(age, 0.25, asc)",
    NSQL,
    "percentile_disc(age, 0.25, asc)"
)]
#[case(
    "percentile_disc(age, 0.25, desc)",
    NSQL,
    "percentile_disc(age, 0.25, desc)"
)]
#[case(
    "percentile_disc(0.25) within group (order by age)",
    NSQL,
    "percentile_disc(age, 0.25)"
)]
#[case(
    "percentile_disc(0.25) within group (order by age asc)",
    NSQL,
    "percentile_disc(age, 0.25, asc)"
)]
#[case(
    "percentile_disc(0.25) within group (order by age desc)",
    NSQL,
    "percentile_disc(age, 0.25, desc)"
)]
//
//
//
//
#[case(
    "percentile(age, 0.25)",
    PostgreSQL,
    "percentile_cont(0.25) within group (order by age)"
)]
#[case(
    "percentile(age, 0.25, asc)",
    PostgreSQL,
    "percentile_cont(0.25) within group (order by age asc)"
)]
#[case(
    "percentile(age, 0.25, desc)",
    PostgreSQL,
    "percentile_cont(0.25) within group (order by age desc)"
)]
#[case(
    "percent(age, 0.25)",
    PostgreSQL,
    "percentile_cont(0.25) within group (order by age)"
)]
#[case(
    "percent(age, 0.25, asc)",
    PostgreSQL,
    "percentile_cont(0.25) within group (order by age asc)"
)]
#[case(
    "percent(age, 0.25, desc)",
    PostgreSQL,
    "percentile_cont(0.25) within group (order by age desc)"
)]
#[case(
    "percentile_cont(age, 0.25)",
    PostgreSQL,
    "percentile_cont(0.25) within group (order by age)"
)]
#[case(
    "percentile_cont(age, 0.25, asc)",
    PostgreSQL,
    "percentile_cont(0.25) within group (order by age asc)"
)]
#[case(
    "percentile_cont(age, 0.25, desc)",
    PostgreSQL,
    "percentile_cont(0.25) within group (order by age desc)"
)]
#[case(
    "percentile_cont(0.25) within group (order by age)",
    PostgreSQL,
    "percentile_cont(0.25) within group (order by age)"
)]
#[case(
    "percentile_cont(0.25) within group (order by age asc)",
    PostgreSQL,
    "percentile_cont(0.25) within group (order by age asc)"
)]
#[case(
    "percentile_cont(0.25) within group (order by age desc)",
    PostgreSQL,
    "percentile_cont(0.25) within group (order by age desc)"
)]
#[case(
    "percentile_disc(age, 0.25)",
    PostgreSQL,
    "percentile_disc(0.25) within group (order by age)"
)]
#[case(
    "percentile_disc(age, 0.25, asc)",
    PostgreSQL,
    "percentile_disc(0.25) within group (order by age asc)"
)]
#[case(
    "percentile_disc(age, 0.25, desc)",
    PostgreSQL,
    "percentile_disc(0.25) within group (order by age desc)"
)]
#[case(
    "percentile_disc(0.25) within group (order by age)",
    PostgreSQL,
    "percentile_disc(0.25) within group (order by age)"
)]
#[case(
    "percentile_disc(0.25) within group (order by age asc)",
    PostgreSQL,
    "percentile_disc(0.25) within group (order by age asc)"
)]
#[case(
    "percentile_disc(0.25) within group (order by age desc)",
    PostgreSQL,
    "percentile_disc(0.25) within group (order by age desc)"
)]
#[case(
    "percentile(age, 0.25)",
    Oracle,
    "percentile_cont(0.25) within group (order by age)"
)]
#[case(
    "percentile(age, 0.25, asc)",
    Oracle,
    "percentile_cont(0.25) within group (order by age asc)"
)]
#[case(
    "percentile(age, 0.25, desc)",
    Oracle,
    "percentile_cont(0.25) within group (order by age desc)"
)]
#[case(
    "percent(age, 0.25)",
    Oracle,
    "percentile_cont(0.25) within group (order by age)"
)]
#[case(
    "percent(age, 0.25, asc)",
    Oracle,
    "percentile_cont(0.25) within group (order by age asc)"
)]
#[case(
    "percent(age, 0.25, desc)",
    Oracle,
    "percentile_cont(0.25) within group (order by age desc)"
)]
#[case(
    "percentile_cont(age, 0.25)",
    Oracle,
    "percentile_cont(0.25) within group (order by age)"
)]
#[case(
    "percentile_cont(age, 0.25, asc)",
    Oracle,
    "percentile_cont(0.25) within group (order by age asc)"
)]
#[case(
    "percentile_cont(age, 0.25, desc)",
    Oracle,
    "percentile_cont(0.25) within group (order by age desc)"
)]
#[case(
    "percentile_cont(0.25) within group (order by age)",
    Oracle,
    "percentile_cont(0.25) within group (order by age)"
)]
#[case(
    "percentile_cont(0.25) within group (order by age asc)",
    Oracle,
    "percentile_cont(0.25) within group (order by age asc)"
)]
#[case(
    "percentile_cont(0.25) within group (order by age desc)",
    Oracle,
    "percentile_cont(0.25) within group (order by age desc)"
)]
#[case(
    "percentile_disc(age, 0.25)",
    Oracle,
    "percentile_disc(0.25) within group (order by age)"
)]
#[case(
    "percentile_disc(age, 0.25, asc)",
    Oracle,
    "percentile_disc(0.25) within group (order by age asc)"
)]
#[case(
    "percentile_disc(age, 0.25, desc)",
    Oracle,
    "percentile_disc(0.25) within group (order by age desc)"
)]
#[case(
    "percentile_disc(0.25) within group (order by age)",
    Oracle,
    "percentile_disc(0.25) within group (order by age)"
)]
#[case(
    "percentile_disc(0.25) within group (order by age asc)",
    Oracle,
    "percentile_disc(0.25) within group (order by age asc)"
)]
#[case(
    "percentile_disc(0.25) within group (order by age desc)",
    Oracle,
    "percentile_disc(0.25) within group (order by age desc)"
)]
//
//
#[case(
    "percentile(age, 0.25)",
    SqlServer,
    "percentile_cont(0.25) within group (order by age) over (partition by 0)"
)]
#[case(
    "percentile(age, 0.25, asc)",
    SqlServer,
    "percentile_cont(0.25) within group (order by age asc) over (partition by 0)"
)]
#[case(
    "percentile(age, 0.25, desc)",
    SqlServer,
    "percentile_cont(0.25) within group (order by age desc) over (partition by 0)"
)]
#[case(
    "percent(age, 0.25)",
    SqlServer,
    "percentile_cont(0.25) within group (order by age) over (partition by 0)"
)]
#[case(
    "percent(age, 0.25, asc)",
    SqlServer,
    "percentile_cont(0.25) within group (order by age asc) over (partition by 0)"
)]
#[case(
    "percent(age, 0.25, desc)",
    SqlServer,
    "percentile_cont(0.25) within group (order by age desc) over (partition by 0)"
)]
#[case(
    "percentile_cont(age, 0.25)",
    SqlServer,
    "percentile_cont(0.25) within group (order by age) over (partition by 0)"
)]
#[case(
    "percentile_cont(age, 0.25, asc)",
    SqlServer,
    "percentile_cont(0.25) within group (order by age asc) over (partition by 0)"
)]
#[case(
    "percentile_cont(age, 0.25, desc)",
    SqlServer,
    "percentile_cont(0.25) within group (order by age desc) over (partition by 0)"
)]
#[case(
    "percentile_cont(0.25) within group (order by age)",
    SqlServer,
    "percentile_cont(0.25) within group (order by age) over (partition by 0)"
)]
#[case(
    "percentile_cont(0.25) within group (order by age asc)",
    SqlServer,
    "percentile_cont(0.25) within group (order by age asc) over (partition by 0)"
)]
#[case(
    "percentile_cont(0.25) within group (order by age desc)",
    SqlServer,
    "percentile_cont(0.25) within group (order by age desc) over (partition by 0)"
)]
#[case(
    "percentile_disc(age, 0.25)",
    SqlServer,
    "percentile_disc(0.25) within group (order by age) over (partition by 0)"
)]
#[case(
    "percentile_disc(age, 0.25, asc)",
    SqlServer,
    "percentile_disc(0.25) within group (order by age asc) over (partition by 0)"
)]
#[case(
    "percentile_disc(age, 0.25, desc)",
    SqlServer,
    "percentile_disc(0.25) within group (order by age desc) over (partition by 0)"
)]
#[case(
    "percentile_disc(0.25) within group (order by age)",
    SqlServer,
    "percentile_disc(0.25) within group (order by age) over (partition by 0)"
)]
#[case(
    "percentile_disc(0.25) within group (order by age asc)",
    SqlServer,
    "percentile_disc(0.25) within group (order by age asc) over (partition by 0)"
)]
#[case(
    "percentile_disc(0.25) within group (order by age desc)",
    SqlServer,
    "percentile_disc(0.25) within group (order by age desc) over (partition by 0)"
)]

fn test_percentile(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[test]
fn test_avg2() {
    test_predicate(NSQL, "avg(a) >9", "avg(a) > 9");
}

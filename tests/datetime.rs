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
#[case("day(now())", NSQL, "day(now())")]
#[case("extract(day from now())", NSQL, "day(now())")]
#[case("day(now())", PostgreSQL, "extract(day from now())")]
#[case("day(now())", Oracle, "extract(day from systimestamp)")]
#[case("day(now())", MySQL, "day(now())")]
//#[case("day(now())", SqlServer, "day(getdate())")]
//#[case("day(now())", SQLite, "")]
fn test_day(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("day_add(now(),3)", NSQL, "day_add(now(), 3)")]
#[case("day_add(now(),3)", PostgreSQL, "now() + interval '3' day")]
#[case("day_add(now(),3+9*7)", NSQL, "day_add(now(), 3 + 9 * 7)")]
#[case("day_add(now(),3+9*7)", PostgreSQL, "now() + interval '66' day")]
//#[case("day_add(now(),3+9*7)", Oracle, "systimestamp + interval '66' day")]
//#[case("day_add(now(),3+9*7)", MySQL, "date_add(now(), interval 66 day)")]
//#[case("day_add(now(),3+9*7)", MySQL, "dateadd(day, 66, getdate()")]
//#[case("day_add(now(),3+9*7)", SQLite, "Datetime(Current_Timestamp, '+66 Day')")]
fn test_day_add(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("day_sub(now(),3)", NSQL, "day_sub(now(), 3)")]
#[case("day_sub(now(),3)", PostgreSQL, "now() - interval '3' day")]
#[case("day_sub(create_date,3)", NSQL, "day_sub(create_date, 3)")]
//#[case("day_sub(now(),3)", Oracle, "systimestamp - interval '3' day")]
//#[case("day_sub(now(),3)", MySQL, "date_sub(now(), interval 3 day)")]
//#[case("day_sub(now(),3)", MySQL, "dateadd(day, -3, getdate()")]
//#[case("day_sub(now(),3)", SQLite, "Datetime(Current_Timestamp, '-3 Day')")]
fn test_day_sub(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("day_diff(a, b)", NSQL, "day_diff(a, b)")]
//#[case("day_diff(a, b)", PostgreSQL, "extract(epoch from (b - a)) / 86400")]
//#[case("day_diff(a, b)", Oracle, "b - a")]
//#[case("day_diff(a, b)", MySQL, "timestampdiff(day, a, b)")]
//#[case("day_diff(a, b)", SqlServer, "datediff(day, a, b)")]
//#[case("day_diff(a, b)", SQLite, "julianday(b) - julianday(a)")]
fn test_day_diff(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("HOUR_add(now(),3)", NSQL, "hour_add(now(), 3)")]
#[case("hour_add(now(),3)", PostgreSQL, "now() + interval '3' hour")]
//#[case("hour_add(now(),3)", Oracle, "systimestamp + interval '3' hour")]
//#[case("hour_add(now(),3)", MySQL, "date_add(now(), interval 3 hour)")]
//#[case("hour_add(now(),3)", SqlServer, "dateadd(hour, 3, getdate())")]
//#[case("hour_add(now(),3)", SQLite, "datetime(current_timestamp, '+3 hour')")]
fn test_hour_add(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("HOUR_sub(now(),3)", NSQL, "hour_sub(now(), 3)")]
#[case("HOUR_sub(now(),3)", PostgreSQL, "now() - interval '3' hour")]
//#[case("HOUR_sub(now(),3)", Oracle, "systimestamp - interval '3' hour")]
//#[case("HOUR_sub(now(),3)", MySQL, "date_sub(now(), interval 3 hour)")]
//#[case("HOUR_sub(now(),3)", SqlServer, "dateadd(hour, 3, getdate())")]
//#[case("HOUR_sub(now(),3)", SQLite, "datetime(current_timestamp, '-1 hour')")]
fn test_hour_sub(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("HOUR(now())", NSQL, "hour(now())")]
#[case("HOUR(now())", PostgreSQL, "extract(hour from now())")]
#[case("extract(hour from now())", NSQL, "hour(now())")]
#[case("extract(hour from now())", PostgreSQL, "extract(hour from now())")]
//#[case("extract(hour from now())", Oracle, "extract(hour from systimestamp)")]
//#[case("extract(hour from now())", MySQL, "hour(now())")]
//#[case("extract(hour from now())", SqlServer, "datepart(hour, getdate())")]
//#[case("extract(hour from now())", SQLite, "")]
fn test_hour(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("hour_diff(a,b)", NSQL, "hour_diff(a, b)")]
//#[case("hour_diff(a,b)", PostgreSQL, "extract(epoch from (b - a)) / 3600")]
//#[case("hour_diff(a,b)", Oracle, "(b - a) * 24")]
//#[case("hour_diff(a,b)", MySQL, "timestampdiff(hour, a, b)")]
//#[case("hour_diff(a,b)", SqlServer, "datediff(hour, a, b)")]
//#[case("hour_diff(a,b)", SQLite, "(julianday(b) - julianday(a)) * 24")]
fn test_hour_diff(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("minute_add(now(),3)", NSQL, "minute_add(now(), 3)")]
#[case("MINUTE_ADD(now(),3)", NSQL, "minute_add(now(), 3)")]
#[case("minute_add(now(),3)", PostgreSQL, "now() + interval '3' minute")]
//#[case("minute_add(now(),3)", Oracle, "systimestamp + interval '3' minute")]
//#[case("minute_add(now(),3)", MySQL, "date_add(now(), interval 3 minute)")]
//#[case("minute_add(now(),3)", SqlServer, "dateadd(minute, 3, getdate())")]
//#[case("minute_add(now(),3)", SQLite, "datetime(current_timestamp, '+3 minute')")]
fn test_minute_add(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("minute_sub(now(),3)", NSQL, "minute_sub(now(), 3)")]
#[case("minute_sub(now(),3)", PostgreSQL, "now() - interval '3' minute")]
//#[case("minute_sub(now(),3)", Oracle, "systimestamp - interval '3' minute")]
//#[case("minute_sub(now(),3)", MySQL, "date_sub(now(), interval 3 minute)")]
//#[case("minute_sub(now(),3)", SqlServer, "dateadd(minute, -3, getdate())")]
//#[case("minute_sub(now(),3)", SQLite, "datetime(current_timestamp, '-3 minute')")]
fn test_minute_sub(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("minute(now())", NSQL, "minute(now())")]
#[case("minute(now())", PostgreSQL, "extract(minute from now())")]
#[case("extract(minute from now())", NSQL, "minute(now())")]
#[case("extract(minute from now())", PostgreSQL, "extract(minute from now())")]
#[case(
    "extract(minute from now())",
    Oracle,
    "extract(minute from systimestamp)"
)]
#[case("extract(minute from now())", MySQL, "minute(now())")]
//#[case("extract(minute from now())", SqlServer, "datepart(minute, getdate())")]
//#[case("extract(minute from now())", SQLite, "")]
fn test_minute(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("minute_diff(a,b)", NSQL, "minute_diff(a, b)")]
//#[case("minute_diff(a,b)", PostgreSQL, "extract(epoch from (b - a)) / 60")]
//#[case("minute_diff(a,b)", Oracle, "(b - a) * 1440")]
//#[case("minute_diff(a,b)", MySQL, "timestampdiff(minute, a, b)")]
//#[case("minute_diff(a,b)", SqlServer, "datediff(minute, a, b)")]
//#[case("minute_diff(a,b)", SQLite, "(julianday(b) - julianday(a)) * 1440")]
fn test_minute_diff(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("month_add(now(),3)", NSQL, "month_add(now(), 3)")]
#[case("month_add(now(),3)", PostgreSQL, "now() + interval '3' month")]
//#[case("month_add(now(),3)", Oracle, "systimestamp + interval '3' month")]
//#[case("month_add(now(),3)", MySQL, "date_add(now(), interval 3 month)")]
//#[case("month_add(now(),3)", SqlServer, "dateadd(month, 3, getdate())")]
//#[case("month_add(now(),3)", SQLite, "datetime(current_timestamp, '-3 Second')")]
fn test_month_add(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("month_sub(now(),3)", NSQL, "month_sub(now(), 3)")]
#[case("month_sub(now(),3)", PostgreSQL, "now() - interval '3' month")]
//#[case("month_sub(now(),3)", Oracle, "systimestamp - interval '3' month")]
//#[case("month_sub(now(),3)", MySQL, "date_sub(now(), interval 3 month)")]
//#[case("month_sub(now(),3)", SqlServer, "dateadd(month, -3, getdate())")]
//#[case("month_sub(now(),3)", SQLite, "datetime(current_timestamp, '-3 month')")]
fn test_month_sub(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("month(now())", NSQL, "month(now())")]
#[case("month(now())", PostgreSQL, "extract(month from now())")]
//#[case("month(now())", Oracle, "extract(month from systimestamp)")]
#[case("extract(month from now())", NSQL, "month(now())")]
#[case("extract(month from now())", PostgreSQL, "extract(month from now())")]
//#[case("extract(month from now())", Oracle, "extract(month from systimestamp)")]
//#[case("extract(month from now())", MySQL, "month(now())")]
//#[case("extract(month from now())", SqlServer, "month(getedate())")]
//#[case("extract(month from now())", SQLite, "")]
fn test_month(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("month_diff(a,b)", NSQL, "month_diff(a, b)")]
//#[case("month_diff(a,b)", PostgreSQL, "extract(epoch from (b - a)) / 2592000")]
//#[case("month_diff(a,b)", Oracle, "(b - a) / 30")]
//#[case("month_diff(a,b)", MySQL, "timestampdiff(month, a, b)")]
//#[case("month_diff(a,b)", SqlServer, "datediff(month, a, b)")]
//#[case("month_diff(a,b)", SQLite, "(julianday(b) - julianday(a)) / 30")]
fn test_month_diff(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[case("second_add(now(),3)", NSQL, "second_add(now(), 3)")]
#[case("second_add(now(),3)", PostgreSQL, "now() + interval '3' second")]
//#[case("second_add(now(),3)", Oracle, "systimestamp + interval '3' second")]
//#[case("second_add(now(),3)", MySQL, "date_add(now(), interval 3 second)")]
//#[case("second_add(now(),3)", SqlServer, "dateadd(second, 3, getdate())")]
//#[case("second_add(now(),3)", SQLite, "datetime(current_timestamp, '+3 second')")]
fn test_second_add(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("second_sub(now(),3)", NSQL, "second_sub(now(), 3)")]
#[case("second_sub(now(),3)", PostgreSQL, "now() - interval '3' second")]
//#[case("second_sub(now(),3)", Oracle, "systimestamp - interval '3' second")]
//#[case("second_sub(now(),3)", MySQL, "date_sub(now(), interval 3 second)")]
//#[case("second_sub(now(),3)", SqlServer, "dateadd(second, -3, getdate())")]
//#[case("second_sub(now(),3)", SQLite, "datetime(current_timestamp, '-3 second')")]
fn test_second_sub(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("second(now())", NSQL, "second(now())")]
#[case("second(now())", PostgreSQL, "extract(second from now())")]
#[case("extract(second from now())", NSQL, "second(now())")]
#[case("extract(second from now())", PostgreSQL, "extract(second from now())")]
//#[case("extract(second from now())", MySQL, "second(now())")]
//#[case("extract(second from now())", SqlServer, "datepart(second, getdate())")]
//#[case("extract(second from now())", SQLite, "")]
fn test_second(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("second_diff(a, b)", NSQL, "second_diff(a, b)")]
//#[case("second_diff(a, b)", PostgreSQL, "extract(epoch from (b - a))")]
//#[case("second_diff(a, b)", Oracle, "(b - a) * 86400")]
//#[case("second_diff(a, b)", MySQL, "timestampdiff(second, a, b)")]
//#[case("second_diff(a, b)", SqlServer, "datediff(second, a, b)")]
//#[case("second_diff(a, b)", SQLite, "(julianday(b) - julianday(a)) * 86400")]
fn test_second_diff(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("year_add(now(),3)", NSQL, "year_add(now(), 3)")]
#[case("year_add(now(),3)", PostgreSQL, "now() + interval '3' year")]
//#[case("year_add(now(),3)", Oracle, "systimestamp + interval '3' year")]
//#[case("year_add(now(),3)", MySQL, "date_add(now(), interval 3 year)")]
//#[case("year_add(now(),3)", SqlServer, "date_add(year, 3, getdate())")]
//#[case("year_add(now(),3)", SQLite, "datetime(current_timestamp, '-3 second')")]
fn test_year_add(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("year_sub(now(),3)", NSQL, "year_sub(now(), 3)")]
#[case("year_sub(now(),3)", PostgreSQL, "now() - interval '3' year")]
//#[case("year_sub(now(),3)", Oracle, "systimestamp - interval '3' year")]
//#[case("year_sub(now(),3)", MySQL, "date_sub(now(), interval 3 year)")]
//#[case("year_sub(now(),3)", SqlServer, "dateadd(year, -3, getdate())")]
//#[case("year_sub(now(),3)", SQLite, "datetime(current_timestamp, '-3 year')")]
fn test_year_sub(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("year(now())", NSQL, "year(now())")]
#[case("year(now())", PostgreSQL, "extract(year from now())")]
//#[case("year(now())", Oracle, "extract(year from systimestamp)")]
#[case("extract(year from abc)", NSQL, "year(abc)")]
#[case("extract(year from abc)", PostgreSQL, "extract(year from abc)")]
//#[case("year(now())", SqlServer, "year(getdate())")]
//#[case("year(now())", SQLite, "")]
fn test_year(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("year_diff(a,b)", NSQL, "year_diff(a, b)")]
//#[case("year_diff(a,b)", PostgreSQL, "extract(epoch from (b - a)) / 31536000")]
//#[case("year_diff(a,b)", Oracle, "(b - a) / 365")]
//#[case("year_diff(a,b)", MySQL, "timestampdiff(year, a, b)")]
//#[case("year_diff(a,b)", SqlServer, "datediff(year, a, b)")]
//#[case("year_diff(a,b)", SQLite, "(julianday(b) - julianday(a)) / 365")]
fn test_year_diff(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[test]
fn test_year_err() {
    let expr = ExpressionParser::new().parse(Lexer::new("extract(year form abc)").tokenizer());
    assert_eq!(true, expr.is_err());
}

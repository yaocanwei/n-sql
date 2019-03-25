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
#[case("nvl(a, 'no value')", NSQL, "nvl(a, 'no value')")]
#[case("nvl(a, 'no value')", MySQL, "ifnull(a, 'no value')")]
#[case("nvl(a, 'no value')", PostgreSQL, "coalesce(a, 'no value')")]
#[case("NVL(a, 'no value')", NSQL, "nvl(a, 'no value')")]
#[case("NVL(a,      123)", NSQL, "nvl(a, 123)")]
#[case("NVL(a,      -123)", NSQL, "nvl(a, -123)")]
#[case("nvl (abc, 'none')", SqlServer, "coalesce(abc, 'none')")]
#[case("nvl (abc, 'none')", SQLite, "ifnull(abc, 'none')")]

fn test_nvl(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("COALESCE(a,b,c)", NSQL, "coalesce(a, b, c)")]
#[case("COALESCE(a,b,c)", PostgreSQL, "coalesce(a, b, c)")]
#[case("COALESCE(a,b,c)", Oracle, "coalesce(a, b, c)")]
#[case("COALESCE(a,b,c)", MySQL, "coalesce(a, b, c)")]
#[case("COALESCE(a,b,c)", SqlServer, "coalesce(a, b, c)")]
#[case("COALESCE(a,b,c)", SQLite, "coalesce(a, b, c)")]
fn test_coalesce(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
#[case("cast(a as text)", NSQL, "a::text")]
#[case("cast(1 as text)", NSQL, "1::text")]
#[case("cast('1' as int)", NSQL, "'1'::int")]
#[case("a::text", NSQL, "a::text")]
#[case("abc:: text", NSQL, "abc::text")]
#[case("abc:: text", PostgreSQL, "abc::text")]
#[case("abc:: text", Oracle, "to_char(abc)")]
#[case("abc:: text", MySQL, "convert(abc, char)")]
#[case("abc:: text", SqlServer, "convert(varchar(8000), abc)")]
#[case("abc:: text", SQLite, "cast(abc as text)")]
#[case("abc::int", NSQL, "abc::int")]
#[case("abc::int", PostgreSQL, "abc::int")]
#[case("abc::int", Oracle, "cast(abc as int)")]
#[case("abc::int", MySQL, "convert(abc, int)")]
#[case("abc::int", SqlServer, "cast(abc as int)")]
#[case("abc::int", SQLite, "cast(abc as int)")]
#[case("abc::float", NSQL, "abc::float")]
#[case("abc::float", PostgreSQL, "abc::float")]
#[case("abc::float", Oracle, "cast(abc as float)")]
#[case("abc::float", MySQL, "convert(abc, decimal(65, 38))")]
#[case("abc::float", SqlServer, "cast(abc as float)")]
#[case("abc::float", SQLite, "cast(abc as float)")]
#[case("abc::numeric", NSQL, "abc::numeric")]
#[case("abc::numeric", PostgreSQL, "abc::numeric")]
#[case("abc::numeric", Oracle, "cast(abc as numeric)")]
#[case("abc::numeric", MySQL, "convert(abc, decimal(65, 38))")]
#[case("abc::numeric", SqlServer, "convert(numeric(38, 19), abc)")]
#[case("abc::numeric", SQLite, "cast(abc as numeric)")]
#[case("abc::date", NSQL, "abc::date")]
#[case("abc::date", PostgreSQL, "abc::date")]
#[case("abc::date", Oracle, "to_date(abc, 'yyyy-mm-dd')")]
#[case("abc::date", MySQL, "convert(abc, date)")]
#[case("abc::date", SqlServer, "convert(date, abc)")]
#[case("abc::date", SQLite, "date(abc)")]
#[case("abc::datetime", NSQL, "abc::datetime")]
#[case("abc::datetime", PostgreSQL, "abc::datetime")]
#[case("abc::datetime", Oracle, "to_timestamp(abc, 'yyyy-mm-dd hh24:mi:ss')")]
#[case("abc::datetime", MySQL, "convert(abc, datetime)")]
#[case("abc::datetime", SqlServer, "convert(datetime, abc)")]
#[case("abc::datetime", SQLite, "datetime(abc)")]
#[case("abc::timestamp", NSQL, "abc::timestamp")]
#[case("abc::timestamp", PostgreSQL, "abc::timestamp")]
#[case("abc::timestamp", Oracle, "to_timestamp(abc, 'yyyy-mm-dd hh24:mi:ss')")]
#[case("abc::timestamp", MySQL, "convert(abc, datetime)")]
#[case("abc::timestamp", SqlServer, "convert(datetime, abc)")]
#[case("abc::timestamp", SQLite, "datetime(abc)")]
#[case("abc::time", NSQL, "abc::time")]
#[case("abc::time", PostgreSQL, "abc::time")]
//#[case("abc::time", Oracle, "")]
#[case("abc::time", MySQL, "convert(abc, time)")]
#[case("abc::time", SqlServer, "convert(time, abc)")]
#[case("abc::time", SQLite, "time(abc)")]
fn test_type_cast(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

#[theory]
#[test]
//#[case("format(now(), 'yyyy-mm-dd')", NSQL, "format(now(), 'yyyy-mm-dd')")]
//#[case("format(now(), 'yyyy-mm-dd')", PostgreSQL, "to_char(now(), 'yyyy-mm-dd')")]
//#[case("format(now(), 'yyyy-mm-dd')", Oracle, "to_char(systimestamp, 'yyyy-mm-dd')")]
//#[case("format(now(), 'yyyy-mm-dd')", MySQL, "date_format(now(), '%Y-%m-%d')")]
//
//#[case("format(now(), 'yyyy-mm-dd hh24:mi:ss')", NSQL, "format(now(), 'yyyy-mm-dd hh24:mi:ss')")]
//#[case("format(now(), 'yyyy-mm-dd hh24:mi:ss')", PostgreSQL, "to_char(now(), 'yyyy-mm-dd hh24:mi:ss')")]
//#[case("format(now(), 'yyyy-mm-dd hh24:mi:ss')", Oracle, "to_char(systimestamp, 'yyyy-mm-dd hh24:mi:ss')")]
//#[case("format(now(), 'yyyy-mm-dd hh24:mi:ss')", MySQL, "date_format(now(), '%Y-%m-%d %H:%i:%s')")]
//
//
//#[case("format(now(), 'yyyy/mm/dd')", NSQL, "format(now(), 'yyyy/mm/dd')")]
//#[case("format(now(), 'yyyy/mm/dd')", PostgreSQL, "to_char(now(), 'yyyy/mm/dd')")]
//#[case("format(now(), 'yyyy/mm/dd')", Oracle, "to_char(systimestamp, 'yyyy/mm/dd')")]
//#[case("format(now(), 'yyyy/mm/dd')", MySQL, "date_format(now(), '%Y/%m/%d')")]
//
//
//#[case("format(now(), 'yyyy/mm/dd hh24:mi:ss')", NSQL, "format(now(), 'yyyy/mm/dd hh24:mi:ss')")]
//#[case("format(now(), 'yyyy/mm/dd hh24:mi:ss')", PostgreSQL, "to_char(now(), 'yyyy/mm/dd hh24:mi:ss')")]
//#[case("format(now(), 'yyyy/mm/dd hh24:mi:ss')", Oracle, "to_char(systimestamp, 'yyyy/mm/dd hh24:mi:ss')")]
//#[case("format(now(), 'yyyy/mm/dd hh24:mi:ss')", MySQL, "date_format(now(), '%Y/%m/%d %H:%i:%s')")]
//
//
//
//#[case("format(now(), 'yyyy年mm月dd日')", NSQL, "format(now(), 'yyyy年mm月dd日')")]
//#[case("format(now(), 'yyyy年mm月dd日')", PostgreSQL, "to_char(now(), 'yyyy年mm月dd日')")]
//#[case("format(now(), 'yyyy年mm月dd日')", Oracle, "to_char(systimestamp, 'yyyy年mm月dd日')")]
//#[case("format(now(), 'yyyy年mm月dd日')", MySQL, "date_format(now(), '%Y年%m月%d日')")]
//
//
//#[case("format(now(), 'yyyy年mm月dd日 hh24:mi:ss')", NSQL, "format(now(), 'yyyy年mm月dd日 hh24:mi:ss')")]
//#[case("format(now(), 'yyyy年mm月dd日 hh24:mi:ss')", PostgreSQL, "to_char(now(), 'yyyy年mm月dd日 hh24:mi:ss')")]
//#[case("format(now(), 'yyyy年mm月dd日 hh24:mi:ss')", Oracle, "to_char(systimestamp, 'yyyy年mm月dd日 hh24:mi:ss')")]
//#[case("format(now(), 'yyyy年mm月dd日 hh24:mi:ss')", MySQL, "date_format(now(), '%Y年%m月%d日 %H:%i:%s')")]
#[allow(dead_code)]
fn test_format(left: &str, database_type: DatabaseType, right: &str) {
    test_expression(database_type, left, right);
}

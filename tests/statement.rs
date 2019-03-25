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
#[case("select * FROM student", NSQL, "select * from student")]
#[case("select name FROM student", NSQL, "select name from student")]
#[case(
    "select nvl(name,     'No Name') FROM student",
    NSQL,
    "select nvl(name, 'No Name') from student"
)]
#[case(
    "select nvl(name,     'No Name') as x FROM student",
    NSQL,
    "select nvl(name, 'No Name') as x from student"
)]
#[case(
    "select nvl(name,     'No Name')       x FROM student",
    NSQL,
    "select nvl(name, 'No Name') as x from student"
)]

fn test_select(left: &str, database_type: DatabaseType, right: &str) {
    test_statement(database_type, left, right);
}

#[test]
fn test_intersect() {
    test_statement(NSQL, "select name from student where gender ='男' intersect select name from student where gender ='女'",
                   "select name from student where gender = '男' intersect select name from student where gender = '女'");
}

#[test]
fn test_select_two_element() {
    test_statement(
        NSQL,
        "select name, age FROM student",
        "select name, age from student",
    );
}

#[test]
fn test_select_two_element1() {
    test_statement(
        NSQL,
        "select t.name, t.age FROM student t",
        "select t.name, t.age from student as t",
    );
}

#[test]
fn test_select_with_where() {
    test_statement(
        NSQL,
        "select name FROM student where age > 3",
        "select name from student where age > 3",
    );
}

#[test]
fn test_select_with_group() {
    test_statement(
        NSQL,
        "select count(id), gender FROM student group by gender",
        "select count(id), gender from student group by gender",
    );
}

#[test]
fn test_select_with_group_having() {
    test_statement(
        NSQL,
        "select count(id), gender FROM student group by gender having count(id) > 3",
        "select count(id), gender from student group by gender having count(id) > 3",
    );
}

#[test]
fn test_select_with_order() {
    test_statement(
        NSQL,
        "select count(id) FROM student order by name",
        "select count(id) from student order by name",
    );
}

#[test]
fn test_select_with_skip() {
    test_statement(
        NSQL,
        "select count(id) FROM student skip 3",
        "select count(id) from student skip 3",
    );
}

#[test]
fn test_oracle_select_with_skip() {
    test_statement(
        Oracle,
        "select count(id) FROM student skip 3",
        "select count(id) from student offset 3 rows",
    );
}

#[test]
fn test_oracle_select_with_skip1() {
    test_statement(
        Oracle,
        "select count(id) FROM student skip 1",
        "select count(id) from student offset 1 row",
    );
}

#[test]
fn test_select_with_limit() {
    test_statement(
        NSQL,
        "select count(id) FROM student limit 3",
        "select count(id) from student limit 3",
    );
}

#[test]
fn test_oracle_select_with_limit() {
    test_statement(
        Oracle,
        "select count(id) FROM student limit 3",
        "select count(id) from student fetch first 3 rows only",
    );
}

#[test]
fn test_oracle_select_with_limit1() {
    test_statement(
        Oracle,
        "select count(id) FROM student limit 1",
        "select count(id) from student fetch first 1 row only",
    );
}

#[test]
fn test_select_with_limit1() {
    test_statement(
        NSQL,
        "select count(id) FROM student limit 3   + 2",
        "select count(id) from student limit 3 + 2",
    );
}

#[test]
fn test_select_full() {
    test_statement(NSQL, "select count(id), name FROM student where a>3 group by name having count(id) > 1 order by age skip 2 limit 3",
                   "select count(id), name from student where a > 3 group by name having count(id) > 1 order by age skip 2 limit 3");
}

#[test]
fn test_select_from_dual() {
    test_statement(NSQL, "select now()", "select now()");
}

#[test]
fn test_select_from_dual1() {
    test_statement(NSQL, "select now() from dual", "select now()");
}

#[test]
fn test_select_from_sub_query() {
    test_statement(
        NSQL,
        "SELECT NOW() from (select * from student) t",
        "select now() from (select * from student) as t",
    );
}

#[test]
fn test_select_case_when() {
    test_statement(NSQL, "select case when score >= 85 then 'A' when score<85 and score >=60 then 'B' else 'C' end level from student",
                   "select case when score >= 85 then 'A' when score < 85 and score >= 60 then 'B' else 'C' end as level from student" );
}

#[test]
fn test_select_join() {
    test_statement(
        NSQL,
        "SELECT name from (select * from tree) t join tree t2 on t2.parent_id = t.id",
        "select name from (select * from tree) as t join tree as t2 on (t2.parent_id = t.id)",
    );
}

#[test]
fn test_union() {
    test_statement(NSQL, "select name from student where gender ='男' union select name from student where gender ='女'",
                   "select name from student where gender = '男' union select name from student where gender = '女'");
}

#[test]
fn test_union1() {
    test_statement(NSQL, "select name from student where gender ='男' union select name from student where gender ='女' skip 1 limit 2",
                   "select name from student where gender = '男' union select name from student where gender = '女' skip 1 limit 2");
}

#[test]
fn test_union2() {
    let statement = StatementParser::new()
        .parse(Lexer::new("select name from student where gender ='男' skip 2 union select name from student where gender ='女' skip 1 limit 2").tokenizer());
    assert_eq!(true, statement.is_err());
}

#[test]
fn test_union3() {
    test_statement(NSQL, "(select name from student where gender ='男' skip 2) union select name from student where gender ='女' skip 1 limit 2",
                   "(select name from student where gender = '男' skip 2) union select name from student where gender = '女' skip 1 limit 2");
}

#[test]
fn test_union_all() {
    test_statement(NSQL, "select name from student where gender ='男' union all select name from student where gender ='女'",
                   "select name from student where gender = '男' union all select name from student where gender = '女'");
}

#[test]
fn test() {
    test_statement(NSQL, "select name from student where gender ='男' except select name from student where gender ='女'",
                   "select name from student where gender = '男' minus select name from student where gender = '女'");
}

#[test]
fn test1() {
    test_statement(NSQL, "select name from student where gender ='男' minus select name from student where gender ='女'",
                   "select name from student where gender = '男' minus select name from student where gender = '女'");
}

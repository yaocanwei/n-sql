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
#[case(1, 2, 3)]
#[case(3, 4, 7)]
#[case(3, - 1, 2)]
#[case(3, - 2, 1)]
fn num_add(a: i32, b: i32, result: i32) {
    (a + b).should().be_equal_to(result);
}

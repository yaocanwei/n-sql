// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
#![cfg(target_arch="wasm32")]

use wasm_bindgen::prelude::*;
use ::{StatementParser, Lexer};
use ::generator::*;
use ::cfg_if;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(all(target_arch="wasm32", feature="wee_alloc"))] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
pub fn translate(sql: &str) -> String {
    let statement = StatementParser::new().parse(Lexer::new(sql).tokenizer()).unwrap();
    statement.to_sql().unwrap()
}
#[wasm_bindgen]
pub fn translate_pgsql(sql: &str) -> String {
    let statement = StatementParser::new().parse(Lexer::new(sql).tokenizer()).unwrap();
    statement.to_pgsql().unwrap()
}
#[wasm_bindgen]
pub fn translate_oracle(sql: &str) -> String {
    let statement = StatementParser::new().parse(Lexer::new(sql).tokenizer()).unwrap();
    statement.to_oracle().unwrap()
}

#[wasm_bindgen]
pub fn translate_mysql(sql: &str) -> String {
    let statement = StatementParser::new().parse(Lexer::new(sql).tokenizer()).unwrap();
    statement.to_mysql().unwrap()
}

#[wasm_bindgen]
pub fn translate_sql_server(sql: &str) -> String {
    let statement = StatementParser::new().parse(Lexer::new(sql).tokenizer()).unwrap();
    statement.to_sql_server().unwrap()
}

#[wasm_bindgen]
pub fn translate_sqlite(sql: &str) -> String {
    let statement = StatementParser::new().parse(Lexer::new(sql).tokenizer()).unwrap();
    statement.to_sqlite().unwrap()
}

mod utils{
    use ::cfg_if;
    cfg_if! {
        // When the `console_error_panic_hook` feature is enabled, we can call the
        // `set_panic_hook` function at least once during initialization, and then
        // we will get better error messages if our code ever panics.
        //
        // For more details see
        // https://github.com/rustwasm/console_error_panic_hook#readme
        if #[cfg(feature = "console_error_panic_hook")] {
            extern crate console_error_panic_hook;
            pub use self::console_error_panic_hook::set_once as set_panic_hook;
        } else {
            #[inline]
            pub fn set_panic_hook() {}
        }
    }
}
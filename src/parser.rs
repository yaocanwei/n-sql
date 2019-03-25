// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
extern crate lalrpop_util;

use self::lalrpop_util::ParseError;

use lexer::Position;
use lexer::SpannedError;
use lexer::Token;
use Expression;
use Lexer;
use Statement;

use lexer::ParserSource;
use ExpressionParser;
use StatementParser;

type ParseResult<'input, T> = Result<T, ParseError<Position, Token<'input>, SpannedError>>;

pub trait IParser<T> {
    fn parse<S>(input: &S) -> ParseResult<T>
    where
        S: ?Sized + ParserSource;
}

impl IParser<Expression> for Expression {
    fn parse<S>(input: &S) -> ParseResult<Expression>
    where
        S: ?Sized + ParserSource,
    {
        ExpressionParser::new().parse(Lexer::new(input).tokenizer())
    }
}

impl IParser<Statement> for Statement {
    fn parse<S>(input: &S) -> ParseResult<Statement>
    where
        S: ?Sized + ParserSource,
    {
        StatementParser::new().parse(Lexer::new(input).tokenizer())
    }
}

pub fn parse<T: IParser<T>>(input: &str) -> ParseResult<T> {
    T::parse(input)
}

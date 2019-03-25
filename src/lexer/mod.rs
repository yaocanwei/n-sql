// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
#![allow(unused_imports)]
#![allow(dead_code)]

extern crate codespan;
extern crate codespan_reporting;
extern crate regex;

mod char_locations;
mod comment;
mod location;
mod parser_source;
mod source;
mod token;

use self::regex::{RegexSet, RegexSetBuilder};

use self::char_locations::CharLocations;
use self::comment::*;
use self::source::Source;

use self::location::{spanned, ByteOffset, Column, ColumnOffset, Line, Span};
pub use self::location::{Location, Position, Spanned};
pub use self::parser_source::ParserSource;
use self::LexicalError::*;

pub use self::token::Token;
use lexer::token::Token::EOF;

//noinspection SpellCheckingInspection
const REGEX_SOURCE: [(&str, Token); 135] = [
    // region [keyword]
    ("^(?i)all$", Token::All),
    ("^(?i)and$", Token::And),
    ("^(?i)as$", Token::As),
    ("^(?i)asc$", Token::Asc),
    ("^(?i)both$", Token::Both),
    ("^(?i)by$", Token::By),
    ("^(?i)case$", Token::Case),
    ("^(?i)cross$", Token::Cross),
    ("^(?i)desc$", Token::Desc),
    ("^(?i)distinct", Token::Distinct),
    ("^(?i)dual$", Token::Dual),
    ("^(?i)else$", Token::Else),
    ("^(?i)end$", Token::End),
    ("^(?i)except$", Token::Except),
    ("^(?i)from$", Token::From),
    ("^(?i)full$", Token::Full),
    ("^(?i)group$", Token::Group),
    ("^(?i)having$", Token::Having),
    ("^(?i)in$", Token::In),
    ("^(?i)is$", Token::Is),
    ("^(?i)inner$", Token::Inner),
    ("^(?i)intersect$", Token::Intersect),
    ("^(?i)join$", Token::Join),
    ("^(?i)leading$", Token::Leading),
    ("^(?i)left$", Token::Left),
    ("^(?i)limit$", Token::Limit),
    ("^(?i)minus$", Token::Minus),
    ("^(?i)not$", Token::Not),
    ("^(?i)null$", Token::Null),
    ("^(?i)offset$", Token::Offset),
    ("^(?i)on$", Token::On),
    ("^(?i)or$", Token::Or),
    ("^(?i)order$", Token::Order),
    ("^(?i)outer$", Token::Outer),
    ("^(?i)right", Token::Right),
    ("^(?i)select$", Token::Select),
    ("^(?i)skip$", Token::Skip),
    ("^(?i)then$", Token::Then),
    ("^(?i)trailing$", Token::Trailing),
    ("^(?i)union$", Token::Union),
    ("^(?i)unique$", Token::Unique),
    ("^(?i)when$", Token::When),
    ("^(?i)where$", Token::Where),
    ("^(?i)with$", Token::With),
    ("^(?i)within$", Token::Within),
    // endregion,

    // region [function]
    ("^(?i)abs", Token::Abs),
    ("^(?i)avg$", Token::Avg),
    ("^(?i)avgif$", Token::AvgIf),
    ("^(?i)btrim$", Token::BTrim),
    ("^(?i)cast$", Token::Cast),
    ("^(?i)ceil$", Token::Ceil),
    ("^(?i)ceiling$", Token::Ceiling),
    ("^(?i)coalesce$", Token::Coalesce),
    ("^(?i)cos$", Token::Cos),
    ("^(?i)concat$", Token::Concat),
    ("^(?i)count$", Token::Count),
    ("^(?i)countif$", Token::CountIf),
    ("^(?i)day$", Token::Day),
    ("^(?i)day_add$", Token::DayAdd),
    ("^(?i)day_diff$", Token::DayDiff),
    ("^(?i)day_sub$", Token::DaySub),
    ("^(?i)decode$", Token::Decode),
    ("^(?i)dense_rank$", Token::DenseRank),
    ("^(?i)extract$", Token::Extract),
    ("^(?i)floor$", Token::Floor),
    ("^(?i)hour$", Token::Hour),
    ("^(?i)hour_add$", Token::HourAdd),
    ("^(?i)hour_diff$", Token::HourDiff),
    ("^(?i)hour_sub$", Token::HourSub),
    ("^(?i)length$", Token::Length),
    ("^(?i)log$", Token::Log),
    ("^(?i)log10$", Token::Log10),
    ("^(?i)lower$", Token::Lower),
    ("^(?i)lpad$", Token::LPad),
    ("^(?i)ltrim$", Token::LTrim),
    ("^(?i)max$", Token::Max),
    ("^(?i)maxif$", Token::MaxIf),
    ("^(?i)median$", Token::Median),
    ("^(?i)medianif$", Token::MedianIf),
    ("^(?i)min$", Token::Min),
    ("^(?i)minif$", Token::MinIf),
    ("^(?i)minute$", Token::Minute),
    ("^(?i)minute_add$", Token::MinuteAdd),
    ("^(?i)minute_diff$", Token::MinuteDiff),
    ("^(?i)minute_sub$", Token::MinuteSub),
    ("^(?i)month$", Token::Month),
    ("^(?i)month_add$", Token::MonthAdd),
    ("^(?i)month_diff$", Token::MonthDiff),
    ("^(?i)month_sub$", Token::MonthSub),
    ("^(?i)now$", Token::Now),
    ("^(?i)nvl$", Token::Nvl),
    ("^(?i)pad_left$", Token::PadLeft),
    ("^(?i)pad_right$", Token::PadRight),
    ("^(?i)percent$", Token::Percent),
    ("^(?i)percentile$", Token::Percentile),
    ("^(?i)percentile_cont$", Token::PercentileCont),
    ("^(?i)percentile_disc$", Token::PercentileDisc),
    ("^(?i)pow$", Token::Pow),
    ("^(?i)power$", Token::Power),
    ("^(?i)replace$", Token::Replace),
    ("^(?i)reverse$", Token::Reverse),
    ("^(?i)rank$", Token::Rank),
    ("^(?i)round$", Token::Round),
    ("^(?i)sign$", Token::Sign),
    ("^(?i)sin$", Token::Sin),
    ("^(?i)sqrt$", Token::Sqrt),
    ("^(?i)stddev$", Token::Stddev),
    ("^(?i)stddevif$", Token::StddevIf),
    ("^(?i)rpad$", Token::RPad),
    ("^(?i)rtrim$", Token::RTrim),
    ("^(?i)second$", Token::Second),
    ("^(?i)second_add$", Token::SecondAdd),
    ("^(?i)second_diff$", Token::SecondDiff),
    ("^(?i)second_sub", Token::SecondSub),
    ("^(?i)substr$", Token::Substr),
    ("^(?i)substring$", Token::Substring),
    ("^(?i)sum$", Token::Sum),
    ("^(?i)sumif$", Token::SumIf),
    ("^(?i)tan$", Token::Tan),
    ("^(?i)trim$", Token::Trim),
    ("^(?i)trim_start$", Token::TrimStart),
    ("^(?i)trim_end$", Token::TrimEnd),
    ("^(?i)upper$", Token::Upper),
    ("^(?i)year$", Token::Year),
    ("^(?i)year_add$", Token::YearAdd),
    ("^(?i)year_diff$", Token::YearDiff),
    ("^(?i)year_sub$", Token::YearSub),
    // endregion

    // region
    ("^text$", Token::Text),
    ("^int$", Token::Int),
    ("^float$", Token::Float),
    ("^numeric$", Token::Numeric),
    ("^timestamp$", Token::Timestamp),
    ("^datetime$", Token::Datetime),
    ("^date$", Token::Date),
    ("^time$", Token::Time),
    // endregion
];

impl<'input> Token<'input> {
    fn token_type(&self) -> &'static str {
        use self::Token::*;
        match self {
            Select | From | Where | Group | Order | Skip | Limit | By | In | Not => "keyword",
            Nvl | Day => "function",
            StringLiteral(_) => "string",
            IntLiteral(_) | FloatLiteral(_) => "number",
            DocComment(_) => "comment",
            _ => "unknown",
        }
    }
}

struct MatcherBuilder {
    regex_set: RegexSet,
}

impl MatcherBuilder {
    fn new() -> Self {
        let regex_set = RegexSetBuilder::new(REGEX_SOURCE.iter().map(|(s, _)| s))
            .build()
            .unwrap();
        MatcherBuilder { regex_set }
    }

    fn matcher<'input, 'builder>(&'builder self, s: &'input str) -> Matcher<'input, 'builder> {
        Matcher {
            text: s,
            consumed: 0,
            regex_set: &self.regex_set,
        }
    }
}

struct Matcher<'input, 'builder> {
    text: &'input str,
    consumed: usize,
    regex_set: &'builder regex::RegexSet,
}

impl<'input, 'builder> Iterator for Matcher<'input, 'builder> {
    type Item = Token<'input>;

    //noinspection RsTypeCheck
    fn next(&mut self) -> Option<Self::Item> {
        let matches: Vec<usize> = self.regex_set.matches(self.text).into_iter().collect();
        if matches.len() > 1 {
            panic!("Non-unique matching error!");
        }
        if let Some(i) = matches.first() {
            let (_, t) = &REGEX_SOURCE[*i];
            Some(t.clone())
        } else {
            None
        }
    }
}

//noinspection ALL
#[derive(Clone, Debug, PartialEq, Eq, Fail)]
pub enum LexicalError {
    #[fail(display = "empty char literal")]
    EmptyCharLiteral,

    #[fail(display = "unexpected character:{}", _0)]
    UnexpectedChar(char),

    #[fail(display = "unexpected end of file")]
    UnexpectedEof,

    #[fail(display = "unexpected escape code:{}", _0)]
    UnexpectedEscapeCode(char),

    #[fail(display = "unterminated string literal")]
    UnterminatedStringLiteral,

    #[fail(display = "cannot parse integer, probable overflow")]
    NonParseableInt,

    #[fail(display = "cannot parse hex literal, overflow")]
    HexLiteralOverflow,

    #[fail(display = "cannot parse hex literal, underflow")]
    HexLiteralUnderflow,

    #[fail(display = "wrong hex literal prefix, should start as '0x' or '-0x'")]
    HexLiteralWrongPrefix,

    #[fail(display = "cannot parse hex literal, incomplete")]
    HexLiteralIncomplete,
}

pub type SpannedToken<'input> = Spanned<Token<'input>, Location>;

pub type SpannedError = Spanned<LexicalError, Location>;

fn error<T>(location: Location, code: LexicalError) -> Result<T, SpannedError> {
    Err(spanned(location, location, code))
}

pub fn is_identifier_start(ch: char) -> bool {
    match ch {
        '_' | 'a'...'z' | 'A'...'Z' | '\u{4E00}'...'\u{9FA5}' | '\u{F900}'...'\u{FA2D}' => true,
        _ => false,
    }
}

pub fn is_identifier_continue(ch: char) -> bool {
    match ch {
        '0'...'9' | '\'' => true,
        ch => is_identifier_start(ch),
    }
}

pub fn is_digit(ch: char) -> bool {
    ch.is_digit(10)
}

pub fn is_hex(ch: char) -> bool {
    ch.is_digit(16)
}

pub struct Lexer<'input> {
    input: &'input str,
    chars: CharLocations<'input>,
    lookahead: Option<(Location, char)>,
    start_index: Position,
    builder: MatcherBuilder,
}

impl<'input> Lexer<'input> {
    pub fn new<S>(input: &'input S) -> Self
    where
        S: ?Sized + ParserSource,
    {
        let mut chars = CharLocations::new(input);
        Lexer {
            input: input.src(),
            start_index: input.start_index(),
            lookahead: chars.next(),
            chars,
            builder: MatcherBuilder::new(),
        }
    }

    pub fn tokenizer(self) -> Tokenizer<'input> {
        Tokenizer(self)
    }

    fn bump(&mut self) -> Option<(Location, char)> {
        match self.lookahead {
            Some((location, ch)) => {
                self.lookahead = self.chars.next();
                Some((location, ch))
            }
            None => None,
        }
    }

    fn skip_to_end(&mut self) {
        while let Some(_) = self.bump() {}
    }

    fn next_location(&self) -> Location {
        self.lookahead.as_ref().map_or(self.chars.location, |l| l.0)
    }

    fn identifier(&mut self, start: Location) -> Result<SpannedToken<'input>, SpannedError> {
        let (mut end, mut identifier) = self.take_while(start, is_identifier_continue);
        match self.lookahead {
            Some((_, c)) if c == '!' => {
                self.bump();
                end.column += 1.into();
                end.absolute += 1.into();
                identifier = self.slice(start, end);
            }
            _ => (),
        }

        let token = match self.builder.matcher(identifier).next() {
            Some(t) => t,
            None => Token::Identifier(identifier),
        };

        //        let token = Token::Identifier(identifier);

        Ok(spanned(start, end, token))
    }

    fn numeric_literal(&mut self, start: Location) -> Result<SpannedToken<'input>, SpannedError> {
        let (end, int) = self.take_while(start, is_digit);
        let (start, end, token) = if int.chars().next().unwrap() == '.' {
            match self.lookahead {
                Some((_, ch)) if ch.is_whitespace() => {
                    (start, end, Token::FloatLiteral(int.parse().unwrap()))
                }
                None => (start, end, Token::FloatLiteral(int.parse().unwrap())),
                _ => panic!("错误"),
            }
        } else {
            match self.lookahead {
                Some((_, '.')) => {
                    self.bump(); // Skip '.'
                    let (end, float) = self.take_while(start, is_digit);
                    match self.lookahead {
                        Some((_, ch)) if is_identifier_start(ch) => {
                            return self.error(end, UnexpectedChar(ch));
                        }
                        _ => (start, end, Token::FloatLiteral(float.parse().unwrap())),
                    }
                }
                Some((_, 'x')) => {
                    self.bump(); // Skip 'x'
                    let int_start = self.next_location();
                    let (end, hex) = self.take_while(int_start, is_hex);
                    match int {
                        "0" | "-0" => match self.lookahead {
                            Some((_, ch)) if is_identifier_start(ch) => {
                                return self.error(end, UnexpectedChar(ch));
                            }
                            _ => {
                                if hex.is_empty() {
                                    return self.error(start, HexLiteralIncomplete);
                                }
                                let is_positive = int == "0";
                                match i64_from_hex(hex, is_positive) {
                                    Ok(val) => (start, end, Token::IntLiteral(val)),
                                    Err(err) => return self.error(start, err),
                                }
                            }
                        },
                        _ => return self.error(start, HexLiteralWrongPrefix),
                    }
                }
                Some((_, 'b')) => {
                    self.bump(); // Skip 'b'
                    let end = self.next_location();
                    match self.lookahead {
                        Some((pos, ch)) if is_identifier_start(ch) => {
                            return self.error(pos, UnexpectedChar(ch));
                        }
                        _ => {
                            if let Ok(val) = int.parse() {
                                (start, end, Token::ByteLiteral(val))
                            } else {
                                return self.error(start, NonParseableInt);
                            }
                        }
                    }
                }
                Some((start, ch)) if is_identifier_start(ch) => {
                    return self.error(start, UnexpectedChar(ch))
                }
                None | Some(_) => {
                    if let Ok(val) = int.parse() {
                        (start, end, Token::IntLiteral(val))
                    } else {
                        return self.error(start, NonParseableInt);
                    }
                }
            }
        };
        Ok(spanned(start, end, token))
    }

    fn test_lookahead<F: FnMut(char) -> bool>(&self, mut test: F) -> bool {
        self.lookahead.map_or(false, |(_, ch)| test(ch))
    }

    fn line_comment(&mut self, start: Location) -> Option<SpannedToken<'input>> {
        let (end, comment) = self.take_until(start, |ch| ch == '\n');

        if comment.starts_with("--") {
            let skip = if comment.starts_with("-- ") { 3 } else { 2 };
            let comment = Token::DocComment(Comment {
                r#type: CommentType::Line,
                content: comment[skip..].to_string(),
            });
            Some(spanned(start, end, comment))
        } else {
            None
        }
    }

    fn eof_error<T>(&mut self) -> Result<T, SpannedError> {
        let location = self.next_location();
        self.error(location, UnexpectedEof)
    }

    fn error<T>(&mut self, location: Location, code: LexicalError) -> Result<T, SpannedError> {
        self.skip_to_end();
        Err(spanned(location, location, code))
    }

    fn escape_code(&mut self) -> Result<char, SpannedError> {
        match self.bump() {
            Some((_, '\'')) => Ok('\''),
            Some((_, '\\')) => Ok('\\'),
            Some((_, '/')) => Ok('/'),
            Some((_, 'n')) => Ok('\n'),
            Some((_, 'r')) => Ok('\r'),
            Some((_, 't')) => Ok('\t'),
            // TODO: Unicode escape codes
            Some((start, ch)) => self.error(start, UnexpectedEscapeCode(ch)),
            None => self.eof_error(),
        }
    }

    fn slice(&self, start: Location, end: Location) -> &'input str {
        let start = start.absolute - ByteOffset(self.start_index.to_usize() as i64);
        let end = end.absolute - ByteOffset(self.start_index.to_usize() as i64);
        &self.input[start.to_usize()..end.to_usize()]
    }

    fn take_while<F: FnMut(char) -> bool>(
        &mut self,
        start: Location,
        mut keep_going: F,
    ) -> (Location, &'input str) {
        self.take_until(start, |c| !keep_going(c))
    }

    fn take_until<F: FnMut(char) -> bool>(
        &mut self,
        start: Location,
        mut terminate: F,
    ) -> (Location, &'input str) {
        while let Some((end, ch)) = self.lookahead {
            if terminate(ch) {
                return (end, self.slice(start, end));
            } else {
                self.bump();
            }
        }
        (
            self.next_location(),
            self.slice(start, self.next_location()),
        )
    }

    fn string_literal(&mut self, start: Location) -> Result<SpannedToken<'input>, SpannedError> {
        let mut string = String::new();
        while let Some((_, ch)) = self.bump() {
            match ch {
                '\'' => {
                    if self.test_lookahead(|c| c == '\'') {
                        self.bump(); // skip '\''
                        string.push(ch);
                    } else {
                        let end = self.next_location();
                        let token = Token::StringLiteral(string);
                        return Ok(spanned(start, end, token));
                    }
                }
                ch => string.push(ch),
            }
        }

        self.error(start, UnterminatedStringLiteral)
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Result<SpannedToken<'input>, SpannedError>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((start, ch)) = self.bump() {
            return match ch {
                '-' if self.test_lookahead(|ch| ch == '-') => match self.line_comment(start) {
                    Some(token) => Some(Ok(token)),
                    None => continue,
                },
                '!' | '^' | '~' if self.test_lookahead(|ch| ch == '=') => {
                    self.bump(); // skip '='
                    Some(Ok(spanned(start, self.next_location(), Token::NotEqual)))
                }
                ':' => {
                    if self.test_lookahead(|ch| ch == ':') {
                        self.bump(); // skip ':'
                        Some(Ok(spanned(start, self.next_location(), Token::DoubleColon)))
                    } else {
                        Some(Ok(spanned(start, self.next_location(), Token::Colon)))
                    }
                }
                '|' => {
                    if self.test_lookahead(|ch| ch == '|') {
                        self.bump(); // skip ':'
                        Some(Ok(spanned(start, self.next_location(), Token::DoublePipe)))
                    } else {
                        Some(Ok(spanned(start, self.next_location(), Token::Pipe)))
                    }
                }
                '<' => {
                    if let Some((_, ch)) = self.lookahead {
                        match ch {
                            '=' => {
                                self.bump(); // skip '='
                                Some(Ok(spanned(start, self.next_location(), Token::LessOrEqual)))
                            }
                            '>' => {
                                self.bump(); // skip '>'
                                Some(Ok(spanned(start, self.next_location(), Token::NotEqual)))
                            }
                            _ => Some(Ok(spanned(start, self.next_location(), Token::Less))),
                        }
                    } else {
                        Some(Ok(spanned(start, self.next_location(), Token::Less)))
                    }
                }
                '>' => {
                    if self.test_lookahead(|ch| ch == '=') {
                        self.bump(); // skip '='
                        Some(Ok(spanned(
                            start,
                            self.next_location(),
                            Token::GreaterOrEqual,
                        )))
                    } else {
                        Some(Ok(spanned(start, self.next_location(), Token::Greater)))
                    }
                }
                ch if is_digit(ch)
                    || ((ch == '-' || ch == '.') && self.test_lookahead(is_digit)) =>
                {
                    Some(self.numeric_literal(start))
                }
                '\'' => Some(self.string_literal(start)),
                '(' => Some(Ok(spanned(start, self.next_location(), Token::LeftParen))),
                ')' => Some(Ok(spanned(start, self.next_location(), Token::RightParen))),
                ',' => Some(Ok(spanned(start, self.next_location(), Token::Comma))),
                '.' => Some(Ok(spanned(start, self.next_location(), Token::Period))),
                '=' => Some(Ok(spanned(start, self.next_location(), Token::Equal))),
                '+' => Some(Ok(spanned(start, self.next_location(), Token::PlusSign))),
                '-' => Some(Ok(spanned(start, self.next_location(), Token::MinusSign))),
                '*' => Some(Ok(spanned(start, self.next_location(), Token::Asterisk))),
                '/' => Some(Ok(spanned(start, self.next_location(), Token::Solidus))),
                ch if is_identifier_start(ch) => Some(self.identifier(start)),
                ch if ch.is_whitespace() => continue,
                ch => Some(self.error(start, UnexpectedChar(ch))),
            };
        }
        // Return EOF instead of None so that the layout algorithm receives the eof location
        //        Some(Ok(spanned(
        //            self.next_location(),
        //            self.next_location(),
        //            Token::EOF,
        //        )))
        None
    }
}

pub struct Tokenizer<'input>(Lexer<'input>);

impl<'input> Iterator for Tokenizer<'input> {
    type Item = Result<(Position, Token<'input>, Position), SpannedError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            Some(t) => Some(match t {
                Ok(t) => Ok((t.span.start().absolute, t.value, t.span.end().absolute)),
                Err(t) => Err(t),
            }),
            None => None,
        }
    }
}

/// Converts partial hex literal (i.e. part after `0x` or `-0x`) to 64 bit signed integer.
///
/// This is basically a copy and adaptation of `std::num::from_str_radix`.
fn i64_from_hex(hex: &str, is_positive: bool) -> Result<i64, LexicalError> {
    const RADIX: u32 = 16;
    let digits = hex.as_bytes();
    let sign: i64 = if is_positive { 1 } else { -1 };
    let mut result = 0i64;
    for &c in digits {
        let x = (c as char).to_digit(RADIX).expect("valid hex literal");
        result = result
            .checked_mul(RADIX as i64)
            .and_then(|result| result.checked_add((x as i64) * sign))
            .ok_or_else(|| {
                if is_positive {
                    HexLiteralOverflow
                } else {
                    HexLiteralUnderflow
                }
            })?;
    }
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::Token::*;
    use super::*;

    fn location(byte: u32) -> Location {
        Location {
            line: Line(0),
            column: Column(byte + 1),
            absolute: Position(byte + 1),
        }
    }

    fn tokenizer<'input>(
        input: &'input str,
    ) -> impl Iterator<Item = Result<SpannedToken<'input>, SpannedError>> + 'input {
        Box::new(Iterator::take_while(
            Lexer::new(input),
            |token| match *token {
                Ok(Spanned {
                    value: Token::EOF, ..
                }) => false,
                _ => true,
            },
        ))
    }

    fn test(input: &str, expected: Vec<(&str, Token)>) {
        use super::Source;

        let mut tokenizer = tokenizer(input);
        let mut count = 0;
        let length = expected.len();
        let source = self::codespan::FileMap::new("test".into(), input.to_string());

        for (token, (expected_span, expected_tok)) in tokenizer.by_ref().zip(expected.into_iter()) {
            count += 1;
            let start_byte =
                source.span().start() + ByteOffset(expected_span.find("~").unwrap() as i64);
            let mut start = Source::location(&source, start_byte).unwrap();
            start.column += ColumnOffset(1);

            let end_byte =
                source.span().start() + ByteOffset(expected_span.rfind("~").unwrap() as i64 + 1);
            let mut end = Source::location(&source, end_byte.into()).unwrap();
            end.column += ColumnOffset(1);

            assert_eq!(Ok(spanned(start, end, expected_tok)), token);
        }

        assert_eq!(count, length);
        assert_eq!(true, count > 0);

        // Make sure that there is nothing else to consume
        assert_eq!(None, tokenizer.next());
    }

    #[test]
    fn builtin_operators() {
        test(
            r#". : = | ( ) + - * / > < , <= >= != <> ^= ~= ::"#,
            vec![
                (r#"~                                             "#, Period),
                (r#"  ~                                           "#, Colon),
                (r#"    ~                                         "#, Equal),
                (r#"      ~                                       "#, Pipe),
                (
                    r#"        ~                                     "#,
                    LeftParen,
                ),
                (
                    r#"          ~                                   "#,
                    RightParen,
                ),
                (
                    r#"            ~                                 "#,
                    PlusSign,
                ),
                (
                    r#"              ~                               "#,
                    MinusSign,
                ),
                (
                    r#"                ~                             "#,
                    Asterisk,
                ),
                (r#"                  ~                           "#, Solidus),
                (r#"                    ~                         "#, Greater),
                (r#"                      ~                       "#, Less),
                (r#"                        ~                     "#, Comma),
                (
                    r#"                          ~~                  "#,
                    LessOrEqual,
                ),
                (
                    r#"                             ~~               "#,
                    GreaterOrEqual,
                ),
                (
                    r#"                                ~~            "#,
                    NotEqual,
                ),
                (
                    r#"                                   ~~         "#,
                    NotEqual,
                ),
                (
                    r#"                                      ~~      "#,
                    NotEqual,
                ),
                (
                    r#"                                         ~~   "#,
                    NotEqual,
                ),
                (
                    r#"                                            ~~"#,
                    DoubleColon,
                ),
            ],
        );
    }

    #[test]
    fn line_comments() {
        test(
            r#"h-i -- hellooo"#,
            vec![
                (r#"~             "#, Identifier("h")),
                (r#" ~            "#, MinusSign),
                (r#"  ~           "#, Identifier("i")),
                (
                    r#"    ~~ ~~~~~~~"#,
                    DocComment(Comment {
                        r#type: CommentType::Line,
                        content: "hellooo".to_string(),
                    }),
                ),
            ],
        );
    }

    #[test]
    fn string_literals() {
        test(
            r#"'abc' mn 'hjk''xyz'"#,
            vec![
                (r#"~~~~~              "#, StringLiteral("abc".to_string())),
                (r#"      ~~           "#, Identifier("mn")),
                (
                    r#"         ~~~~~~~~~~"#,
                    StringLiteral(r#"hjk'xyz"#.to_string()),
                ),
            ],
        );
    }

    #[test]
    fn float_literals() {
        test(
            r#"3.2 4.236 -9.365"#,
            vec![
                (r#"~~~             "#, FloatLiteral(3.2)),
                (r#"    ~~~~~       "#, FloatLiteral(4.236)),
                (r#"          ~~~~~~"#, FloatLiteral(-9.365)),
            ],
        );
    }

    #[test]
    fn keywords() {
        test(
            r#"select * from order left"#,
            vec![
                (r#"~~~~~~                  "#, Select),
                (r#"       ~                "#, Asterisk),
                (r#"         ~~~~           "#, From),
                (r#"              ~~~~~     "#, Order),
                (r#"                    ~~~~"#, Left),
            ],
        );
    }

    #[test]
    fn variable() {
        test(
            r#"a = :  m"#,
            vec![
                (r#"~       "#, Identifier("a")),
                (r#"  ~     "#, Equal),
                (r#"    ~   "#, Colon),
                (r#"       ~"#, Identifier("m")),
            ],
        );
    }

    #[test]
    fn test1() {
        test(
            "a >-3.2",
            vec![
                ("~      ", Identifier("a")),
                ("  ~    ", Greater),
                ("   ~~~~", FloatLiteral(-3.2)),
            ],
        )
    }
    #[test]
    fn test2() {
        test(
            "a > '3.2'",
            vec![
                ("~        ", Identifier("a")),
                ("  ~      ", Greater),
                ("    ~~~~~", StringLiteral("3.2".to_string())),
            ],
        )
    }
    #[test]
    fn test3() {
        test(
            "trim(trailing  'a' from 'abc')",
            vec![
                ("~~~~                          ", Trim),
                ("    ~                         ", LeftParen),
                ("     ~~~~~~~~                 ", Trailing),
                (
                    "               ~~~            ",
                    StringLiteral("a".to_string()),
                ),
                ("                   ~~~~       ", From),
                (
                    "                        ~~~~~ ",
                    StringLiteral("abc".to_string()),
                ),
                ("                             ~", RightParen),
            ],
        )
    }

    #[test]
    fn test_err() {
        test(
            "trim(trailing  'a' from 'abc')",
            vec![
                ("~~~~                          ", Trim),
                ("    ~                         ", LeftParen),
                ("     ~~~~~~~~                 ", Trailing),
                (
                    "               ~~~            ",
                    StringLiteral("a".to_string()),
                ),
                ("                   ~~~~       ", From),
                (
                    "                        ~~~~~ ",
                    StringLiteral("abc".to_string()),
                ),
                ("                             ~", RightParen),
            ],
        )
    }

    #[test]
    fn string_literal_unterminated() {
        assert_eq!(
            tokenizer(r#"foo 'bar''\n baz"#).last(),
            Some(error(location(4), UnterminatedStringLiteral))
        );
    }

}

// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use super::comment::Comment;

#[derive(Clone, PartialEq, Debug)]
pub enum Token<'input> {
    Identifier(&'input str),

    StringLiteral(String),
    IntLiteral(i64),
    ByteLiteral(u8),
    FloatLiteral(f64),
    DocComment(Comment),

    // region [builtin datatype]
    Text,
    Int,
    Float,
    Numeric,
    Timestamp,
    Datetime,
    Date,
    Time,
    // endregion

    // region [Symbol]
    /// `(`
    LeftParen,
    /// `)`
    RightParen,
    /// `|`
    Pipe,
    /// `||`
    DoublePipe,
    /// `,`
    Comma,
    /// `:`
    Colon,
    /// `::`
    DoubleColon,
    /// `.`
    Period,
    /// `=`
    Equal,
    /// `!=`,`<>`, `^=`, `~=`
    NotEqual,
    /// `<`
    Less,
    /// `<=`
    LessOrEqual,
    /// `>`
    Greater,
    /// `>=`
    GreaterOrEqual,
    /// `+`
    PlusSign,
    /// `-`
    MinusSign,
    /// `*`
    Asterisk,
    /// `/`
    Solidus,

    // endregion

    // region [keywords]
    All,
    And,
    As,
    Asc,
    Both,
    By,
    Case,
    Cross,
    Desc,
    Distinct,
    Dual,
    Else,
    End,
    Except,
    From,
    Full,
    Group,
    Having,
    In,
    Is,
    Inner,
    Intersect,
    Join,
    Leading,
    Left,
    Limit,
    Minus,
    Not,
    Null,
    Offset,
    On,
    Or,
    Order,
    Outer,
    Right,
    Select,
    Skip,
    Then,
    Trailing,
    Union,
    Unique,
    When,
    Where,
    With,
    Within,
    // endregion

    // region [function]
    Abs,
    Avg,
    AvgIf,
    BTrim,
    Cast,
    Ceil,
    Ceiling,
    Coalesce,
    Cos,
    Concat,
    Count,
    CountIf,
    Day,
    DayAdd,
    DayDiff,
    DaySub,
    Decode,
    DenseRank,
    Extract,
    Floor,
    Hour,
    HourAdd,
    HourDiff,
    HourSub,
    Length,
    Log,
    Log10,
    Lower,
    LPad,
    LTrim,
    Max,
    MaxIf,
    Median,
    MedianIf,
    Min,
    MinIf,
    Minute,
    MinuteAdd,
    MinuteDiff,
    MinuteSub,
    Month,
    MonthAdd,
    MonthDiff,
    MonthSub,
    Now,
    Nvl,
    PadLeft,
    PadRight,
    Percent,
    Percentile,
    PercentileCont,
    PercentileDisc,
    Pow,
    Power,
    Replace,
    Reverse,
    Rank,
    Round,
    Sign,
    Sin,
    Sqrt,
    Stddev,
    StddevIf,
    RPad,
    RTrim,
    Second,
    SecondAdd,
    SecondDiff,
    SecondSub,
    Substr,
    Substring,
    Sum,
    SumIf,
    Tan,
    Trim,
    TrimStart,
    TrimEnd,
    Upper,
    Year,
    YearAdd,
    YearDiff,
    YearSub,
    // endregion
    EOF, // Required for the layout algorithm
}

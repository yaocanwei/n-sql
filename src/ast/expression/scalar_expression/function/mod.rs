// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod cast_fn;
mod coalesce_fn;
mod datetime;
mod format_fn;
mod numeric;
mod nvl_fn;
mod string;

pub use self::cast_fn::*;
pub use self::coalesce_fn::*;
pub use self::datetime::*;
pub use self::format_fn::*;
pub use self::numeric::*;
pub use self::nvl_fn::*;
pub use self::string::*;

#[derive(Clone, Debug)]
pub enum Function {
    // region [numeric]
    Abs(AbsFn),
    Ceil(CeilFn),
    Cos(CosFn),
    DenseRank(DenseRankFn),
    Floor(FloorFn),
    Log10(Log10Fn),
    Log(LogFn),
    Pow(PowFn),
    Rank(RankFn),
    Round(RoundFn),
    Sin(SinFn),
    Sign(SignFn),
    Sqrt(SqrtFn),
    Tan(TanFn),
    // endregion
    String(StringFn),
    Cast(Box<CastFn>),
    Nvl(Box<NvlFn>),
    Datetime(DatetimeFn),
    Coalesce(CoalesceFn),
    Now,
}

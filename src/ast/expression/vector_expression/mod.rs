// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod aggregate_expression;

pub use self::aggregate_expression::*;

#[derive(Clone, Debug)]
pub enum VectorExpression {
    Avg(AvgFn),
    Count(CountFn),
    Max(MaxFn),
    Median(MedianFn),
    Min(MinFn),
    Sum(SumFn),
    Stddev(StddevFn),
    AvgIf(AvgIfFn),
    CountIf(CountIfFn),
    MaxIf(MaxIfFn),
    MedianIf(MedianIfFn),
    MinIf(MinIfFn),
    SumIf(SumIfFn),
    StddevIf(StddevIfFn),
    Percentile(PercentileFn),
}

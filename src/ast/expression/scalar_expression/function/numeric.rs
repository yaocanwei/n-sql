// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ast::{Expression, SortingDirection};

#[derive(Clone, Debug)]
pub struct AbsFn {
    pub expr: Box<Expression>,
}

impl AbsFn {
    pub fn new(expr: Box<Expression>) -> AbsFn {
        AbsFn { expr }
    }
}

#[derive(Clone, Debug)]
pub struct CeilFn {
    pub expr: Box<Expression>,
}

impl CeilFn {
    pub fn new(expr: Box<Expression>) -> CeilFn {
        CeilFn { expr }
    }
}

#[derive(Clone, Debug)]
pub struct CosFn {
    pub expr: Box<Expression>,
}

impl CosFn {
    pub fn new(expr: Box<Expression>) -> CosFn {
        CosFn { expr }
    }
}

#[derive(Clone, Debug)]
pub struct DenseRankFn {
    pub expr: Box<Expression>,
    pub order: Option<SortingDirection>,
}

impl DenseRankFn {
    pub fn new(expr: Box<Expression>, order: Option<SortingDirection>) -> DenseRankFn {
        DenseRankFn { expr, order }
    }
}

#[derive(Clone, Debug)]
pub struct FloorFn {
    pub expr: Box<Expression>,
}

impl FloorFn {
    pub fn new(expr: Box<Expression>) -> FloorFn {
        FloorFn { expr }
    }
}

#[derive(Clone, Debug)]
pub struct Log10Fn {
    pub expr: Box<Expression>,
}

impl Log10Fn {
    pub fn new(expr: Box<Expression>) -> Log10Fn {
        Log10Fn { expr }
    }
}

#[derive(Clone, Debug)]
pub struct LogFn {
    pub base: Option<Box<Expression>>,
    pub number: Box<Expression>,
}

impl LogFn {
    pub fn new(base: Option<Box<Expression>>, number: Box<Expression>) -> LogFn {
        LogFn { base, number }
    }
}

#[derive(Clone, Debug)]
pub struct PowFn {
    pub x: Box<Expression>,
    pub y: Box<Expression>,
}

impl PowFn {
    pub fn new(x: Box<Expression>, y: Box<Expression>) -> PowFn {
        PowFn { x, y }
    }
}

#[derive(Clone, Debug)]
pub struct RankFn {
    pub expr: Box<Expression>,
    pub order: Option<SortingDirection>,
}

impl RankFn {
    pub fn new(expr: Box<Expression>, order: Option<SortingDirection>) -> RankFn {
        RankFn { expr, order }
    }
}

#[derive(Clone, Debug)]
pub struct RoundFn {
    pub expr: Box<Expression>,
    pub precision: Option<Box<Expression>>,
}

impl RoundFn {
    pub fn new(expr: Box<Expression>, precision: Option<Box<Expression>>) -> RoundFn {
        RoundFn { expr, precision }
    }
}

#[derive(Clone, Debug)]
pub struct SignFn {
    pub expr: Box<Expression>,
}

impl SignFn {
    pub fn new(expr: Box<Expression>) -> SignFn {
        SignFn { expr }
    }
}

#[derive(Clone, Debug)]
pub struct SinFn {
    pub expr: Box<Expression>,
}

impl SinFn {
    pub fn new(expr: Box<Expression>) -> SinFn {
        SinFn { expr }
    }
}

#[derive(Clone, Debug)]
pub struct SqrtFn {
    pub expr: Box<Expression>,
}

impl SqrtFn {
    pub fn new(expr: Box<Expression>) -> SqrtFn {
        SqrtFn { expr }
    }
}

#[derive(Clone, Debug)]
pub struct TanFn {
    pub expr: Box<Expression>,
}

impl TanFn {
    pub fn new(expr: Box<Expression>) -> TanFn {
        TanFn { expr }
    }
}

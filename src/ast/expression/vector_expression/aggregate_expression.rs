// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ast::{Expression, PredicateExpression, SortingDirection};

#[derive(Clone, Debug)]
pub enum AggregateType {
    All,
    Distinct,
    Unique,
}

#[derive(Clone, Debug)]
pub struct AvgFn {
    pub expr: Box<Expression>,
    pub aggregate_type: Option<AggregateType>,
}

impl AvgFn {
    pub fn new(aggregate_type: Option<AggregateType>, expr: Box<Expression>) -> AvgFn {
        AvgFn {
            expr,
            aggregate_type,
        }
    }
}

#[derive(Clone, Debug)]
pub struct AvgIfFn {
    pub expr: Box<Expression>,
    pub predicate: Box<PredicateExpression>,
    pub aggregate_type: Option<AggregateType>,
}

impl AvgIfFn {
    pub fn new(
        predicate: Box<PredicateExpression>,
        aggregate_type: Option<AggregateType>,
        expr: Box<Expression>,
    ) -> AvgIfFn {
        AvgIfFn {
            predicate,
            expr,
            aggregate_type,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CountFn {
    pub expr: Box<Expression>,
    pub aggregate_type: Option<AggregateType>,
}

impl CountFn {
    pub fn new(aggregate_type: Option<AggregateType>, expr: Box<Expression>) -> CountFn {
        CountFn {
            expr,
            aggregate_type,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CountIfFn {
    pub expr: Box<Expression>,
    pub predicate: Box<PredicateExpression>,
    pub aggregate_type: Option<AggregateType>,
}

impl CountIfFn {
    pub fn new(
        predicate: Box<PredicateExpression>,
        aggregate_type: Option<AggregateType>,
        expr: Box<Expression>,
    ) -> CountIfFn {
        CountIfFn {
            predicate,
            expr,
            aggregate_type,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MaxFn {
    pub expr: Box<Expression>,
    pub aggregate_type: Option<AggregateType>,
}

impl MaxFn {
    pub fn new(aggregate_type: Option<AggregateType>, expr: Box<Expression>) -> MaxFn {
        MaxFn {
            expr,
            aggregate_type,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MaxIfFn {
    pub expr: Box<Expression>,
    pub predicate: Box<PredicateExpression>,
    pub aggregate_type: Option<AggregateType>,
}

impl MaxIfFn {
    pub fn new(
        predicate: Box<PredicateExpression>,
        aggregate_type: Option<AggregateType>,
        expr: Box<Expression>,
    ) -> MaxIfFn {
        MaxIfFn {
            predicate,
            expr,
            aggregate_type,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MedianFn {
    pub expr: Box<Expression>,
    pub aggregate_type: Option<AggregateType>,
}

impl MedianFn {
    pub fn new(aggregate_type: Option<AggregateType>, expr: Box<Expression>) -> MedianFn {
        MedianFn {
            expr,
            aggregate_type,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MedianIfFn {
    pub expr: Box<Expression>,
    pub predicate: Box<PredicateExpression>,
    pub aggregate_type: Option<AggregateType>,
}

impl MedianIfFn {
    pub fn new(
        predicate: Box<PredicateExpression>,
        aggregate_type: Option<AggregateType>,
        expr: Box<Expression>,
    ) -> MedianIfFn {
        MedianIfFn {
            predicate,
            expr,
            aggregate_type,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MinFn {
    pub expr: Box<Expression>,
    pub aggregate_type: Option<AggregateType>,
}

impl MinFn {
    pub fn new(aggregate_type: Option<AggregateType>, expr: Box<Expression>) -> MinFn {
        MinFn {
            expr,
            aggregate_type,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MinIfFn {
    pub expr: Box<Expression>,
    pub predicate: Box<PredicateExpression>,
    pub aggregate_type: Option<AggregateType>,
}

impl MinIfFn {
    pub fn new(
        predicate: Box<PredicateExpression>,
        aggregate_type: Option<AggregateType>,
        expr: Box<Expression>,
    ) -> MinIfFn {
        MinIfFn {
            predicate,
            expr,
            aggregate_type,
        }
    }
}

#[derive(Clone, Debug)]
pub struct StddevFn {
    pub expr: Box<Expression>,
    pub aggregate_type: Option<AggregateType>,
}

impl StddevFn {
    pub fn new(aggregate_type: Option<AggregateType>, expr: Box<Expression>) -> StddevFn {
        StddevFn {
            expr,
            aggregate_type,
        }
    }
}

#[derive(Clone, Debug)]
pub struct StddevIfFn {
    pub expr: Box<Expression>,
    pub predicate: Box<PredicateExpression>,
    pub aggregate_type: Option<AggregateType>,
}

impl StddevIfFn {
    pub fn new(
        predicate: Box<PredicateExpression>,
        aggregate_type: Option<AggregateType>,
        expr: Box<Expression>,
    ) -> StddevIfFn {
        StddevIfFn {
            predicate,
            expr,
            aggregate_type,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SumFn {
    pub expr: Box<Expression>,
    pub aggregate_type: Option<AggregateType>,
}

impl SumFn {
    pub fn new(aggregate_type: Option<AggregateType>, expr: Box<Expression>) -> SumFn {
        SumFn {
            expr,
            aggregate_type,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SumIfFn {
    pub expr: Box<Expression>,
    pub predicate: Box<PredicateExpression>,
    pub aggregate_type: Option<AggregateType>,
}

impl SumIfFn {
    pub fn new(
        predicate: Box<PredicateExpression>,
        aggregate_type: Option<AggregateType>,
        expr: Box<Expression>,
    ) -> SumIfFn {
        SumIfFn {
            predicate,
            expr,
            aggregate_type,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PercentileFn {
    pub expr: Box<Expression>,
    pub p: Box<Expression>,
    pub r#type: PercentileType,
    pub order: Option<SortingDirection>,
}

#[derive(Clone, Debug)]
pub enum PercentileType {
    Cont,
    Disc,
}

impl PercentileFn {
    pub fn new(
        expr: Box<Expression>,
        p: Box<Expression>,
        order: Option<SortingDirection>,
        tp: PercentileType,
    ) -> PercentileFn {
        PercentileFn {
            expr,
            p,
            order,
            r#type: tp,
        }
    }
}

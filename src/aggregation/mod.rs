mod max_aggregation;
mod min_aggregation;
mod sum_aggregation;
mod terms_aggregation;
mod top_hits_aggregation;
mod nested_aggregation;
mod filter_aggregation;
mod multi_terms_aggregation;

use crate::aggregation::max_aggregation::MaxAggregation;
use crate::aggregation::min_aggregation::MinAggregation;
use crate::aggregation::sum_aggregation::SumAggregation;
use crate::aggregation::terms_aggregation::TermsAggregation;
use crate::aggregation::top_hits_aggregation::TopHitsAggregation;
use serde_json::Value;
use crate::aggregation::filter_aggregation::FilterAggregation;
use crate::aggregation::multi_terms_aggregation::MultiTermsAggregation;
use crate::aggregation::nested_aggregation::NestedAggregation;

pub trait AggregationTrait {
    fn name(&self) -> &str;
    fn build(&self) -> Value;
    fn query_name(&self) -> String;
}

pub struct Aggregation {}

impl Aggregation {
    pub fn terms(name: &str) -> TermsAggregation {
        TermsAggregation::new(name)
    }
    pub fn multi_terms(name: &str) -> MultiTermsAggregation {
        MultiTermsAggregation::new(name)
    }
    pub fn top_hits(name: &str) -> TopHitsAggregation {
        TopHitsAggregation::new(name)
    }
    pub fn sum(name: &str) -> SumAggregation {
        SumAggregation::new(name)
    }
    pub fn max(name: &str) -> MaxAggregation {
        MaxAggregation::new(name)
    }
    pub fn min(name: &str) -> MinAggregation {
        MinAggregation::new(name)
    }
    pub fn nested(name: &str) -> NestedAggregation {
        NestedAggregation::new(name)
    }
    pub fn filter(name: &str) -> FilterAggregation {
        FilterAggregation::new(name)
    }
}

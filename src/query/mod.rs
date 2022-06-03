use serde_json::Value;

pub mod bool_query;
pub mod exists_query;
pub mod geo_distance_query;
pub mod match_query;
pub mod multi_match_query;
pub mod range_query;
pub mod term_query;
pub mod terms_query;
pub mod wildcard_query;

pub trait QueryTrait {
    fn build(&self) -> Value;
    fn query_name(&self) -> String;
}

trait MatchQueryTrait: QueryTrait {
    fn query_name() -> String;
}

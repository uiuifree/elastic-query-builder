use serde_json::json;
use elastic_query_builder::aggregation::Aggregation;
use elastic_query_builder::query::match_query::MatchQuery;
use elastic_query_builder::QueryBuilder;

#[test]
pub fn case01() {
    let mut builder = QueryBuilder::new();
    assert_eq!(r#"{"query":{"match_all":{}}}"#, builder.build().to_string());
    builder.set_script(json!({
    "source": "ctx._source.count++",
    "lang": "painless"
  }));
    assert_eq!(r#"{"query":{"match_all":{}},"script":{"lang":"painless","source":"ctx._source.count++"}}"#, builder.build().to_string());
}

#[test]
pub fn case02() {
    let mut builder = QueryBuilder::new();
    let filter = Aggregation::filter("filter").set_filter(MatchQuery::new("key", "value"));
    let agg = Aggregation::nested("name").set_path("city").append_aggregation(filter);
    builder.set_aggregation(vec![
        agg
    ]);
    assert_eq!(builder.build().to_string(),
               r#"{"aggs":{"name":{"aggs":{"filter":{"filter":{"match":{"key":{"query":"value"}}}}},"nested":{"path":"city"}}},"query":{"match_all":{}}}"#);
}
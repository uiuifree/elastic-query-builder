use serde_json::json;
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
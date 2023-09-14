## ElasticQueryBuilder
![Crates.io](https://img.shields.io/crates/v/elastic_query_builder?style=flat-square)
![GitHub](https://img.shields.io/github/license/openai-rs/openai-api?style=flat-square)

ElasticQueryBuilder is a query builder designed to easily build ElasticSearch related DSLs.



## install
```toml
# Cargo.toml
[dependencies]
elastic-query-builder ="0.1"
```

## Futures
* Mapping
* DML Query
* Aggregation


## Mapping
```
let mut mapping = MappingBuilder::new();
mapping
    .add_property("title",KeywordFieldType::new())
    .add_property("content",TextFieldType::new());
assert_eq!!(
    mapping.build().to_string(),
    r#"{"mappings":{"properties":{"content":{"type":"text"},"title":{"type":"keyword"}}}}"#
    );

```


# DML Query

### Build Json Query
```rust
fn main() {
    let mut query = QueryBuilder::new();
    query.set_query(MatchQuery::new("field", "value"));
    query.set_from(10);
    query.set_size(100);
    let value_json = query.build();
}
```


### Providing Query
* bool_query
* exists_query
* geo_distance_query
* match_all_query
* match_query
* multi_match_query
* nested
* range_query
* script_query
* script_score_query
* term_query
* terms_query
* wildcard_query



### BoolQuery Example
```rust
fn main() {
    let mut query = QueryBuilder::new();
    let mut bool = BoolQuery::new();
    bool.add_must(MatchQuery::new("field1", "value"));
    bool.add_must_not(MatchQuery::new("field2", "value"));
    bool.add_should(MatchQuery::new("field3", "value"));
    query.set_query(bool);
}
```
### Nested Example
```rust
fn main() {
    let mut query = QueryBuilder::new();
    let nested = NestedQuery::new("locations", MatchQuery::new("locations.country", "JP"));
    query.set_query(nested);
}
```


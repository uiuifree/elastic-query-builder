## ElasticQueryBuilder
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
```
use elastic_query_builder::query::bool_query::BoolQuery;
use elastic_query_builder::query::match_query::MatchQuery;
use elastic_query_builder::QueryBuilder;
use serde_json::{Value};
let mut query = QueryBuilder::new();
let mut bool = BoolQuery::new();
bool.add_must(MatchQuery::new("field","value"));
bool.add_must(MatchQuery::new("field2","value2"));
query.set_query(bool);
let value:Value = query.build();
```

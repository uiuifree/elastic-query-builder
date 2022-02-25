[GitHub Page](https://github.com/uiuifree/elastic-query-builder)

# example

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

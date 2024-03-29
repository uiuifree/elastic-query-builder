extern crate core;

use crate::aggregation::AggregationTrait;
use crate::query::QueryTrait;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use serde_json::{json, Value};

pub mod aggregation;
pub mod mapping;
pub mod query;
pub(crate) mod util;

extern crate serde_json;

#[derive(Default, Clone)]
pub struct QueryBuilder {
    query: Value,
    aggs: Value,
    size: i64,
    from: i64,
    scroll: String,
    sort: Value,
    source: Vec<String>,
    script: Value,
}

/// # example
///
/// ```
/// use elastic_query_builder::query::bool_query::BoolQuery;
/// use elastic_query_builder::query::match_query::MatchQuery;
/// use elastic_query_builder::QueryBuilder;
/// use serde_json::{Value};
/// let mut query = QueryBuilder::new();
/// let mut bool = BoolQuery::new();
/// bool.add_must(MatchQuery::new("field","value"));
/// bool.add_must(MatchQuery::new("field2","value2"));
/// query.set_query(bool);
/// let value:Value = query.build();
/// ```
impl QueryBuilder {
    pub fn new() -> QueryBuilder {
        let mut val = QueryBuilder::default();
        val.size = 10;
        val.from = 0;
        return val;
    }
    pub fn set_query<T>(&mut self, query: T) -> &QueryBuilder
    where
        T: QueryTrait,
    {
        self.query = query.build();
        return self;
    }
    pub fn set_query_from_value(&mut self, query: Value) -> &QueryBuilder {
        self.query = query;
        return self;
    }
    pub fn set_aggregation<T>(&mut self, query: Vec<T>) -> &QueryBuilder
    where
        T: AggregationTrait,
    {
        let mut values = Value::default();

        for q in query {
            merge(&mut values, &q.build());
        }
        self.aggs = json!(values);
        return self;
    }

    pub fn append_aggregation<T>(&mut self, query: T) -> &QueryBuilder
    where
        T: AggregationTrait,
    {
        let values = self.aggs.clone();

        let mut values = serde_json::from_value::<Value>(values).unwrap();
        merge(&mut values, &query.build());

        self.aggs = json!(values);
        return self;
    }

    pub fn set_size(&mut self, size: i64) -> &QueryBuilder {
        self.size = size;
        return self;
    }
    pub fn set_from(&mut self, from: i64) -> &QueryBuilder {
        self.from = from;
        return self;
    }
    pub fn set_scroll(&mut self, value: &str) -> &QueryBuilder {
        self.scroll = value.to_string();
        return self;
    }
    pub fn set_sort(&mut self, value: Value) -> &QueryBuilder {
        self.sort = value;
        return self;
    }
    pub fn set_source(&mut self, value: Vec<String>) -> &QueryBuilder {
        self.source = value;
        return self;
    }
    pub fn set_script(&mut self, value: Value) -> &QueryBuilder {
        self.script = value;
        return self;
    }

    pub fn get_size(&self) -> i64 {
        self.size.clone()
    }
    pub fn get_scroll(&self) -> &str {
        self.scroll.as_str()
    }

    pub fn get_from(&self) -> i64 {
        self.from.clone()
    }
    pub fn get_sort(&self) -> &Value {
        &self.sort
    }
    pub fn get_script(&self) -> &Value {
        &self.script
    }

    pub fn build(&self) -> Value {
        json!(self)
    }
}

impl Serialize for QueryBuilder {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("QueryBuilder", 0)?;
        if !self.source.is_empty() {
            let _ = state.serialize_field("_source", &self.source);
        }
        if self.query.is_null() || self.query.to_string().is_empty() {
            let _ = state.serialize_field("query", &json!({"match_all":{}}));
        } else {
            let _ = state.serialize_field("query", &self.query);
        }
        if !(self.aggs.is_null() || self.query.to_string().is_empty()) {
            let _ = state.serialize_field("aggs", &self.aggs);
        }
        if !(self.script.is_null() || self.script.to_string().is_empty()) {
            let _ = state.serialize_field("script", &self.script);
        }
        if !(self.sort.is_null() || self.sort.to_string().is_empty()) {
            let _ = state.serialize_field("sort", &self.sort);
        }
        state.end()
    }
}

pub(crate) fn merge(a: &mut Value, b: &Value) {
    match (a, b) {
        (&mut Value::Object(ref mut a), &Value::Object(ref b)) => {
            for (k, v) in b {
                merge(a.entry(k.clone()).or_insert(Value::Null), v);
            }
        }
        (a, b) => {
            *a = b.clone();
        }
    }
}

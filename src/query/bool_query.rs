use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use serde_json::{json, Value};

use crate::query::QueryTrait;

#[derive(Default)]
pub struct BoolQuery {
    must: Vec<Value>,
    must_not: Vec<Value>,
    should: Vec<Value>,
    filter: Vec<Value>,
}

impl BoolQuery {
    pub fn new() -> BoolQuery {
        BoolQuery::default()
    }
    pub fn add_must<T>(&mut self, value: T)
        where
            T: QueryTrait,
    {
        self.must.push(value.build());
    }
    pub fn add_must_not<T>(&mut self, value: T)
        where
            T: QueryTrait,
    {
        self.must_not.push(value.build());
    }
    pub fn add_should<T>(&mut self, value: T)
        where
            T: QueryTrait,
    {
        self.should.push(value.build());
    }
    pub fn add_filter<T>(&mut self, value: T)
        where
            T: QueryTrait, {
        self.filter.push(value.build());
    }
    pub fn is_empty(&self) -> bool {
        self.must.is_empty()
            && self.must_not.is_empty()
            && self.should.is_empty()
            && self.filter.is_empty()
    }
}

impl Serialize for BoolQuery {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("BoolQuery", 0)?;
        if !self.must.is_empty() {
            let _ = state.serialize_field("must", &self.must);
        }
        if !self.must_not.is_empty() {
            let _ = state.serialize_field("must_not", &self.must_not);
        }
        if !self.should.is_empty() {
            let _ = state.serialize_field("should", &self.should);
        }
        if !self.filter.is_empty() {
            let _ = state.serialize_field("filter", &self.filter);
        }
        state.end()
    }
}

impl QueryTrait for BoolQuery {
    fn build(&self) -> Value {
        let name = self.query_name();
        json!({ name: self })
    }
    fn query_name(&self) -> String {
        return "bool".to_string();
    }
}

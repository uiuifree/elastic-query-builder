use std::collections::HashMap;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use serde_json::{json, Value};
use crate::query::QueryTrait;

#[derive(Default)]
pub struct RangeQuery {
    field: String,
    lt: String,
    gt: String,
    lte: String,
    gte: String,
}


impl RangeQuery {
    pub fn new(field: &str) -> RangeQuery {
        let mut v = RangeQuery::default();
        v.field = field.to_string();
        return v;
    }
    pub fn gt(mut self, value: String)->RangeQuery {
        self.gt = value;
        return self;
    }
    pub fn lt(mut self, value: String)->RangeQuery {
        self.lt = value;
        return self;
    }
    pub fn gte(mut self, value: String)->RangeQuery {
        self.gte = value;
        return self;
    }
    pub fn lte(mut self, value: String)->RangeQuery {
        self.gte = value;
        return self;
    }
}

impl Serialize for RangeQuery {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("RangeQuery", 0)?;
        if !self.gt.is_empty() {
            state.serialize_field("gt", &self.gt);
        }
        if !self.lt.is_empty() {
            state.serialize_field("lt", &self.lt);
        }
        if !self.gte.is_empty() {
            state.serialize_field("gte", &self.gte);
        }
        if !self.lte.is_empty() {
            state.serialize_field("lte", &self.lte);
        }
        state.end()
    }
}

impl QueryTrait for RangeQuery {
    fn build(&self) -> Value {
        let name = self.query_name();
        let field = self.field.to_string();
        json!({
            name: {
                field:self
            }
        })
    }
    fn query_name(&self) -> String {
        return "range".to_string();
    }
}

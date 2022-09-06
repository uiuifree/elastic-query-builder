use crate::aggregation::AggregationTrait;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use serde_json::{json, Value};
use crate::QueryTrait;

#[derive(Default)]
pub struct FilterAggregation {
    name: String,
    filter: Value,
    aggregation: Value,
}


impl FilterAggregation {
    pub fn new(name: &str) -> Self {
        FilterAggregation {
            name: name.to_string(),
            ..Default::default()
        }
    }

    pub fn set_filter<T>(mut self, value: T)-> Self
        where
            T: QueryTrait,
    {
        self.filter = value.build();
        self

    }
    pub fn set_aggregation<T>(mut self, aggregation: T) -> Self
        where
            T: AggregationTrait,
    {
        self.aggregation = aggregation.build();
        self
    }
}


impl Serialize for FilterAggregation {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("FilterAggregation", 0)?;
        if !(self.filter.is_null() || self.filter.to_string().is_empty()) {
            state.serialize_field("filter", &self.filter)?;
        }
        if !(self.aggregation.is_null() || self.aggregation.to_string().is_empty()) {
            state.serialize_field("aggs", &self.aggregation)?;
        }
        state.end()
    }
}

impl AggregationTrait for FilterAggregation {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn build(&self) -> Value {
        let name = self.name.to_string();
        json!({ name: self })
    }

    fn query_name(&self) -> String {
        "filter".to_string()
    }
}


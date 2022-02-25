use std::collections::HashMap;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use serde_json::{json, Value};
use crate::aggregation::AggregationTrait;

#[derive(Default)]
pub struct MinAggregation {
    name: String,
    value: MinValue,
    aggregation: Value,
}

#[derive(Default)]
struct MinValue {
    field: String,
    script: String,
    missing: i64,
}

impl MinAggregation {
    pub fn new(name: &str) -> Self {
        MinAggregation {
            name: name.to_string(),
            ..Default::default()
        }
    }

    pub fn set_field(mut self, field: &str) -> Self {
        self.value.field = field.to_string();
        self
    }

    pub fn set_script(mut self, script: &str) -> Self {
        self.value.script = script.to_string();
        self
    }
    pub fn set_missing(mut self, missing: i64) -> Self {
        self.value.missing = missing;
        self
    }
    pub fn set_aggregation<T>(mut self, aggregation: T) -> Self where T: AggregationTrait {
        self.aggregation = aggregation.build();
        self
    }
}

impl Serialize for MinValue {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("MinValue", 0)?;

        if !self.field.is_empty() {
            state.serialize_field("field", &self.field)?;
        }
        if self.missing != 0 {
            state.serialize_field("missing", &self.missing)?;
        }
        state.end()
    }
}

impl Serialize for MinAggregation {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("MinAggregation", 0)?;
        state.serialize_field("min", &self.value)?;
        if !(self.aggregation.is_null() || self.aggregation.to_string().is_empty()) {
            state.serialize_field("aggs", &self.aggregation)?;
        }
        state.end()
    }
}


impl AggregationTrait for MinAggregation {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn build(&self) -> Value {
        let name = self.name.to_string();
        json!({
            name : self
        })
    }

    fn query_name(&self) -> String {
        "min".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aggregation::AggregationTrait;

    #[test]
    fn test_terms_aggregation() {
        let agg = MinAggregation::new("hoge")
            .add_sort("hoge", "asc");


        let json = agg.build();
        println!("{}", json);
        // assert_eq!(json["test"]["terms"]["field"], "test");
        // assert_eq!(json["test"]["terms"]["size"], 1);
    }
}

use std::collections::HashMap;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use serde_json::{json, Value};
use crate::aggregation::AggregationTrait;

#[derive(Default)]
pub struct SumAggregation {
    name: String,
    value: SumValue,
    aggregation: Value,
}

#[derive(Default)]
struct SumValue {
    field: String,
    script: String,
    missing: i64,
}

impl SumAggregation {
    pub fn new(name: &str) -> Self {
        SumAggregation {
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

impl Serialize for SumValue {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("SumValue", 0)?;

        if !self.field.is_empty() {
            state.serialize_field("field", &self.field)?;
        }
        if self.missing != 0 {
            state.serialize_field("missing", &self.missing)?;
        }
        state.end()
    }
}

impl Serialize for SumAggregation {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("SumAggregation", 0)?;
        state.serialize_field("sum", &self.value)?;
        if !(self.aggregation.is_null() || self.aggregation.to_string().is_empty()) {
            state.serialize_field("aggs", &self.aggregation)?;
        }
        state.end()
    }
}


impl AggregationTrait for SumAggregation {
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
        "sum".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aggregation::AggregationTrait;

    #[test]
    fn test_terms_aggregation() {
        let agg = SumAggregation::new("hoge")
            .add_sort("hoge", "asc");


        let json = agg.build();
        println!("{}", json);
        // assert_eq!(json["test"]["terms"]["field"], "test");
        // assert_eq!(json["test"]["terms"]["size"], 1);
    }
}

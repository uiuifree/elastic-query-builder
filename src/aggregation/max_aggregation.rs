use crate::aggregation::AggregationTrait;
use crate::merge;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use serde_json::{json, Value};

#[derive(Default)]
pub struct MaxAggregation {
    name: String,
    value: MaxValue,
    aggregation: Value,
}

#[derive(Default)]
struct MaxValue {
    field: String,
    script: String,
    format: String,
    missing: i64,
}

impl MaxAggregation {
    pub fn new(name: &str) -> Self {
        MaxAggregation {
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
    pub fn set_format(mut self, fmt: &str) -> Self {
        self.value.format = fmt.to_string();
        self
    }
    pub fn set_aggregation<T>(mut self, aggregation: T) -> Self
    where
        T: AggregationTrait,
    {
        self.aggregation = aggregation.build();
        self
    }
    pub fn append_aggregation<T>(mut self, query: T) -> Self
    where
        T: AggregationTrait,
    {
        let mut values = self.aggregation.clone();
        merge(&mut values, &query.build());
        self.aggregation = json!(values);
        return self;
    }
}

impl Serialize for MaxValue {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("MaxValue", 0)?;

        if !self.field.is_empty() {
            state.serialize_field("field", &self.field)?;
        }
        if !self.format.is_empty() {
            state.serialize_field("format", &self.format)?;
        }
        if self.missing != 0 {
            state.serialize_field("missing", &self.missing)?;
        }
        state.end()
    }
}

impl Serialize for MaxAggregation {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("MaxAggregation", 0)?;
        state.serialize_field("max", &self.value)?;
        if !(self.aggregation.is_null() || self.aggregation.to_string().is_empty()) {
            state.serialize_field("aggs", &self.aggregation)?;
        }
        state.end()
    }
}

impl AggregationTrait for MaxAggregation {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn build(&self) -> Value {
        let name = self.name.to_string();
        json!({ name: self })
    }

    fn query_name(&self) -> String {
        "max".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aggregation::AggregationTrait;

    #[test]
    fn test_terms_aggregation() {
        let agg = MaxAggregation::new("hoge");

        let json = agg.build();
        println!("{}", json);
        // assert_eq!(json["test"]["terms"]["field"], "test");
        // assert_eq!(json["test"]["terms"]["size"], 1);
    }
}

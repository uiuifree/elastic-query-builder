use crate::aggregation::AggregationTrait;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use serde_json::{json, Value};

#[derive(Default)]
pub struct NestedAggregation {
    name: String,
    value: NestedValue,
    aggregation: Value,
}

#[derive(Default)]
struct NestedValue {
    path: String,
}

impl NestedAggregation {
    pub fn new(name: &str) -> Self {
        let mut term = NestedAggregation {
            name: name.to_string(),
            ..Default::default()
        };
        term
    }
    pub fn set_path(mut self, path: &str) -> Self {
        self.value.path = path.to_string();
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

impl Serialize for NestedValue {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("TermsBuilder", 0)?;


        if !self.path.is_empty() {
            state.serialize_field("path", &self.path)?;
        }
        state.end()
    }
}

impl Serialize for NestedAggregation {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("BoolQuery", 0)?;
        state.serialize_field("nested", &self.value)?;
        if !(self.aggregation.is_null() || self.aggregation.to_string().is_empty()) {
            state.serialize_field("aggs", &self.aggregation)?;
        }
        // }
        state.end()
    }
}



impl AggregationTrait for NestedAggregation {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn build(&self) -> Value {
        let name = self.name.to_string();
        json!({ name: self })
    }

    fn query_name(&self) -> String {
        "nested".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aggregation::AggregationTrait;
    use crate::aggregation::terms_aggregation::TermsAggregation;

    #[test]
    fn test_nested_aggregation() {
        let agg = NestedAggregation::new("hoge")
            .set_path("path")
            .set_aggregation(NestedAggregation::new("agg"));

        let json = agg.build();
        println!("{}", json);
        let agg = NestedAggregation::new("hoge")
            .set_path("path")
            .set_aggregation(TermsAggregation::new("agg").set_field("key"));

        let json = agg.build();
        println!("{}", json);
        // assert_eq!(json["test"]["nested"]["field"], "test");
        // assert_eq!(json["test"]["nested"]["size"], 1);
    }
}

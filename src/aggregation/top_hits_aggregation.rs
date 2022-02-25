use std::collections::HashMap;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use serde_json::{json, Value};
use crate::aggregation::AggregationTrait;

#[derive(Default)]
pub struct TopHitsAggregation {
    name: String,
    value: TopHitsValue,
    aggregation: Value,
}

#[derive(Default)]
struct TopHitsValue {
    script: String,
    size: i64,
    sort: Vec<(String, String)>,
}

impl TopHitsAggregation {
    pub fn new(name: &str) -> Self {
        let mut term = TopHitsAggregation {
            name: name.to_string(),
            ..Default::default()
        };
        term.value.size = 10;
        term
    }
    pub fn add_sort(mut self, field: &str, order: &str) -> Self {
        self.value.sort.push((field.to_string(), order.to_string()));
        self
    }
    pub fn set_size(mut self, size: i64) -> Self {
        self.value.size = size;
        self
    }
    pub fn set_script(mut self, script: &str) -> Self {
        self.value.script = script.to_string();
        self
    }
    pub fn set_aggregation<T>(mut self, aggregation: T) -> Self where T: AggregationTrait {
        self.aggregation = aggregation.build();
        self
    }
}

impl Serialize for TopHitsValue {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("TopHitsBuilder", 0)?;

        if self.script.is_empty() {
            state.serialize_field("size", &self.size)?;
            if !self.sort.is_empty() {
                let mut sort_map = HashMap::new();
                for (field, order) in self.sort.iter() {
                    sort_map.insert(field.to_string(), order.to_string());
                }
                state.serialize_field("sort", &sort_map)?;
            }
        } else {
            let mut source = HashMap::new();
            source.insert("includes", self.script.clone());
            state.serialize_field("_source", &source)?;
            state.serialize_field("size", &self.size)?;
            // TODO:
        }
        state.end()
    }
}

impl Serialize for TopHitsAggregation {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("TopHitsAggregation", 0)?;
        state.serialize_field("top_hits", &self.value)?;
        if !(self.aggregation.is_null() || self.aggregation.to_string().is_empty()) {
            state.serialize_field("aggs", &self.aggregation)?;
        }
        // }
        state.end()
    }
}


impl AggregationTrait for TopHitsAggregation {
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
        "top_hits".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aggregation::AggregationTrait;

    #[test]
    fn test_terms_aggregation() {
        let agg = TopHitsAggregation::new("hoge")
            .add_sort("hoge", "asc");


        let json = agg.build();
        println!("{}", json);
        // assert_eq!(json["test"]["terms"]["field"], "test");
        // assert_eq!(json["test"]["terms"]["size"], 1);
    }
}

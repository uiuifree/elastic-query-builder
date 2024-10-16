use crate::aggregation::AggregationTrait;
use crate::merge;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use serde_json::{json, Map, Value};

#[derive(Default)]
pub struct ReverseNestedAggregation {
    name: String,
    aggregation: Value,
}



impl ReverseNestedAggregation {
    pub fn new(name: &str) -> Self {
        let term = ReverseNestedAggregation {
            name: name.to_string(),
            ..Default::default()
        };
        term
    }

    pub fn append_aggregation<T>(mut self, query: T) -> Self
    where
        T: AggregationTrait,
    {
        let mut values = self.aggregation.clone();
        merge(&mut values, &query.build());
        self.aggregation = json!(values);
        self
    }
}


impl Serialize for ReverseNestedAggregation {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("BoolQuery", 0)?;
        state.serialize_field("reverse_nested", &Value::Object(Map::new()))?;
        if !(self.aggregation.is_null() || self.aggregation.to_string().is_empty()) {
            state.serialize_field("aggs", &self.aggregation)?;
        }
        // }
        state.end()
    }
}

impl AggregationTrait for ReverseNestedAggregation {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn build(&self) -> Value {
        let name = self.name.to_string();
        json!({ name: self })
    }

    fn query_name(&self) -> String {
        "reverse_nested".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aggregation::AggregationTrait;
    use crate::aggregation::max_aggregation::MaxAggregation;

    #[test]
    fn test_nested_aggregation() {
        let agg = ReverseNestedAggregation::new("hoge")
            .append_aggregation(MaxAggregation::new("updated_at").set_field("updated_at"));

        let json = agg.build();

        println!("{}", json);
        // assert_eq!(json["test"]["nested"]["field"], "test");
        // assert_eq!(json["test"]["nested"]["size"], 1);
    }
}

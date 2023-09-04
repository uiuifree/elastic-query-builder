use crate::aggregation::AggregationTrait;
use crate::merge;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use serde_json::{json, Value};

#[derive(Default)]
pub struct ValueCountAggregation {
    name: String,
    value: SumValue,
    aggregation: Value,
}

#[derive(Default)]
struct SumValue {
    field: String,
    script: String,
}

impl ValueCountAggregation {
    pub fn new(name: &str) -> Self {
        ValueCountAggregation {
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
        merge(&mut self.aggregation, &query.build());
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
        state.end()
    }
}

impl Serialize for ValueCountAggregation {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ValueCountAggregation", 0)?;
        state.serialize_field("value_count", &self.value)?;
        if !(self.aggregation.is_null() || self.aggregation.to_string().is_empty()) {
            state.serialize_field("aggs", &self.aggregation)?;
        }
        state.end()
    }
}

impl AggregationTrait for ValueCountAggregation {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn build(&self) -> Value {
        let name = self.name.to_string();
        json!({ name: self })
    }

    fn query_name(&self) -> String {
        "value_count".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aggregation::AggregationTrait;

    #[test]
    fn test_terms_aggregation() {
        let agg = ValueCountAggregation::new("hoge").set_field("set_field");
        let json = agg.build();
        assert_eq!(
            json.to_string(),
            "{\"hoge\":{\"value_count\":{\"field\":\"set_field\"}}}"
        )
    }
}

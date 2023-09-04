use crate::aggregation::AggregationTrait;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use serde_json::{json, Value};

#[derive(Default)]
pub struct CardinalityAggregation {
    name: String,
    value: CardinalityValue,
}

#[derive(Default)]
struct CardinalityValue {
    field: String,
}

impl CardinalityAggregation {
    pub fn new(name: &str) -> Self {
        CardinalityAggregation {
            name: name.to_string(),
            ..Default::default()
        }
    }

    pub fn set_field(mut self, field: &str) -> Self {
        self.value.field = field.to_string();
        self
    }
}

impl Serialize for CardinalityValue {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("CardinalityValue", 0)?;
        if !self.field.is_empty() {
            state.serialize_field("field", &self.field)?;
        }

        state.end()
    }
}

impl Serialize for CardinalityAggregation {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("CardinalityAggregation", 0)?;
        state.serialize_field("cardinality", &self.value)?;

        state.end()
    }
}

impl AggregationTrait for CardinalityAggregation {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn build(&self) -> Value {
        let name = self.name.to_string();
        json!({ name: self })
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
        let agg = CardinalityAggregation::new("hoge").set_field("id");
        let json = agg.build();
        println!("{}", json);
        // assert_eq!(json["test"]["terms"]["field"], "test");
        // assert_eq!(json["test"]["terms"]["size"], 1);
    }
}

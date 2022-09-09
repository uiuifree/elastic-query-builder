use crate::aggregation::AggregationTrait;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use serde_json::{json, Value};

#[derive(Default)]
pub struct MultiTermsAggregation {
    name: String,
    // aggregation: Option<T>,
    value: TermsValue,
    aggregation: Value,
}

#[derive(Default, Clone)]
struct MultiTermsOrder {
    order_field: String,
    order_value: String,
}

#[derive(Default)]
struct TermsValue {
    script: String,
    fields: Vec<String>,
    size: i64,
    order: Option<MultiTermsOrder>,
}

impl MultiTermsAggregation {
    pub fn new(name: &str) -> Self {
        let mut term = MultiTermsAggregation {
            name: name.to_string(),
            ..Default::default()
        };
        term.value.size = 10;
        term
    }
    pub fn set_fields(mut self, fields: Vec<String>) -> Self {
        self.value.fields = fields;
        self
    }
    pub fn set_size(mut self, size: i64) -> Self {
        self.value.size = size;
        self
    }

    pub fn set_order(mut self, order_field: &str, order_value: &str) -> Self {
        self.value.order = Some(MultiTermsOrder {
            order_field: order_field.to_string(),
            order_value: order_value.to_string(),
        });
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

impl Serialize for TermsValue {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("MultiTermsBuilder", 0)?;

        if self.script.is_empty() {
            let mut values = vec![];
            for field in &self.fields{
                values.push(json!({"field":field}))
            }
            state.serialize_field("terms", &json!(values))?;
            state.serialize_field("size", &self.size)?;
        } else {
            // TODO:
        }

        if self.order.is_some() {
            let order = self.order.clone().unwrap();
            let name = order.order_field;
            let value = order.order_value;
            state.serialize_field("order", &json!({name:value}))?;
        }
        state.end()
    }
}
//
// impl Serialize for MultiTermsOrder {
//     fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
//     where
//         S: Serializer,
//     {
//         let mut state = serializer.serialize_struct("MultiTermsOrder", 0)?;
//
//         state.serialize_field("order_field", &self.order_field)?;
//         state.serialize_field("order_value", &self.order_value)?;
//         state.end()
//     }
// }

impl Serialize for MultiTermsAggregation {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("MultiTerum", 0)?;
        state.serialize_field("multi_terms", &self.value)?;
        if !(self.aggregation.is_null() || self.aggregation.to_string().is_empty()) {
            state.serialize_field("aggs", &self.aggregation)?;
        }
        // }
        state.end()
    }
}

// impl Serialize for MultiTermsOrder {
//     fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
//         where
//             S: Serializer,
//     {
//         let mut state = serializer.serialize_struct("BoolQuery", 0)?;
//         state.serialize_field(&self.order_field.to_string(), &self.order_value.to_string());
//         state.end()
//     }
// }

impl AggregationTrait for MultiTermsAggregation {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn build(&self) -> Value {
        let name = self.name.to_string();
        json!({ name: self })
    }

    fn query_name(&self) -> String {
        "terms".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aggregation::AggregationTrait;

    #[test]
    fn test_terms_aggregation() {
        let agg = MultiTermsAggregation::new("hoge")
            .set_fields(vec![
                "field_name1".to_string(),
                "field_name2".to_string()
            ])
            .set_order("_key","asc")
;

        let json = agg.build();
        println!("{}", json);
        // assert_eq!(json["test"]["terms"]["field"], "test");
        // assert_eq!(json["test"]["terms"]["size"], 1);
    }
}

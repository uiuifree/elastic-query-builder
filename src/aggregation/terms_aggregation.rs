use crate::aggregation::AggregationTrait;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use serde_json::{json, Value};
use crate::merge;

#[derive(Default)]
pub struct TermsAggregation {
    name: String,
    // aggregation: Option<T>,
    value: TermsValue,
    aggregation: Value,
}

#[derive(Default, Clone)]
struct TermsOrder {
    order_field: String,
    order_value: String,
}

#[derive(Default)]
struct TermsValue {
    script: String,
    field: String,
    size: i64,
    order: Option<TermsOrder>,
    include: String,
    exclude: String,
}

impl TermsAggregation {
    pub fn new(name: &str) -> Self {
        let mut term = TermsAggregation {
            name: name.to_string(),
            ..Default::default()
        };
        term.value.size = 10;
        term
    }
    pub fn set_field(mut self, field: &str) -> Self {
        self.value.field = field.to_string();
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
    pub fn set_order(mut self, order_field: &str, order_value: &str) -> Self {
        self.value.order = Some(TermsOrder {
            order_field: order_field.to_string(),
            order_value: order_value.to_string(),
        });
        self
    }
    pub fn set_include(mut self, include: &str) -> Self {
        self.value.include = include.to_string();
        self
    }
    pub fn set_exclude(mut self, exclude: &str) -> Self {
        self.value.exclude = exclude.to_string();
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

impl Serialize for TermsValue {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("TermsBuilder", 0)?;

        if self.script.is_empty() {
            state.serialize_field("field", &self.field)?;
            state.serialize_field("size", &self.size)?;
        } else {
            // TODO:
        }

        if self.order.is_some() {
            state.serialize_field("order", &self.order.as_ref().unwrap())?;
        }
        if !self.include.is_empty() {
            state.serialize_field("include", &self.include)?;
        }
        if !self.exclude.is_empty() {
            state.serialize_field("exclude", &self.exclude)?;
        }
        state.end()
    }
}

impl Serialize for TermsOrder {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("TermsOrder", 0)?;

        state.serialize_field("order_field", &self.order_field)?;
        state.serialize_field("order_value", &self.order_value)?;
        state.end()
    }
}

impl Serialize for TermsAggregation {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("BoolQuery", 0)?;
        state.serialize_field("terms", &self.value)?;
        if !(self.aggregation.is_null() || self.aggregation.to_string().is_empty()) {
            state.serialize_field("aggs", &self.aggregation)?;
        }
        // }
        state.end()
    }
}

// impl Serialize for TermsOrder {
//     fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
//         where
//             S: Serializer,
//     {
//         let mut state = serializer.serialize_struct("BoolQuery", 0)?;
//         state.serialize_field(&self.order_field.to_string(), &self.order_value.to_string());
//         state.end()
//     }
// }

impl AggregationTrait for TermsAggregation {
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
        let agg = TermsAggregation::new("hoge")
            .set_field("field_name")
            .set_aggregation(TermsAggregation::new("agg"));

        let json = agg.build();
        println!("{}", json);
        // assert_eq!(json["test"]["terms"]["field"], "test");
        // assert_eq!(json["test"]["terms"]["size"], 1);
    }
}

use crate::query::QueryTrait;
use crate::util::UtilMap;
use serde_json::{json, Value};

#[derive(Default)]
pub struct TermQuery {
    field: String,
    value: String,
    boost: Option<f64>,
}

impl TermQuery {
    pub fn new(field: &str, value: &str) -> TermQuery {
        let mut query = TermQuery::default();
        query.field = field.to_string();
        query.value = value.to_string();
        return query;
    }
    pub fn set_boost(mut self, boost: f64) -> TermQuery {
        self.boost = Some(boost);
        self
    }
}

impl QueryTrait for TermQuery {
    fn build(&self) -> Value {
        let mut query = UtilMap::new();
        query.append_string("value", self.value.to_string());
        query.append_boost(self.boost);

        let mut root = UtilMap::new();
        root.append_object(self.field.to_string(), query);
        root.build_object(self.query_name())
    }

    fn query_name(&self) -> String {
        return "term".to_string();
    }
}

use crate::query::QueryTrait;
use serde_json::{json, Value};

#[derive(Default)]
pub struct TermQuery {
    field: String,
    value: String,
}

impl TermQuery {
    pub fn new(field: &str, value: &str) -> TermQuery {
        let mut query = TermQuery::default();
        query.field = field.to_string();
        query.value = value.to_string();
        return query;
    }
}

impl QueryTrait for TermQuery {
    fn build(&self) -> Value {
        let field = self.field.to_string();
        let query = self.value.to_string();
        json!({ field: query })
    }

    fn query_name(&self) -> String {
        return "term".to_string();
    }
}

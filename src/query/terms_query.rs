use serde_json::{json, Value};
use crate::query::QueryTrait;

#[derive(Default)]
pub struct TermsQuery {
    field: String,
    values: Vec<String>,
}

impl TermsQuery {
    pub fn new(field: &str, values: Vec<String>) -> TermsQuery {
        let mut query = TermsQuery::default();
        query.field = field.to_string();
        query.values = values;
        return query;
    }
}

impl QueryTrait for TermsQuery {
    fn build(&self) -> Value {
        let field = self.field.to_string();
        let query = self.values.clone();
        json!({
            field:query
        })
    }

    fn query_name(&self) -> String {
        return "terms".to_string();
    }
}

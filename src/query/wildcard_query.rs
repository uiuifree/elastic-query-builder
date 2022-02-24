use serde_json::{json, Value};
use crate::query::QueryTrait;

#[derive(Default)]
pub struct WildcardQuery {
    field: String,
    value: String,
}

impl WildcardQuery {
    pub fn new(field: &str, value: &str) -> WildcardQuery {
        let mut query = WildcardQuery::default();
        query.field = field.to_string();
        query.value = value.to_string();
        return query;
    }
}

impl QueryTrait for WildcardQuery {
    fn build(&self) -> Value {
        let field = self.field.to_string();
        let query = self.value.to_string();
        let name = self.query_name();
        json!({
            name:{
                field:query
            }
        })
    }

    fn query_name(&self) -> String {
        return "wildcard".to_string();
    }
}

use crate::query::QueryTrait;
use serde_json::{json, Value};

#[derive(Default)]
pub struct MatchQuery {
    field: String,
    query: String,
}

impl MatchQuery {
    pub fn new(field: &str, query: &str) -> MatchQuery {
        let mut value = MatchQuery::default();
        value.field = field.to_string();
        value.query = query.to_string();
        return value;
    }
}

impl QueryTrait for MatchQuery {
    fn build(&self) -> Value {
        let field = self.field.to_string();
        let query = self.query.to_string();
        let name = self.query_name();
        json!({
            name:{
                field:{
                    "query":query
                }
            }
        })
    }

    fn query_name(&self) -> String {
        return "match".to_string();
    }
}

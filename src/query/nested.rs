use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use serde_json::{json, Value};

use crate::query::QueryTrait;

#[derive(Default)]
pub struct NestedQuery {
    path: String,
    query: Value,
    score_mode: Option<String>,
}

impl NestedQuery {
    pub fn new<T>(path: &str, query: T) -> NestedQuery where
        T: QueryTrait {
        NestedQuery {
            path: path.to_string(),
            query: query.build(),
            score_mode: None,
        }
    }
}

impl Serialize for NestedQuery {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("NestedQuery", 0)?;
        let _ = state.serialize_field("path", &self.path);
        let _ = state.serialize_field("query", &self.query);
        if self.score_mode.is_some() {
            let _ = state.serialize_field("score_mode", &self.score_mode.as_ref().unwrap());
        }
        state.end()
    }
}

impl QueryTrait for NestedQuery {
    fn build(&self) -> Value {
        let name = self.query_name();
        json!({ name: self })
    }
    fn query_name(&self) -> String {
        return "nested".to_string();
    }
}

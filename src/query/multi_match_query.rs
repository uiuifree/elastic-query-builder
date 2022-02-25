use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use serde_json::{json, Value};
use crate::query::QueryTrait;

#[derive(Default)]
pub struct MultiMatchQuery {
    fields: Vec<String>,
    query: String,
    search_type: String,
    fuzziness: String,
}

impl MultiMatchQuery {
    pub fn new(fields: Vec<String>, query: &str) -> MultiMatchQuery {
        let mut value = MultiMatchQuery::default();
        value.fields = fields;
        value.query = query.to_string();
        return value;
    }
    pub fn set_fields(&mut self, fields: Vec<String>) {
        self.fields = fields;
    }
    pub fn set_query(&mut self, query: &str) {
        self.query = query.to_string();
    }
    pub fn set_fuzziness(&mut self, fuzziness: &str) {
        self.fuzziness = fuzziness.to_string();
    }
    pub fn set_type(&mut self, search_type: &str) {
        self.search_type = search_type.to_string();
    }
}

impl Serialize for MultiMatchQuery {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("MultiMatchQuery", 0)?;
        let _ = state.serialize_field("query", &self.query);
        let _ = state.serialize_field("fields", &self.fields);
        if !self.search_type.is_empty() {
            let _ = state.serialize_field("type", &self.search_type);
        }
        if !self.fuzziness.is_empty() {
            let _ = state.serialize_field("fuzziness", &self.fuzziness);
        }
        state.end()
    }
}

impl QueryTrait for MultiMatchQuery {
    fn build(&self) -> Value {
        let name = self.query_name();
         json!({
            name: self
        })
    }
    fn query_name(&self) -> String {
        return "multi_match".to_string();
    }
}

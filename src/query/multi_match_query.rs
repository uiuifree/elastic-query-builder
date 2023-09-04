use crate::query::QueryTrait;
use crate::util::UtilMap;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use serde_json::{json, Value};

#[derive(Default)]
pub struct MultiMatchQuery {
    fields: Vec<String>,
    query: String,
    search_type: String,
    fuzziness: String,
    boost: Option<f64>,
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
    pub fn set_boost(mut self, boost: f64) -> MultiMatchQuery {
        self.boost = Some(boost);
        self
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
        let mut root = UtilMap::new();
        root.append_string("query", self.query.to_string());
        root.append_vec_string("fields", self.fields.clone());
        root.append_boost(self.boost);
        if !self.search_type.is_empty() {
            root.append_string("type", self.search_type.clone());
        }
        if !self.fuzziness.is_empty() {
            root.append_string("fuzziness", self.fuzziness.clone());
        }
        root.build_object(self.query_name())
    }
    fn query_name(&self) -> String {
        return "multi_match".to_string();
    }
}
#[test]
fn test() {
    let build = MultiMatchQuery::new(
        vec!["title".to_string(), "description".to_string()],
        "elastic",
    )
    .set_boost(100.0);
    assert_eq!("{\"multi_match\":{\"boost\":100.0,\"fields\":[\"title\",\"description\"],\"query\":\"elastic\"}}", build.build().to_string());
}

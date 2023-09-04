use crate::query::QueryTrait;
use crate::util::UtilMap;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use serde_json::{json, Map, Value};
use std::collections::HashMap;

#[derive(Default)]
pub struct MatchQuery {
    field: String,
    query: String,
    boost: Option<f64>,
}

impl MatchQuery {
    pub fn new(field: &str, query: &str) -> MatchQuery {
        let mut value = MatchQuery::default();
        value.field = field.to_string();
        value.query = query.to_string();
        return value;
    }
    pub fn set_boost(mut self, boost: f64) -> MatchQuery {
        self.boost = Some(boost);
        self
    }
}

impl QueryTrait for MatchQuery {
    fn build(&self) -> Value {
        let mut query = UtilMap::new();
        query.append_string("query", self.query.to_string());
        query.append_boost(self.boost);

        let mut root = UtilMap::new();
        root.append_object(self.field.to_string(), query);
        root.build_object(self.query_name())
    }
    fn query_name(&self) -> String {
        return "match".to_string();
    }
}

#[test]
fn test() {
    let build = MatchQuery::new("title", "elastic").set_boost(100.0);
    assert_eq!(
        "{\"match\":{\"title\":{\"boost\":100.0,\"query\":\"elastic\"}}}",
        build.build().to_string()
    );
}

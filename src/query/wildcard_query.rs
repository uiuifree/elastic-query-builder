use crate::query::QueryTrait;
use crate::util::UtilMap;
use serde_json::{json, Value};

#[derive(Default)]
pub struct WildcardQuery {
    field: String,
    value: String,
    boost: Option<f64>,
}

impl WildcardQuery {
    pub fn new(field: &str, value: &str) -> WildcardQuery {
        let mut query = WildcardQuery::default();
        query.field = field.to_string();
        query.value = value.to_string();
        return query;
    }
    pub fn set_boost(mut self, boost: f64) -> WildcardQuery {
        self.boost = Some(boost);
        self
    }
}

impl QueryTrait for WildcardQuery {
    fn build(&self) -> Value {
        let mut query = UtilMap::new();
        query.append_string("value", self.value.to_string());
        query.append_boost(self.boost);

        let mut root = UtilMap::new();
        root.append_object(self.field.to_string(), query);
        root.build_object(self.query_name())
    }

    fn query_name(&self) -> String {
        return "wildcard".to_string();
    }
}
#[test]
fn test() {
    let build = WildcardQuery::new("title", "elastic").set_boost(100.0);
    assert_eq!(
        "{\"wildcard\":{\"title\":{\"boost\":100.0,\"value\":\"elastic\"}}}",
        build.build().to_string()
    );
}

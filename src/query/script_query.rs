use crate::query::QueryTrait;
use serde_json::{json, Value};

#[derive(Default)]
pub struct ScriptQuery {
    script: String,
    source: String,
    lang: String,
    params: Value,
}

impl ScriptQuery {
    pub fn new(script: &str) -> ScriptQuery {
        let mut query = ScriptQuery::default();
        query.script = script.to_string();
        return query;
    }
}

impl QueryTrait for ScriptQuery {
    fn build(&self) -> Value {
        let script = self.script.to_string();
        let name = self.query_name();
        json!({
            name:{
                "script":script
            }
        })
    }

    fn query_name(&self) -> String {
        return "script".to_string();
    }
}

#[test]
fn test() {
    let q = ScriptQuery::new("doc['keywords'].length == 2")
        .build()
        .to_string();
    assert_eq!(
        q,
        "{\"script\":{\"script\":\"doc['keywords'].length == 2\"}}"
    )
}

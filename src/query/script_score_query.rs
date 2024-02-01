use crate::query::match_all_query::MatchAllQuery;
use crate::query::QueryTrait;
use crate::util::UtilMap;
use serde_json::{Value};

#[derive(Default)]
pub struct ScriptScoreQuery {
    query: Value,
    script: String,
    min_score: Option<f64>,
    boost: Option<f64>,
}

impl ScriptScoreQuery {
    pub fn new(script: &str) -> ScriptScoreQuery {
        let mut query = ScriptScoreQuery::default();
        query.script = script.to_string();
        let q = MatchAllQuery::new().build();
        query.query = q;
        return query;
    }
    pub fn set_boost(mut self, boost: f64) -> ScriptScoreQuery {
        self.boost = Some(boost);
        self
    }
}

impl QueryTrait for ScriptScoreQuery {
    fn build(&self) -> Value {
        let mut root = UtilMap::new();
        root.append_value("query", self.query.clone());

        let mut script = UtilMap::new();
        script.append_string("source", self.script.clone());

        root.append_object("script", script);
        root.append_boost(self.boost);

        root.build_object(self.query_name())
    }

    fn query_name(&self) -> String {
        return "script_score".to_string();
    }
}

#[test]
fn test() {
    let q = ScriptScoreQuery::new("doc['keywords'].length == 2")
        .build()
        .to_string();
    assert_eq!(q, "{\"script_score\":{\"query\":{\"match_all\":{}},\"script\":{\"source\":\"doc['keywords'].length == 2\"}}}")
}

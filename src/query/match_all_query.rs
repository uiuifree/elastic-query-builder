use crate::query::QueryTrait;
use crate::util::UtilMap;
use serde_json::Value;

#[derive(Default)]
pub struct MatchAllQuery {
    boost: Option<f64>,
}

impl MatchAllQuery {
    pub fn new() -> MatchAllQuery {
        let mut query = MatchAllQuery::default();
        return query;
    }
    pub fn set_boost(mut self, boost: f64) -> MatchAllQuery {
        self.boost = Some(boost);
        self
    }
}

impl QueryTrait for MatchAllQuery {
    fn build(&self) -> Value {
        let mut root = UtilMap::new();
        root.append_boost(self.boost);
        root.build_object(self.query_name())
    }

    fn query_name(&self) -> String {
        return "match_all".to_string();
    }
}
#[test]
fn test() {
    let build = MatchAllQuery::new().set_boost(100.0);
    assert_eq!(
        "{\"match_all\":{\"boost\":100.0}}",
        build.build().to_string()
    );
    let build = MatchAllQuery::new();
    assert_eq!("{\"match_all\":{}}", build.build().to_string());
}

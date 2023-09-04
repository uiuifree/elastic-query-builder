use crate::query::QueryTrait;
use crate::util::UtilMap;
use serde_json::{json, Value};

#[derive(Default)]
pub struct TermsQuery {
    field: String,
    values: Vec<String>,
}

impl TermsQuery {
    pub fn new(field: &str, values: Vec<String>) -> TermsQuery {
        let mut query = TermsQuery::default();
        query.field = field.to_string();
        query.values = values;
        return query;
    }
}

impl QueryTrait for TermsQuery {
    fn build(&self) -> Value {
        let mut map = UtilMap::new();
        map.append_vec_string(self.field.as_str(), self.values.clone());
        map.build_object(self.query_name())
    }

    fn query_name(&self) -> String {
        return "terms".to_string();
    }
}

mod tests {
    use super::*;
    use crate::aggregation::stats_aggregation::StatsAggregation;
    use crate::aggregation::terms_aggregation::TermsAggregation;
    use crate::aggregation::AggregationTrait;

    #[test]
    fn test() {
        let terms = TermsQuery::new("f", vec!["value1".to_string(), "value2".to_string()]);
        let json = terms.build();
        assert_eq!(
            "{\"terms\":{\"f\":[\"value1\",\"value2\"]}}",
            json.to_string()
        );
    }
}

use serde_json::{json, Value};
use crate::query::QueryTrait;

#[derive(Default)]
pub struct ExistsQuery {
    field: String,
}

impl ExistsQuery {
    pub fn new(field: &str) -> ExistsQuery {
        let mut value = ExistsQuery::default();
        value.field = field.to_string();
        return value;
    }
}

impl QueryTrait for ExistsQuery {
    fn build(&self) -> Value {
        let name = self.query_name();
        let field = self.field.to_string();
        json!({
            name: {
                "field":field
            }
        })
    }

    fn query_name(&self) -> String {
        return "exists".to_string();
    }
}


#[cfg(test)]
mod tests {
    use crate::query::exists_query::ExistsQuery;
    use crate::query::QueryTrait;

    #[test]
    fn build() {
        assert_eq!(ExistsQuery::new("id").build().to_string(), "{\"exists\":{\"field\":\"id\"}}");
    }
}

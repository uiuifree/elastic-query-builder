use serde_json::Value;
use crate::mapping::MappingTrait;
use crate::util::UtilMap;

///  https://www.elastic.co/guide/en/elasticsearch/reference/current/binary.html
#[derive(Default)]
pub struct BooleanFieldType {
    name: String,
    doc_values: Option<bool>,
    store: Option<bool>,
}

impl BooleanFieldType {
    pub fn new() -> Self {
        BooleanFieldType::default()
    }
    pub fn set_doc_values(&mut self, value: bool) {
        self.doc_values = Some(value);
    }
    pub fn set_store(&mut self, value: bool) {
        self.store = Some(value);
    }
}

impl MappingTrait for BooleanFieldType {
    fn build(&self) -> Value {
        let mut map = UtilMap::new();
        map.append_string("type", self.query_name());
        map.build()
    }

    fn query_name(&self) -> String {
        "boolean".to_string()
    }
}

#[test]
fn test() {
    let query = BooleanFieldType::new().build();
    println!("{:?}", query)
}
use crate::mapping::MappingTrait;
use crate::util::UtilMap;
use serde_json::Value;

///  https://www.elastic.co/guide/en/elasticsearch/reference/current/binary.html
#[derive(Default)]
pub struct BinaryFieldType {
    doc_values: Option<bool>,
    store: Option<bool>,
}

impl BinaryFieldType {
    pub fn new() -> Self {
        BinaryFieldType::default()
    }
    pub fn set_doc_values(&mut self, value: bool) {
        self.doc_values = Some(value);
    }
    pub fn set_store(&mut self, value: bool) {
        self.store = Some(value);
    }
}

impl MappingTrait for BinaryFieldType {
    fn build(&self) -> Value {
        let mut map = UtilMap::new();
        map.append_string("type", self.query_name());
        map.build()
    }

    fn query_name(&self) -> String {
        "binary".to_string()
    }
}

#[test]
fn test() {
    let query = BinaryFieldType::new().build();
    println!("{:?}", query)
}

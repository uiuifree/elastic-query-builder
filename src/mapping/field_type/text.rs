use crate::mapping::MappingTrait;
use crate::util::UtilMap;
use serde_json::Value;

///  https://www.elastic.co/guide/en/elasticsearch/reference/current/binary.html
#[derive(Default)]
pub struct TextFieldType {
    index: Option<bool>,
}

impl TextFieldType {
    pub fn new() -> Self {
        TextFieldType::default()
    }
    pub fn set_index(&mut self, index: bool) {
        self.index = Some(index);
    }
}

impl MappingTrait for TextFieldType {
    fn build(&self) -> Value {
        let mut map = UtilMap::new();
        map.append_string("type", self.query_name());
        if self.index.is_some() {
            map.append_string("index", self.index.clone().unwrap().to_string());
        }
        map.build()
    }

    fn query_name(&self) -> String {
        "text".to_string()
    }
}

#[test]
fn test() {
    let query = TextFieldType::new().build();
    println!("{:?}", query)
}

use serde_json::Value;
use crate::mapping::MappingTrait;
use crate::util::UtilMap;

///  https://www.elastic.co/guide/en/elasticsearch/reference/current/binary.html
#[derive(Default)]
pub struct NumericFieldType {
    mapping_type: String,
    index: Option<bool>,
}

impl NumericFieldType {
    fn new<T: Into<String>>(mapping_type: T) -> Self {
        NumericFieldType {
            mapping_type: mapping_type.into(),
            ..NumericFieldType::default()
        }
    }
    pub fn new_long() -> Self {
        NumericFieldType::new("long")
    }
    pub fn new_integer() -> Self {
        NumericFieldType::new("integer")
    }
    pub fn new_short() -> Self {
        NumericFieldType::new("short")
    }
    pub fn new_byte() -> Self {
        NumericFieldType::new("byte")
    }
    pub fn new_double() -> Self {
        NumericFieldType::new("double")
    }
    pub fn new_float() -> Self {
        NumericFieldType::new("float")
    }
    pub fn new_half_float() -> Self {
        NumericFieldType::new("half_float")
    }
    pub fn new_scaled_float() -> Self {
        NumericFieldType::new("scaled_float")
    }
    pub fn new_unsigned_long() -> Self {
        NumericFieldType::new("unsigned_long")
    }
    pub fn set_index(&mut self, index: bool) {
        self.index = Some(index);
    }
}

impl MappingTrait for NumericFieldType {
    fn build(&self) -> Value {
        let mut map = UtilMap::new();
        map.append_string("type", self.mapping_type.to_string());
        if self.index.is_some() {
            map.append_string("index", self.index.clone().unwrap().to_string());
        }
        map.build()
    }

    fn query_name(&self) -> String {
        "".to_string()
    }
}

#[test]
fn test() {
    let query = NumericFieldType::new_long().build();
    println!("{:?}", query)
}
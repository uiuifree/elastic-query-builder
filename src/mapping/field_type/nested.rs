use serde_json::Value;
use crate::mapping::field_type::boolean::BooleanFieldType;
use crate::mapping::MappingTrait;
use crate::mapping::properties::MappingProperties;
use crate::util::UtilMap;

///  https://www.elastic.co/guide/en/elasticsearch/reference/current/binary.html
#[derive(Default)]
pub struct NestedFieldType {
    properties: MappingProperties,
}

impl NestedFieldType {
    pub fn new(properties: MappingProperties) -> NestedFieldType {
        NestedFieldType {
            properties
        }
    }
}


impl MappingTrait for NestedFieldType {
    fn build(&self) -> Value {
        let mut map = UtilMap::new();
        map.append_string("type", self.query_name());
        let a = self.properties.build();
        if a.is_object() {
            let obj = a.as_object().unwrap();
            for (k, v) in obj {
                map.append_value(k.clone(), v.clone());
            }
        }

        map.build()
    }

    fn query_name(&self) -> String {
        "nested".to_string()
    }
}

#[test]
fn test() {
    let mut hoge = MappingProperties::new();
    hoge.add_property("img", BooleanFieldType::default());
    let query = NestedFieldType::new(
        hoge
    ).build();
    println!("{:?}", query)
}
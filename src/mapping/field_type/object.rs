use crate::mapping::field_type::boolean::BooleanFieldType;
use crate::mapping::properties::MappingProperties;
use crate::mapping::MappingTrait;
use crate::util::UtilMap;
use serde_json::Value;

///  https://www.elastic.co/guide/en/elasticsearch/reference/current/binary.html
#[derive(Default)]
pub struct ObjectFieldType {
    properties: MappingProperties,
}

impl ObjectFieldType {
    pub fn new(properties: MappingProperties) -> ObjectFieldType {
        ObjectFieldType { properties }
    }
}

impl MappingTrait for ObjectFieldType {
    fn build(&self) -> Value {
        let mut map = UtilMap::new();
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
        "".to_string()
    }
}

#[test]
fn test() {
    let mut hoge = MappingProperties::new();
    hoge.add_property("img", BooleanFieldType::default());
    let query = ObjectFieldType::new(hoge).build();
    println!("{:?}", query)
}

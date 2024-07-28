pub mod field_type;
pub mod properties;

use crate::mapping::field_type::keyword::KeywordFieldType;
use crate::mapping::field_type::text::TextFieldType;
use crate::mapping::properties::MappingProperties;
use crate::util::UtilMap;
use serde_json::{json, Value};

pub trait MappingTrait {
    fn build(&self) -> Value;
    fn query_name(&self) -> String;
}

pub struct MappingBuilder {
    properties: MappingProperties,
    setting: Option<Value>,
}

impl MappingBuilder {
    pub fn new() -> MappingBuilder {
        MappingBuilder {
            properties: MappingProperties::new(),
            setting: None,
        }
    }
    pub fn add_property<T>(&mut self, key: &str, value: T) -> &mut MappingBuilder
    where
        T: MappingTrait + 'static,
    {
        self.properties.add_property(key, value);
        self
    }
    pub fn set_properties(&mut self, properties: MappingProperties) {
        self.properties = properties;
    }
    pub fn set_setting(&mut self, properties: Value) {
        self.setting = Some(properties);
    }
    pub fn build(self) -> Value {
        let mut map = UtilMap::new();
        map.append_value("mappings", self.properties.build());
        if let Some(ref setting) = self.setting {
            map.append_value("setting", setting.clone())
        }
        map.build()
    }
}

#[test]
fn test() {
    let mut mapping = MappingBuilder::new();
    mapping
        .add_property("title", KeywordFieldType::new())
        .add_property("content", TextFieldType::new());
    mapping.set_setting(json!({
       "index.lifecycle.name": "logs_policy"
    }));
    println!("{}", mapping.build().to_string())
}

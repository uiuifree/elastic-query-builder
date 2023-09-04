pub mod field_type;
pub mod properties;

use crate::mapping::field_type::keyword::KeywordFieldType;
use crate::mapping::field_type::text::TextFieldType;
use crate::mapping::properties::MappingProperties;
use crate::util::UtilMap;
use serde_json::{json, Value};
use std::collections::HashMap;

pub trait MappingTrait {
    fn build(&self) -> Value;
    fn query_name(&self) -> String;
}

pub struct MappingBuilder {
    properties: MappingProperties,
}

impl MappingBuilder {
    pub fn new() -> MappingBuilder {
        MappingBuilder {
            properties: MappingProperties::new(),
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
    pub fn build(self) -> Value {
        let mut map = UtilMap::new();
        map.append_value("mappings", self.properties.build());
        map.build()
    }
}

#[test]
fn test() {
    let mut mapping = MappingBuilder::new();
    mapping
        .add_property("title", KeywordFieldType::new())
        .add_property("content", TextFieldType::new());
    println!("{}", mapping.build().to_string())
}

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
    settings: Option<Value>,
    aliases: Option<Value>,
}

impl MappingBuilder {
    pub fn new() -> MappingBuilder {
        MappingBuilder {
            properties: MappingProperties::new(),
            settings: None,
            aliases: None,
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
    pub fn set_settings(&mut self, settings: Value) {
        self.settings = Some(settings);
    }
    pub fn set_aliases(&mut self, aliases: Value) {
        self.aliases = Some(aliases);
    }
    pub fn build(self) -> Value {
        let mut map = UtilMap::new();
        map.append_value("mappings", self.properties.build());
        if let Some(ref settings) = self.settings {
            map.append_value("settings", settings.clone())
        }
      if let Some(ref aliases) = self.aliases {
            map.append_value("aliases", aliases.clone())
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
    mapping.set_settings(json!({
       "index.lifecycle.name": "logs_policy"
    }));
    println!("{}", mapping.build().to_string())
}

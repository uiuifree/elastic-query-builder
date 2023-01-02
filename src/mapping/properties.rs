use std::collections::HashMap;
use serde_json::Value;
use crate::mapping::MappingTrait;
use crate::util::UtilMap;

#[derive(Default)]
pub struct MappingProperties {
    properties: HashMap<String, Box<dyn MappingTrait>>,
}

impl MappingProperties {
    pub(crate) fn new() -> Self where Self: Sized {
        MappingProperties::default()
    }
    pub fn add_property<T>(&mut self, key: &str, value: T) -> &mut MappingProperties
        where T: MappingTrait + 'static {
        self.properties.insert(key.to_string(), Box::new(value));
        self
    }
}

impl MappingTrait for MappingProperties {
    fn build(&self) -> Value {
        let mut map = UtilMap::new();
        for (k, v) in &self.properties {
            map.append_value(k, v.build());
        }
        map.build_object("properties")
    }

    fn query_name(&self) -> String {
        "properties".to_string()
    }
}
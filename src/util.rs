use serde_json::{json, Map, Value};


pub(crate) struct UtilMap {
    pub map: Map<String, Value>,
}

impl UtilMap {
    pub fn new() -> UtilMap {
        UtilMap {
            map: Default::default(),
        }
    }
    pub fn append_string<T: Into<String>>(&mut self, key: T, value: String) {
        self.map.insert(key.into(), Value::String(value));
    }
    pub fn append_vec_string<T: Into<String>>(&mut self, key: T, values: Vec<String>) {
        let mut list = vec![];
        for value in values{
            list.push(Value::String(value));
        }

        self.map.insert(key.into(), Value::Array(list));
    }
    pub fn append_boost(&mut self, boost: Option<f64>){
        if boost.is_none() {
            return ;
        }
        self.map.insert("boost".to_string(), Value::from(boost.unwrap()));
    }
    pub fn append_object<T: Into<String>>(&mut self,key: T, map:UtilMap){
        self.map.insert(key.into(),Value::Object(map.map));
    }
    pub fn append_value<T: Into<String>>(&mut self,key: T, value:Value){
        self.map.insert(key.into(),Value::from(value));
    }

    pub fn build_object<T:Into<String>>(mut self ,key:T)->Value{
        let  mut  map  = Map::default();
        map.insert(key.into(),Value::Object(self.map));

        Value::Object(map)
    }
}

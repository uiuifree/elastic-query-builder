use serde_json::Value;
use crate::mapping::MappingTrait;
use crate::util::UtilMap;

///  https://www.elastic.co/guide/en/elasticsearch/reference/current/binary.html
#[derive(Default)]
pub struct KeywordFieldType {
    index: Option<bool>,
}

impl KeywordFieldType {
 pub   fn new() -> Self {
        KeywordFieldType::default()
    }
    pub fn  set_index(&mut self,index:bool){
        self.index = Some(index);
    }
}

impl MappingTrait for KeywordFieldType {

    fn build(&self) -> Value {
        let mut map = UtilMap::new();
        map.append_string("type", self.query_name());
        if self.index.is_some(){
            map.append_string("index", self.index.clone().unwrap().to_string());
        }
        map.build()
    }

    fn query_name(&self) -> String {
        "keyword".to_string()
    }
}

#[test]
fn test(){
    let query = KeywordFieldType::new().build();
    println!("{:?}",query)
}
use serde_json::Value;
use crate::mapping::MappingTrait;
use crate::util::UtilMap;

///  https://www.elastic.co/guide/en/elasticsearch/reference/current/binary.html
#[derive(Default)]
pub struct DateFieldType {
    name: String,
    doc_values: Option<bool>,
    format: Option<String>,
    index: Option<bool>,
}

impl DateFieldType {
    pub fn new() -> Self {
        DateFieldType ::default()
    } pub fn  set_doc_values(&mut self,value:bool){
        self.doc_values = Some(value);
    }
    pub fn  set_format(&mut self, format:String){
        self.format = Some(format);
    }
    pub fn  set_index(&mut self,index:bool){
        self.index = Some(index);
    }
}

impl MappingTrait for DateFieldType {

    fn build(&self) -> Value {
        let mut map = UtilMap::new();
        map.append_string("type", self.query_name());
        map.build()
    }

    fn query_name(&self) -> String {
        "date".to_string()
    }
}

#[test]
fn test(){
    let query = DateFieldType::new().build();
    println!("{:?}",query)
}
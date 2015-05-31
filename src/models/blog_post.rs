extern crate rustc_serialize;

use self::rustc_serialize::json::{ToJson,Json};
use std::collections::BTreeMap;

pub struct Record {
    pub title: String,
    pub body: String
}

impl ToJson for Record {
    fn to_json(&self) -> Json {
        let mut m: BTreeMap<String, Json> = BTreeMap::new();
        m.insert("title".to_string(), self.title.to_json());
        m.insert("body".to_string(), self.body.to_json());
        m.to_json()
    }
}

pub fn find_one() -> Record {
    Record {
        title: "Hello World!".to_string(),
        body: "Lorem ipsum dolor sit amet.".to_string()
    }
}

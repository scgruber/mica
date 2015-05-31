extern crate iron;
extern crate handlebars_iron as hbs;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use hbs::{Template, HandlebarsEngine};
use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;

struct BlogPost {
    title: String,
    body: String
}

impl ToJson for BlogPost {
    fn to_json(&self) -> Json {
        let mut m: BTreeMap<String, Json> = BTreeMap::new();
        m.insert("title".to_string(), self.title.to_json());
        m.insert("body".to_string(), self.body.to_json());
        m.to_json()
    }
}

fn hello_world(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    let template_data = BlogPost {
                            title: "Hello World!".to_string(),
                            body: "Lorem ipsum dolor sit amet.".to_string()
                        };
    resp.set_mut(Template::new("blog_post", template_data)).set_mut(status::Ok);
    Ok(resp)
}

fn main() {
    let mut chain = Chain::new(hello_world);
    chain.link_after(HandlebarsEngine::new("./templates/", ".hbs"));
    println!("Server running on http://locahost:3000/");
    Iron::new(chain).http("localhost:3000").unwrap();
}

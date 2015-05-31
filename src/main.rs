extern crate iron;
extern crate handlebars_iron as hbs;

use iron::prelude::*;
use iron::status;
use hbs::{Template, HandlebarsEngine};

mod models {
    pub mod blog_post;
}

fn hello_world(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    let template_data = models::blog_post::find_one();
    resp.set_mut(Template::new("blog_post", template_data)).set_mut(status::Ok);
    Ok(resp)
}

fn main() {
    let mut chain = Chain::new(hello_world);
    chain.link_after(HandlebarsEngine::new("./templates/", ".hbs"));
    println!("Server running on http://locahost:3000/");
    Iron::new(chain).http("localhost:3000").unwrap();
}

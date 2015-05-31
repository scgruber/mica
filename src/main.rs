extern crate iron;
extern crate handlebars_iron as hbs;
extern crate sqlite3;
extern crate persistent;

use iron::{Iron, Request, Response, IronResult, Chain};
use iron::status;
use hbs::{Template, HandlebarsEngine};
use sqlite3::{DatabaseConnection, SqliteResult, SqliteError};
use persistent::State;

mod models {
    pub mod blog_post;
}

pub struct Database;
impl Assoc<DatabaseConnection> for Database {}

fn init_db() -> Result<DatabaseConnection, SqliteError> {
    let mut conn = try!(DatabaseConnection::in_memory());
    Ok(conn)
}

fn hello_world(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    // TODO: Query template_data using the database connection
    let template_data = models::blog_post::find_one();
    resp.set_mut(Template::new("blog_post", template_data)).set_mut(status::Ok);
    Ok(resp)
}

fn main() {
    let mut chain = Chain::new(hello_world);
    chain.link(State::<Database, DatabaseConnection>::both(init_db().unwrap()));
    chain.link_after(HandlebarsEngine::new("./templates/", ".hbs"));
    println!("Server running on http://locahost:3000/");
    Iron::new(chain).http("localhost:3000").unwrap();
}

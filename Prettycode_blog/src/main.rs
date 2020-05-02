#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::templates::Template;
use std::collections::HashMap;
use diesel::PgConnection;
use rocket::request::{Form};
use rocket_contrib::serve::StaticFiles;
use rocket::http::RawStr;

#[derive(FromForm)]
struct NewPostForm<'r> {
    name: &'r RawStr,
    body: &'r RawStr
}

#[database("pg_db")]
pub struct DbConn(PgConnection);

#[get("/")]
fn homepage() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("homepage", &context)
}

#[post("/new_post", data="<form>")]
fn new_post(form: Form<NewPostForm>) -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("new_post", &context)
}

fn main() {
    rocket::ignite()
    .attach(Template::fairing())
    .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
    .mount("/", routes![homepage, new_post])
    .launch();
}

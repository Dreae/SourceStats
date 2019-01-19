#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

use std::collections::HashMap;

#[get("/")]
fn hello() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("index", &context)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![hello])
        .mount("/static", StaticFiles::from("./static"))
        .attach(Template::fairing())
        .launch();
}
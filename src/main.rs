#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate sourcestats_database;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

mod ui;

fn main() {
    rocket::ignite()
        .mount("/", routes![
            ui::index
        ])
        .mount("/static", StaticFiles::from("./static"))
        .attach(Template::fairing())
        .launch();
}
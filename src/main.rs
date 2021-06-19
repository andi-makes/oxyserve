mod config;

use config::Config;

#[macro_use]
extern crate rocket;

use std::path::PathBuf;

use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

#[get("/<path..>", rank = 1000)]
fn index(path: PathBuf) -> Template {
    let page = Config::from_file("./data/config.json").unwrap();

    Template::render("page", page.context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/static", FileServer::from("./data/static"))
        .attach(Template::fairing())
}

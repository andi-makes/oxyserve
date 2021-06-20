mod config;

use config::Config;

#[macro_use]
extern crate rocket;

use std::path::PathBuf;

use rocket::{fs::FileServer, http::Status};
use rocket_dyn_templates::Template;

#[get("/<path..>", rank = 1000)]
fn index(path: PathBuf) -> Result<Template, Status> {
    let config_path: PathBuf = ["./data", path.to_str().unwrap().trim(), "config.json"].iter().collect();

    println!("{}", config_path.to_str().unwrap());

    let page = match Config::from_file(config_path) {
        Some(p) => p,
        None => return Err(Status::NotFound),
    };

    Ok(Template::render(page.template_name, page.context))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/static", FileServer::from("./data/static"))
        .attach(Template::fairing())
}

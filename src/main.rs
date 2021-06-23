mod config;

use config::Config;

#[macro_use]
extern crate rocket;

use std::path::PathBuf;

use rocket::{fs::FileServer, http::Status, request::FromRequest};
use rocket_dyn_templates::{Metadata, Template};

#[get("/<path..>", rank = 1000)]
fn index(path: PathBuf) -> Result<Template, Status> {
    let config_path: PathBuf = ["./data", path.to_str().unwrap().trim(), "config.json"].iter().collect();

    let page = match Config::from_file(config_path) {
        Ok(p) => p,
        Err(e) => match e { // TODO: Better Error logging / responding
            config::ConfigError::File => return Err(Status::NotFound),
            config::ConfigError::Parse => return Err(Status::InternalServerError),
            config::ConfigError::MissingField => return Err(Status::InternalServerError),
            config::ConfigError::ReplaceContent => return Err(Status::InternalServerError),
            config::ConfigError::Type => return Err(Status::InternalServerError),
        },
    };

    Ok(Template::render(page.template_name, page.context))
}

#[catch(404)]
async fn not_found<'a>(req: &rocket::Request<'a>) -> Result<Template, &'static str> {
    let metadata = Metadata::from_request(req).await.unwrap();
    if metadata.contains_template("404") {
        Ok(Template::render("404", ()))
    } else {
        Err("404 not found")
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![not_found])
        .mount("/", routes![index])
        .mount("/static", FileServer::from("./data/static"))
        .attach(Template::fairing())
}

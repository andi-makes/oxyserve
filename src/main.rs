mod catcher;
mod config;
mod helpers;

use config::Config;

#[macro_use]
extern crate rocket;

use std::path::PathBuf;

use rocket::{fs::FileServer, http::Status};
use rocket_dyn_templates::Template;

#[get("/<path..>", rank = 1000)]
fn index(path: PathBuf) -> Result<Template, Status> {
    let config_path: String = format!("./data/pages/{}/index.json", path.to_str().unwrap().trim());

    let page = match Config::from_file(&config_path) {
        Ok(p) => p,
        Err(e) => match e {
            config::ConfigError::NotFound { name: _ } => return Err(Status::NotFound),
            config::ConfigError::JsonParseError { context: _ } => {
                return Err(Status::InternalServerError)
            }
        },
    };

    Ok(Template::render(page.template_name, page.context))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![catcher::not_found])
        .mount("/", routes![index])
        .mount("/static", FileServer::from("./data/static"))
        .attach(Template::custom(|engines| {
            helpers::customize(&mut engines.handlebars);
        }))
}

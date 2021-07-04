mod config;

use config::Config;

#[macro_use]
extern crate rocket;

use std::path::PathBuf;

use rocket::{fs::FileServer, http::Status, request::FromRequest};
use rocket_dyn_templates::{Metadata, Template};

#[get("/<path..>", rank = 1000)]
fn index(path: PathBuf) -> Result<Template, Status> {
    let config_path: String = format!("./data/pages/{}/index.json", path.to_str().unwrap().trim());

    let page = match Config::from_file(&config_path) {
        Ok(p) => p,
        Err(e) => match e {
            config::ConfigError::NotFound { name: _ } => return Err(Status::NotFound),
            config::ConfigError::InternalServerError { context: _ } => {
                return Err(Status::InternalServerError)
            }
        },
    };

    Ok(Template::render(page.template_name, page.context))
}

#[catch(404)]
async fn not_found<'a>(req: &rocket::Request<'a>) -> Result<Template, &'static str> {
    let metadata = Metadata::from_request(req).await.unwrap();
    if metadata.contains_template("404") {
        match Config::from_file("./data/404.json") {
            Ok(c) => Ok(Template::render("404", c.context)),
            Err(_) => Ok(Template::render("404", ())),
        }
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

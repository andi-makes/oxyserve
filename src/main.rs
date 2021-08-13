/// Oxyserve
///
/// Custom webserver built on top of rocket.rs
/// Render your websites using handlebars templates
/// Additional Helpers for embedding files into your site
mod catcher;
mod config;
mod fileserver;
mod helpers;

use config::Config;

use std::path::PathBuf;

use rocket::http::Status;
use rocket_dyn_templates::Template;

#[macro_use]
extern crate rocket;

/// Main path handler
/// Tries to load a website from the `data` directory
#[get("/<path..>", rank = 1000)]
fn index(path: PathBuf) -> Result<Template, Status> {
    // Get the data directory
    let data_dir = &std::env::var("DATA_DIR").unwrap_or_else(|_| "./data".to_string());

    // First, construct the path to a normal page
    let mut page_path = PathBuf::from(data_dir);
    page_path.push("pages");
    page_path.push(&path);
    page_path.push("index");
    page_path.set_extension("json");

    // If there is no normal page, construct a path to a note
    if !page_path.exists() {
        page_path = PathBuf::from(data_dir);
        page_path.push("notes");
        page_path.push(&path);
        page_path.set_extension("json");
    }

    // Try that path and get the config file
    let page = match Config::from_file(&page_path) {
        Ok(p) => p,
        Err(e) => match e {
            config::ConfigError::NotFound { name: _ } => return Err(Status::NotFound),
            config::ConfigError::JsonParseError { context: _ } => {
                return Err(Status::InternalServerError)
            }
        },
    };

    // Render the html page based on the config file
    Ok(Template::render(page.template_name, page.context))
}

// READY FOR LAUNCH
#[launch]
fn rocket() -> _ {
    let data_dir = std::env::var("DATA_DIR").unwrap_or_else(|_| "./data".to_string());

    std::env::set_var("ROCKET_TEMPLATE_DIR", format!("{}/templates", data_dir));

    rocket::build()
        .register("/", catchers![catcher::not_found])
        .mount("/", routes![index, fileserver::files])
        .attach(Template::custom(move |engines| {
            helpers::customize(&mut engines.handlebars);
        }))
}

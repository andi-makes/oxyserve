mod catcher;
mod config;
mod helpers;

use config::Config;

#[macro_use]
extern crate rocket;

use std::path::PathBuf;

use rocket::{fs::NamedFile, http::Status, response, Request, Response};
use rocket_dyn_templates::Template;

#[get("/<path..>", rank = 1000)]
fn index(path: PathBuf) -> Result<Template, Status> {
    let data_dir = std::env::var("DATA_DIR").unwrap_or("./data".to_string());
    let config_path: String = format!(
        "{}/pages/{}/index.json",
        data_dir,
        path.to_str().unwrap().trim()
    );

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

#[get("/notes/<path..>", rank = 999)]
fn notes(path: PathBuf) -> Result<Template, Status> {
    let data_dir = std::env::var("DATA_DIR").unwrap_or("./data".to_string());
    let config_path: String = format!("{}/notes/{}.json", data_dir, path.to_str().unwrap().trim());

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

// CACHE STATIC FILES

struct CachedFile(NamedFile);

impl<'r> response::Responder<'r, 'static> for CachedFile {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        Response::build_from(self.0.respond_to(req)?)
            .raw_header("Cache-control", "max-age=31536000") // 1y
            .ok()
    }
}

#[get("/static/<file..>", rank = 10)]
async fn files(file: PathBuf) -> Option<CachedFile> {
    let data_dir = std::env::var("DATA_DIR").unwrap_or("./data".to_string());

    NamedFile::open(std::path::Path::new(&format!("{}/static", &data_dir)).join(file))
        .await
        .ok()
        .map(|nf| CachedFile(nf))
}

// READY FOR LAUNCH

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![catcher::not_found])
        .mount("/", routes![index, notes, files])
        // .mount("/static", FileServer::from(format!("{}/static", &data_dir)))
        .attach(Template::custom(move |engines| {
            helpers::customize(&mut engines.handlebars);
        }))
}

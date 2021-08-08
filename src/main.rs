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
    // Get the data directory
    let data_dir = &std::env::var("DATA_DIR").unwrap_or("./data".to_string());

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
        .mount("/", routes![index, files])
        .attach(Template::custom(move |engines| {
            helpers::customize(&mut engines.handlebars);
        }))
}

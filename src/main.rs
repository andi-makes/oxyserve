/// Oxyserve
///
/// Custom webserver built on top of rocket.rs
/// Render your websites using handlebars templates
/// Additional Helpers for embedding files into your site
use std::path::PathBuf;

use actix_files::Files;
use actix_web::{get, App, HttpRequest, HttpServer};
use actix_web::{web, HttpResponse};
use handlebars::Handlebars;

mod config;
mod helpers;
mod fileserver;
mod catcher;

#[get("/{filename:.*}")]
async fn index(req: HttpRequest, hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    use config::{Config, ConfigError};

    // Get the data directory
    let data_dir = &std::env::var("DATA_DIR").unwrap_or_else(|_| "./data".to_string());

    let path = req.match_info().query("filename");

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

    let page = match Config::from_file(&page_path) {
        Ok(p) => p,
        Err(e) => match e {
            ConfigError::NotFound { name: _ } => return HttpResponse::NotFound().finish(),
            ConfigError::JsonParseError { context: _ } => {
                return HttpResponse::InternalServerError().finish()
            }
        },
    };

    let body = hb.render(&page.template_name, &page.context).unwrap();

    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Get the data directory
    let data_dir = &std::env::var("DATA_DIR").unwrap_or_else(|_| "./data".to_string());

    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html.hbs", format!("{}/templates", data_dir))
        .unwrap();
    helpers::customize(&mut handlebars);
    // Wrap Handlebars into something we can access later in the routing functions
    let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
//            .wrap(catcher::error_handlers())
            .app_data(handlebars_ref.clone())
            .service(Files::new("/static", "./data"))
            .service(index)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}


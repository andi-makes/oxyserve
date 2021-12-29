/// Oxyserve
///
/// Custom webserver built on top of rocket.rs
/// Render your websites using handlebars templates
/// Additional Helpers for embedding files into your site
use std::path::PathBuf;

use actix_files::NamedFile;
use actix_web::{get, App, HttpRequest, HttpServer};
use actix_web::{web, HttpResponse, Result};

use handlebars::Handlebars;

mod config;
mod helpers;

#[get("/static/{filename:.*}")]
async fn hello(req: HttpRequest) -> Result<NamedFile> {
    let file_path: PathBuf = ["./data/static", req.match_info().query("filename")]
        .iter()
        .collect();

    println!("{:?}", file_path);
    Ok(NamedFile::open(file_path)?)
}

#[get("/{filename:.*}")]
async fn index(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    use config::{Config, ConfigError};

    let page = match Config::from_file(&PathBuf::from("./data/pages/index.json")) {
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
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html.hbs", "./data/templates")
        .unwrap();
    helpers::customize(&mut handlebars);
    // Wrap Handlebars into something we can access later in the routing functions
    let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .service(hello)
            .service(index)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

use crate::config::Config;

use rocket::request::FromRequest;
use rocket_dyn_templates::{Metadata, Template};

#[catch(404)]
pub async fn not_found<'a>(req: &rocket::Request<'a>) -> Result<Template, &'static str> {
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

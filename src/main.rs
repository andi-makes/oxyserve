#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket::serde::{Deserialize, Serialize};
use rocket_dyn_templates::Template;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Project {
    link: String,
    name: String,
    description: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct NavLink {
    link: String,
    svg: String,
    name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Config {
    description: String,
    project: Vec<Project>,
    nav: Vec<NavLink>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Context {
    description: String,
    project: Vec<Project>,
    nav: Vec<NavLink>,
    embedded_css: String,
}

impl From<Config> for Context {
    fn from(c: Config) -> Self {
        Self {
            description: c.description,
            project: c.project,
            nav: c.nav,
            embedded_css: String::new(),
        }
    }
}

#[get("/")]
fn index() -> Template {
    let mut context: Context =
        toml::from_str::<Config>(&std::fs::read_to_string("./data/Config.toml").unwrap())
            .unwrap()
            .into();

    context.embedded_css = std::fs::read_to_string("./data/embedded.css").unwrap();

    Template::render("page", context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/static", FileServer::from("./data/static"))
        .attach(Template::fairing())
}

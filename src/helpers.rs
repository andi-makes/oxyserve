mod markdown;
mod svg;
mod util;

use rocket_dyn_templates::handlebars::Handlebars;

trait CustomHelper {
    fn register(hbs: &mut Handlebars);
}

pub fn customize(hbs: &mut Handlebars) {
    svg::Helpers::register(hbs);
    util::Helpers::register(hbs);
    markdown::Helpers::register(hbs);
}

mod markdown;
mod notes;
mod svg;
mod util;

use handlebars::Handlebars;

trait CustomHelper {
    fn register(hbs: &mut Handlebars);
}

pub fn customize(hbs: &mut Handlebars) {
    svg::Helpers::register(hbs);
    util::Helpers::register(hbs);
    markdown::Helpers::register(hbs);
    notes::Helpers::register(hbs);
}

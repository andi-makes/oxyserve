use rocket_dyn_templates::handlebars::{
    Context, Handlebars, Helper, HelperResult, Output, RenderContext,
};

use super::CustomHelper;

pub fn markdownify(content: &str) -> String {
    use pulldown_cmark::{html, Options, Parser};

    // Set up options and parser. Strikethroughs are not part of the CommonMark standard
    // and we therefore must enable it explicitly.
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&content, options);

    // Write to String buffer.
    let mut html_output = String::with_capacity(content.len() * 3 / 2);
    html::push_html(&mut html_output, parser);
    html_output
}

pub fn embed<'reg: 'rc, 'rc>(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h.param(0).unwrap();

    let data_dir = std::env::var("DATA_DIR").unwrap_or("./data".to_string());
    let path = format!("{}/{}", data_dir, param.value().as_str().unwrap().trim());

    let content = std::fs::read_to_string(path).unwrap();

    out.write(&content)?;
    Ok(())
}

pub struct Helpers;

impl CustomHelper for Helpers {
    fn register(hbs: &mut Handlebars) {
        hbs.register_helper("embed", Box::new(embed));
    }
}

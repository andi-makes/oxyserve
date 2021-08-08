use regex::Regex;
use rocket_dyn_templates::handlebars::{
    Context, Handlebars, Helper, HelperResult, Output, RenderContext,
};

use super::CustomHelper;

fn get_text_svg(path: &str) -> String {
    let svg = std::fs::read_to_string(path).unwrap();

    let re = Regex::new(r#"<svg .*?viewBox="([\d\s.]+)".*?>([\s\S]*)</svg>"#).unwrap();
    let captures = re.captures(&svg).unwrap();

    format!("<svg xmlns=\"http://www.w3.org/2000/svg\" class=\"svg-icon\" width=\"1em\" height=\"1em\" viewBox=\"{}\"> {} </svg>",  &captures[1], &captures[2])
}

fn text_svg(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h.param(0).unwrap();

    let data_dir = std::env::var("DATA_DIR").unwrap_or_else(|_| "./data".to_string());
    let path = format!("{}/{}", data_dir, param.value().as_str().unwrap().trim());
    let svg = get_text_svg(&path);

    out.write(&svg)?;
    Ok(())
}

fn asvg(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let text = h.param(0).unwrap();
    let link = h.param(1).unwrap();
    let svg = h.param(2).unwrap();

    let data_dir = std::env::var("DATA_DIR").unwrap_or_else(|_| "./data".to_string());
    let svg_path = format!("{}/{}", data_dir, svg.value().as_str().unwrap().trim());
    let svg = get_text_svg(&svg_path);

    out.write("<a href=\"")?;
    out.write(link.value().as_str().unwrap())?;
    out.write("\" target=\"_blank\" rel=\"noopener noreferrer\">")?;
    out.write(&svg)?;
    out.write(" ")?;
    out.write(text.value().as_str().unwrap())?;
    out.write("</a>")?;
    Ok(())
}

pub struct Helpers;

impl CustomHelper for Helpers {
    fn register(hbs: &mut Handlebars) {
        hbs.register_helper("textsvg", Box::new(text_svg));
        hbs.register_helper("asvg", Box::new(asvg));
    }
}

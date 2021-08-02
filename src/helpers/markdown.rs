use super::CustomHelper;
use rocket_dyn_templates::handlebars::{
    Context, Handlebars, Helper, HelperResult, Output, RenderContext,
};

pub struct Helpers;

impl Helpers {
    fn markdown<'reg: 'rc, 'rc>(
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        use pulldown_cmark::{html, Options, Parser};

        let param = h.param(0).unwrap();
        let data_dir = std::env::var("DATA_DIR").unwrap_or("./data".to_string());
        let path = format!("{}/{}", data_dir, param.value().as_str().unwrap().trim());
        let content = std::fs::read_to_string(path).unwrap();

        // Set up options and parser. Strikethroughs are not part of the CommonMark standard
        // and we therefore must enable it explicitly.
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        let parser = Parser::new_ext(&content, options);

        // Write to String buffer.
        let mut html_output: String = String::with_capacity(content.len() * 3 / 2);
        html::push_html(&mut html_output, parser);

        out.write(&html_output)?;
        Ok(())
    }
}

impl CustomHelper for Helpers {
    fn register(hbs: &mut Handlebars) {
        hbs.register_helper("markdown", Box::new(Self::markdown));
    }
}

use crate::helpers::util;

use super::CustomHelper;
use rocket_dyn_templates::handlebars::{
    Context, Handlebars, Helper, HelperResult, Output, RenderContext,
};

pub struct Helpers;

impl Helpers {
    fn markdown(
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let param = h.param(0).unwrap();
        let data_dir = std::env::var("DATA_DIR").unwrap_or_else(|_| "./data".to_string());
        let path = format!("{}/{}", data_dir, param.value().as_str().unwrap().trim());
        let content = std::fs::read_to_string(path).unwrap();

        out.write(&util::markdownify(&content))?;
        Ok(())
    }
}

impl CustomHelper for Helpers {
    fn register(hbs: &mut Handlebars) {
        hbs.register_helper("markdown", Box::new(Self::markdown));
    }
}

use crate::helpers::util;

use super::CustomHelper;
use rocket_dyn_templates::handlebars::{
    Context, Handlebars, Helper, HelperResult, Output, RenderContext,
};

use regex::Regex;

pub struct Helpers;

impl Helpers {
    fn note_title(
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

        let re = Regex::new(r#"(?m)^# (.*)$"#).unwrap();
        let title = &re.captures(&content).unwrap()[1];

        out.write(title)?;
        Ok(())
    }

    fn note_description(
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

        let re = Regex::new(r#"(?m)^[A-Za-z].*(?:\n[A-Za-z].*)*"#).unwrap();
        let description = &re.captures(&content).unwrap()[0];

        out.write(&util::markdownify(description))?;
        Ok(())
    }

    fn note_author(
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let param = h.param(0).unwrap().value().as_str().unwrap();

        let re = Regex::new(r#"\w+-(\w+)-\w+\.md$"#).unwrap();
        let author = &re.captures(param).unwrap()[1];

        out.write(&author.replace('_', " "))?;
        Ok(())
    }

    fn note_date(
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let param = h.param(0).unwrap().value().as_str().unwrap();

        let re = Regex::new(r#"(\d{4})_(\d{2})_(\d{2})-\w+-\w+\.md$"#).unwrap();
        let year = &re.captures(param).unwrap()[1];
        let month = &re.captures(param).unwrap()[2].parse::<i32>().unwrap();
        let day = &re.captures(param).unwrap()[3].parse::<i32>().unwrap();

        let day_suffix = match day % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        };

        let month = match month {
            1 => "January",
            2 => "Febuary",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => panic!("Invalid Month!"),
        };

        out.write(&format!("{}{} of {}, {}", day, day_suffix, month, year))?;
        Ok(())
    }

    fn note(
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

        let re = Regex::new(r#"(?m)^# .*$"#).unwrap();
        let title = &re.captures(&content).unwrap()[0];

        let content = content.replace(title, "");

        out.write(&util::markdownify(&content))?;
        Ok(())
    }
}

impl CustomHelper for Helpers {
    fn register(hbs: &mut Handlebars) {
        hbs.register_helper("note_title", Box::new(Self::note_title));
        hbs.register_helper("note_description", Box::new(Self::note_description));
        hbs.register_helper("note_author", Box::new(Self::note_author));
        hbs.register_helper("note_date", Box::new(Self::note_date));
        hbs.register_helper("note", Box::new(Self::note));
    }
}

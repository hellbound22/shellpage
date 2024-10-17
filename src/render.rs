use std::time::SystemTime;

use chrono::{DateTime, Utc, Local};
use tera::Tera;

use crate::{catalogue::Catalogue, errors::ShellpageError, ConfigFile};


pub struct RenderEngine {
    pub tera: Tera,
}

impl RenderEngine {
    pub fn new_from_config(config: &ConfigFile) -> Result<Self, ShellpageError> {
        let tera = if let Ok(t) = Tera::new(&config.template_path) {
            t
        } else {
            return Err(ShellpageError::RenderConfigurationError(config.template_path.to_owned()));
        };
        
        Ok(Self {
            tera
        })
    }

    pub fn index(&self, catalogue: &Catalogue) -> Result<String, ShellpageError> {
        let mut context = tera::Context::new();
        context.insert("posts", &catalogue.all_posts_ordered());

        let template_name = "index.html.tera";
        match self.tera.render(template_name, &context) {
            Ok(r) => Ok(r),
            Err(e) => Err(ShellpageError::RenderExecutionError(template_name.to_owned(), e.to_string())),
        }
    }

    pub fn post(&self, html_source: &str, created: &SystemTime) -> Result<String, ShellpageError> {
        let mut context = tera::Context::new();
        
        let now: DateTime<Local> = DateTime::from(*created); 
        context.insert("date", &now.to_rfc2822());

        context.insert("post", &html_source);

        let template_name = "post.html.tera";
        match self.tera.render(template_name, &context) {
            Ok(r) => Ok(r),
            Err(e) => Err(ShellpageError::RenderExecutionError(template_name.to_owned(), e.to_string())),
        }
    }
}

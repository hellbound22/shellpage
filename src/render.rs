use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use tera::Tera;

use crate::{catalogue::Catalogue, ConfigFile};


pub struct RenderEngine {
    pub tera: Tera,
}

impl RenderEngine {
    pub fn new_from_config(config: &ConfigFile) -> Self {
        let tera = Tera::new(&config.template_path).unwrap();
        
        Self {
            tera
        }
    }

    pub fn index(&self, catalogue: &Catalogue) -> String {
        let mut context = tera::Context::new();
        context.insert("posts", &catalogue.all_posts_ordered());

        self.tera.render("index.html.tera", &context).unwrap()
    }

    pub fn post(&self, html_source: &str) -> String {
        let mut context = tera::Context::new();
        
        let now: DateTime<Utc> = Utc::now();
        context.insert("date", &now.to_rfc2822());

        context.insert("post", &html_source);
        self.tera.render("post.html.tera", &context).unwrap()
    }
}

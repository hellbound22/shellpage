use crate::ConfigFile;

use std::collections::HashMap;
use std::fs::{self};

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    path: String,
    pub name: String,
    pub file_name: String,
    pub created: std::time::SystemTime,
}

pub struct Catalogue {
    list: HashMap<String, Post>,
}

impl Catalogue {
    pub fn new_from_config(config: &ConfigFile) -> Self {
        let mut map = HashMap::new();
        let entries = fs::read_dir(&config.md_storage).unwrap();
        
        for doc in entries {
            let doc = doc.unwrap().path();
            
            let e = Post {
                path: doc.to_str().unwrap().to_owned(),
                name: doc.file_stem().unwrap().to_str().unwrap().to_owned(),
                file_name: doc.file_name().unwrap().to_str().unwrap().to_owned(),
                created: fs::metadata(doc).unwrap().created().unwrap(),
            };

            let name = e.name.clone();

            map.insert(name, e);
        }
        
        Self {
            list: map,
        }
    }

    pub fn all_posts_ordered(&self) -> Vec<&Post> {
        let mut list = Vec::new();

        for post in self.list.values() {
            list.push(post);
        }
        
        list.sort_by(|a, b| a.created.cmp(&b.created));
        list.reverse();
        list
    }

    pub fn get_post(&self, file_name: &str) -> &Post {
        self.list.get(file_name).unwrap()
    }

    pub fn posts_len(&self) -> usize {
        self.list.len()
    }
}

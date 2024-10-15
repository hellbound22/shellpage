mod convert;
mod fs_utils;
mod catalogue;
mod render;

use catalogue::Catalogue;
use convert::*;
use fs_utils::*;
use render::RenderEngine;

use std::{fs::{self, File}, io::{Read, Write}, path::PathBuf};
use std::process::Command;
use std::io;

use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use tera::Tera;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub action: Option<Action>,
}

#[derive(Subcommand)]
enum Action {
    NewPost {
        file_name: String,
    },
    Publish {
        #[arg(short, long)]
        all: bool,
        #[arg(short, long)]
        overwrite: bool,
        file_name: Option<String>,
    },
    UpdateIndex,
}

#[derive(Deserialize)]
pub struct ConfigFile {
    pub _sign_name: String,
    pub repo_path: String,
    pub md_storage: String,
    pub html_storage: String,
    pub template_path: String,
}

fn main() {
    let args = Cli::parse();

    let mut config_contents = String::new();
    let mut config_file = File::open(args.config.as_deref().unwrap()).unwrap();

    config_file.read_to_string(&mut config_contents).unwrap();

    let config: ConfigFile = toml::from_str(&config_contents).unwrap();

    match &args.action {
        Some(Action::NewPost {file_name}) => {
            let new_file_name = format!("{}.md", file_name);
            let mut new_file_path = config.md_storage.clone();
            new_file_path.push_str(&new_file_name);
            
            let _file = File::create_new(&new_file_path).unwrap();
            Command::new("nvim").arg(&new_file_path).status().unwrap();
        },
        Some(Action::Publish { all, file_name, overwrite }) => {
            let catalogue = Catalogue::new_from_config(&config);
            let render_engine = RenderEngine::new_from_config(&config);

            if *all {
                unimplemented!()
            } else {
                let file_name = file_name.as_deref().unwrap();
                let htmlfile_path = format!("{}{}.html", config.html_storage, file_name);

                //let post = catalogue.get_post(file_name);

                let html_source = md_to_html(&config, file_name);

                let render = render_engine.post(&html_source);

                if *overwrite {
                    write_overwrite(&htmlfile_path, &render);
                } else {
                    write_new(&htmlfile_path, &render);
                }
            }
        },
        UpdateIndex => {
            let catalogue = Catalogue::new_from_config(&config);
            let render_engine = RenderEngine::new_from_config(&config);

            let render = render_engine.index(&catalogue);

            let index_path = format!("{}index.html", config.repo_path);
            write_overwrite(&index_path, &render);
        }
        None => unimplemented!()
        
    }
}

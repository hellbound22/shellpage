mod convert;
mod fs_utils;
mod catalogue;
mod render;
mod errors;

use catalogue::Catalogue;
use convert::*;
use errors::ShellpageError;
use fs_utils::*;
use render::RenderEngine;

use std::env;
use std::{fs::File, io::Read};
use std::process::Command;

use clap::{crate_version, Parser, Subcommand};
use serde::Deserialize;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<String>,

    #[command(subcommand)]
    pub action: Option<Action>,
}

#[derive(Subcommand)]
enum Action {
    NewPost {
        file_name: Option<String>,
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        open: bool,
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
    pub editor: Option<String>,
}

fn main() -> Result<(), ShellpageError> {
    let version = crate_version!();
    println!("Shellpage {}", version); 
    let args = Cli::parse();

    let config_contents = if let Some(a) = args.config.as_deref() {
        let mut s = String::new();
        let mut config_file = File::open(a)?;
        config_file.read_to_string(&mut s)?;

        s
    } else {
        let path = env::current_dir()?;

        let mut s = String::new();
        let mut config_file = File::open(format!("{}/config.toml", path.to_str().unwrap()))?;
        config_file.read_to_string(&mut s)?;

        s
    };


    let config: ConfigFile = toml::from_str(&config_contents)?;

    println!("Indexing posts");
    let catalogue = Catalogue::new_from_config(&config);
    println!("Configuring render engine");
    let render_engine = RenderEngine::new_from_config(&config)?;

    match &args.action {
        Some(Action::NewPost { file_name, open }) => {
            let file_name = if let Some(f) = file_name.as_deref() {
                f
            } else {
                return Err(ShellpageError::RequiredArg("file_name".to_owned()));
            };

            let new_file_name = format!("{}.md", file_name);
            let mut new_file_path = config.md_storage.clone();
            new_file_path.push_str(&new_file_name);
            
            let _file = File::create_new(&new_file_path)?;

            if *open {
                if let Some(ed) = config.editor {
                    Command::new(ed).arg(&new_file_path).status()?;
                } else {
                    return Err(ShellpageError::RequiredConfigField("editor".to_owned()))
                }
            }
        },
        Some(Action::Publish { all, file_name, overwrite }) => {

            if *all {
                let all_posts = catalogue.all_posts_ordered();
                
                for post in &all_posts {
                    let htmlfile_path = format!("{}{}.html", config.html_storage, post.name);
                    let html_source = md_to_html(&config, &post.file_name)?;

                    let render = render_engine.post(&html_source, &post.created)?;

                    if *overwrite {
                        write_overwrite(&htmlfile_path, &render)?;
                    } else {
                        write_new(&htmlfile_path, &render)?;
                    }
                    println!("post {} converted", post.name);
                }
                println!("total {} posts converted", all_posts.len());
            } else {
                let file_name = if let Some(f) = file_name.as_deref() {
                    f
                } else {
                    return Err(ShellpageError::RequiredArg("file_name".to_owned()));
                };

                let cat_post = catalogue.get_post(&file_name);

                let htmlfile_path = format!("{}{}.html", config.html_storage, cat_post.name);
                let html_source = md_to_html(&config, file_name)?;

                let render = render_engine.post(&html_source, &cat_post.created)?;

                if *overwrite {
                    write_overwrite(&htmlfile_path, &render)?;
                } else {
                    write_new(&htmlfile_path, &render)?;
                }
                println!("post {} converted", cat_post.name);
            }

            let render = render_engine.index(&catalogue)?;

            let index_path = format!("{}index.html", config.repo_path);
            write_overwrite(&index_path, &render)?;
            println!("blog index updated: {} posts", catalogue.posts_len());
        },
        #[allow(unused_variables, non_snake_case)]
        UpdateIndex => {
            let render = render_engine.index(&catalogue)?;

            let index_path = format!("{}index.html", config.repo_path);
            write_overwrite(&index_path, &render)?;
            println!("blog index updated: {} posts", catalogue.posts_len());
        }
    }

    Ok(())
}


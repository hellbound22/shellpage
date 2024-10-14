use std::{fs::{self, File}, io::{Read, Write}, path::PathBuf};
use std::process::Command;
use std::io;

use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};
use markdown::to_html;
use tera::Tera;
use chrono::{DateTime, Utc};


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
struct Config {
    pub _sign_name: String,
    pub _repo_name: String,
    pub storage: PathBuf,
    pub template_path: PathBuf,
}

fn main() {
    let args =  Cli::parse();

    let mut config_contents = String::new();
    let mut config_file = File::open(args.config.as_deref().unwrap()).unwrap();

    config_file.read_to_string(&mut config_contents).unwrap();

    let config: Config = toml::from_str(&config_contents).unwrap();

    let mut md_storage = config.storage.clone();
    md_storage.push("md/");
    let mut html_storage = config.storage.clone();
    html_storage.push("html/");

    match &args.action {
        Some(Action::NewPost {file_name}) => {
            let new_file_name = format!("{}.md", file_name);
            let mut new_file_path = md_storage.clone();
            new_file_path.push(new_file_name);
            
            let _file = File::create_new(&new_file_path).unwrap();
            Command::new("nvim").arg(&new_file_path).status().unwrap();
        },
        Some(Action::Publish { all, file_name, overwrite }) => {
            if *all {
                unimplemented!()
            } else {
                let mut file_contents = String::new();
                let mdfile_path = format!("{}{}.md", md_storage.as_path().to_str().unwrap(), file_name.as_deref().unwrap());
                let htmlfile_path = format!("{}{}.html", html_storage.as_path().to_str().unwrap(), file_name.as_deref().unwrap());
                
                let mut file = File::open(mdfile_path).unwrap();
                file.read_to_string(&mut file_contents).unwrap();

                let html = to_html(&file_contents);

                let tera = Tera::new(config.template_path.as_path().to_str().unwrap()).unwrap();
                let mut context = tera::Context::new();
                
                let now: DateTime<Utc> = Utc::now();
                context.insert("date", &now.to_rfc2822());

                context.insert("post", &html);
                let render = tera.render("post.html.tera", &context).unwrap();

                if *overwrite {
                    let mut html_file = File::create(htmlfile_path).unwrap();
                    html_file.write_all(render.as_bytes()).unwrap();
                } else {
                    let mut html_file = File::create_new(htmlfile_path).unwrap();
                    html_file.write_all(render.as_bytes()).unwrap();
                }
            }
        },
        UpdateIndex => {
            let tera = Tera::new(config.template_path.as_path().to_str().unwrap()).unwrap();
            let mut context = tera::Context::new();
            
            let mut entries = fs::read_dir(html_storage).unwrap()
                .map(|res| res.map(|e| e.path()))
                .collect::<Result<Vec<_>, io::Error>>().unwrap();

            entries.sort_by_key(|path| {
                fs::metadata(path)
                    .and_then(|metadata| metadata.created())
                    .unwrap_or_else(|_| std::time::SystemTime::UNIX_EPOCH) // Fallback if creation time is not available.
            });
            entries.reverse();

            #[derive(Serialize, Deserialize)]
            struct Entry {
                path: String,
                name: String,
                file_name: String,
            }
            
            let mut posts = Vec::new();
            for e in entries {
                posts.push(Entry {
                    path: e.to_str().unwrap().to_owned(),
                    name: e.file_stem().unwrap().to_str().unwrap().to_owned(),
                    file_name: e.file_name().unwrap().to_str().unwrap().to_owned(),
                })
            }
            
            context.insert("posts", &posts);

            let render = tera.render("index.html.tera", &context).unwrap();

            let mut html_file = File::create(format!("{}index.html", config.storage.as_path().to_str().unwrap())).unwrap();
            html_file.write_all(render.as_bytes()).unwrap();
        }
        None => unimplemented!()
        
    }
}

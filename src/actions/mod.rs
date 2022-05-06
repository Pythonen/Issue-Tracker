use config::{Config};
use walkdir::{DirEntry, WalkDir};
use std::{fs, collections::HashSet, env::current_dir};

use crate::util::{ignore_files, filter_files};

pub fn init_new_project() -> Config {
    match Config::builder()
            .add_source(config::File::with_name(".it.toml"))
            .build() 
            {
                Ok(cfg) => {
                    println!("Config file found... using it from now on!");
                    return cfg;
                }
                Err(_) => {
                    println!("No config file found, creating a new one...");
                    let comment = "# This is your project specific config file for *it* (issue tracker).\n\
                                  # You can add your own custom settings here in TOML format.";
                    fs::write(".it.toml", comment).unwrap();
                    // TODO: better error handling
                    let cfg = Config::builder().add_source(config::File::with_name(".it.toml")).build().unwrap();
                    println!("Config file created!");
                    return cfg;
                }
            }
}
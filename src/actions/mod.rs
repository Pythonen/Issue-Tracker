use config::{Config};
use std::fs;
pub fn init_new_project() {
    println!("Initializing new project...");
    match Config::builder()
            .add_source(config::File::with_name(".it"))
            .build() 
            {
                Ok(cfg) => {
                    // config file found
                    println!("{:?}", cfg);
                }
                Err(_) => {
                    println!("No config file found, creating a new one...");
                    let comment = "# This is the config file for *it* (issue tracker).\n\
                                  # You can add your own custom settings here in TOML format.";
                    fs::write(".it.toml", comment).unwrap();
                    Config::builder().add_source(config::File::with_name(".it.toml")).build().unwrap();
                }
            }
}
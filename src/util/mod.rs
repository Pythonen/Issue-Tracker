use std::path::{Path, PathBuf};

use config::Config;
use dialoguer::{Input, Password};

pub fn filter_files(ignore: &Vec<String>, path: &Path) -> bool {
    let path_str = path.to_str().unwrap().to_string();
    !ignore.iter().any(|x| path_str.contains(x))
}

pub fn create_toml_stub(username: &str, password: &str) -> String {
    return format!(
        "# This is your global config file for *it* (issue tracker).\n\
                            # You can add your own custom settings here in TOML format.\n\
                            [account]\n\
                            username = \"{}\"\n\
                            password = \"{}\"",
        username, password
    );
}

pub fn get_creds() -> (String, String) {
    let username = Input::<String>::new()
        .with_prompt("Enter your username")
        .interact_text()
        .unwrap();
    let password = Password::new()
        .with_prompt("Enter your password")
        .interact()
        .unwrap();
    return (username, password);
}

pub fn ignore_files(cur_dir: &PathBuf) -> Option<Vec<String>> {
    for entry in cur_dir.read_dir().unwrap() {
        if let Ok(entry) = entry {
            // if file is gitignore, read its contents
            // TODO: add support for other ignore files
            if entry.path().to_str().unwrap().ends_with(".gitignore") {
                let contents = std::fs::read_to_string(entry.path()).unwrap();
                // append to ignore folders from contents
                let mut ignore: Vec<String> = contents
                    .split("\n")
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>();
                // Ignore .git folder
                ignore.push(".git".to_string());
                return Some(ignore);
            }
            continue;
        }
        continue;
    }
    return None;
}

pub fn get_config_file() -> Config {
    return Config::builder()
        .add_source(config::File::with_name(".it.toml"))
        .build()
        .unwrap();
}

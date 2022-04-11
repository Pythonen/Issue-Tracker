use std::{env::current_dir, path::Path};
use walkdir::WalkDir;

#[tokio::main]
async fn main() {
    // get files from current directory
    let cur_dir = current_dir().unwrap();
    if let Some(asdf) = ignore_files(&cur_dir) {
        WalkDir::new(".")
        .into_iter()
        .filter_entry(|e| filter_files(&asdf, e.path()) || !e.path().display().to_string().starts_with("."))
        .filter_map(|v| v.ok())
        .for_each(|x| println!("{}", x.path().display()));
    }
}

fn filter_files(ignore: &Vec<String>, path: &Path) -> bool {
    let path_str = path.display().to_string();
    !ignore.iter().any(|x| path_str.contains(x))
}


fn ignore_files(cur_dir: &std::path::PathBuf) -> Option<Vec<String>> {
    for entry in cur_dir.read_dir().unwrap() {
        if let Ok(entry) = entry {
            // if file is gitignore, read its contents
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

use std::env::current_dir;

#[tokio::main]
async fn main() {
    // get files from current directory
    if let Some(asdf) = ignore_files() {
        println!("{:?}", asdf);
    }
}

fn ignore_files() -> Option<Vec<String>> {
    let path = current_dir().unwrap();
    for entry in path.read_dir().unwrap() {
        if let Ok(entry) = entry {
            // if file is gitignore, read its contents
            if entry.path().to_str().unwrap().ends_with(".gitignore") {
                let contents = std::fs::read_to_string(entry.path()).unwrap();
                // append to ignore folders from contents
                let ignore: Vec<String> = contents
                    .split("\n")
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>();
                return Some(ignore);
            }
            continue;
        }
        continue;
    }
    return None;
}

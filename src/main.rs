use std::{env::current_dir, path::Path, collections::HashSet};
use walkdir::{WalkDir, DirEntry};

#[tokio::main]
async fn main() {
    // get files from current directory
    let cur_dir = current_dir().unwrap();
    if let Some(to_ignore) = ignore_files(&cur_dir) {
        WalkDir::new(".")
        .into_iter()
        .filter_entry(|e| filter_files(&to_ignore, e.path()))
        .filter_map(|v| v.ok())
        .for_each(|x| {
            let todos = find_todos(x);
            if todos.len() > 0 {
                for todo in todos {
                    println!("\t{}", todo);
                }
            } 
        });
    }
}

fn filter_files(ignore: &Vec<String>, path: &Path) -> bool {
    let path_str = path.display().to_string();
    !ignore.iter().any(|x| path_str.contains(x))
}

fn find_todos(dir: DirEntry) -> HashSet<String> {
    let path = dir.path();
    let mut unique_todos: HashSet<String> = HashSet::new();
    if path.is_file() {
        let contents = std::fs::read_to_string(path).unwrap();
        let todos: Vec<_> = contents.match_indices("TODO").collect();
        if todos.len() > 0 {
            for (_, todo) in todos.iter().enumerate() {
                let mut todo_str = String::new();
                for c in contents.chars().skip(todo.0).take_while(|c| c != &'\n') {
                    todo_str.push(c);
                }
                unique_todos.insert(todo_str);
            }
        }
    }
    return unique_todos;
}

fn ignore_files(cur_dir: &std::path::PathBuf) -> Option<Vec<String>> {
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

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
                                  # You can add your own custom settings here in TOML format.\n\
                                  project = \"\"";
                    fs::write(".it.toml", comment).unwrap();
                    // TODO: better error handling
                    let cfg = Config::builder().add_source(config::File::with_name(".it.toml")).build().unwrap();
                    println!("Config file created!");
                    return cfg;
                }
            }
}

fn find_todos_wrapper(fun: fn(&String) -> ()) -> Vec<String> {
    // TODO: Could be refactored to be a bit more efficient
    // as it now returns the todos everytime
    let cur_dir = current_dir().unwrap();
    let mut opt_todos = vec![];
    if let Some(to_ignore) = ignore_files(&cur_dir) {
        WalkDir::new(".")
        .into_iter()
        .filter_entry(|e| filter_files(&to_ignore, e.path()))
        .filter_map(|v| v.ok())
        .for_each(|x| {
            let todos = find_todos(x);
            if todos.len() > 0 {
                for todo in todos {
                    fun(&todo);
                    opt_todos.push(todo);
                }
            }
        })
    }
    return opt_todos;
}

pub async fn report_todos() {
    let client = reqwest::Client::new();
    let todos = find_todos_wrapper(|_| {});
    match client
        // TODO: Read from env by appending the project name
        // to the URL etc.
        .post("http://localhost:8080/report")
        .json(&todos)
        .send()
        .await {
            Ok(_) => println!("Todos reported!"),
            Err(e) => println!("Something went wrong: {}", e),
        }
}

pub fn print_todos() {
    // TODO: Make formatting of this more userfriendly and pretty
    find_todos_wrapper(|todo| {
        println!("{:_^40}", "TODOS");
        println!("{}", todo);
        println!("\n----------------------------------------\n");
    });
}

fn find_todos(dir: DirEntry) -> HashSet<String> {
    let path = dir.path();
    let mut unique_todos: HashSet<String> = HashSet::new();
    if path.is_file() && path.extension().is_some() {
        if let Ok(contents) = std::fs::read_to_string(path) {
            let todos: Vec<_> = contents.match_indices("TODO").collect();
            if todos.len() > 0 {
                for (_, todo) in todos.iter().enumerate() {
                    let mut todo_str = String::new();
                    let file_for_todo = format!("{}: ", path.display());
                    todo_str.push_str(&file_for_todo);
                    for c in contents.chars().skip(todo.0).take_while(|c| c != &'\n') {
                        todo_str.push(c);
                    }
                    unique_todos.insert(todo_str);
                }
            }
        }
    }
    return unique_todos;
}
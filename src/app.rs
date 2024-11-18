use std::path::PathBuf;
use std::collections;
use std::fs;

pub struct App {
    pub start_dir: PathBuf,
    pub current_dir: PathBuf,
    pub user_input: String,
    pub output: OutputType,
    pub subdirectories: Vec<PathBuf>,
    pub show_hidden: bool,
}

impl App {
    pub fn new(current_dir: PathBuf, show_hidden: bool) -> App {
        let mut new_app = App {
            start_dir: current_dir.clone(),
            current_dir,
            user_input: String::new(),
            output: OutputType::Current,
            subdirectories: vec![],
            show_hidden,
        };
        new_app.update_subdirectories();
        new_app
    }

    pub fn set_current_dir(&mut self, dir: PathBuf) {
        self.current_dir = dir;
        self.update_subdirectories();
    }

    pub fn update_subdirectories(&mut self) {
        // remove all entries from subdirectories
        self.subdirectories.clear();
        // find all the subdirectories within the current_dir add them to subdirectories, if
        if let Ok(entries) = fs::read_dir(&self.current_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        if !self.show_hidden {
                            if let Some(name) = path.file_name() {
                                if name.to_str().map_or(false, |s| s.starts_with('.')) {
                                    continue;
                                }
                            }
                        }
                        self.subdirectories.push(path);
                    }
                }
            }
        }
    }

    pub fn delete_input_letter(&mut self) {
        self.user_input.pop();
    }
}

pub enum OutputType {
    Start,
    Current
}
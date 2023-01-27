#![allow(dead_code)]

use anyhow::Error;
use just_macros::{crashln, ternary};
use std::path::Path;

pub fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

pub struct Exists;
impl Exists {
    pub fn folder(dir_name: String) -> Result<bool, Error> {
        Ok(Path::new(string_to_static_str(dir_name)).is_dir())
    }
    pub fn file(file_name: String) -> Result<bool, Error> {
        Ok(Path::new(string_to_static_str(file_name)).exists())
    }
}

pub fn create_default_kv() {
    match home::home_dir() {
        Some(path) => {
            if !Exists::folder(format!("{}/.mango", path.display())).unwrap() {
                std::fs::create_dir_all(format!("{}/.mango", path.display())).unwrap();
                println!("created {}/.mango", &path.display());
            }
        }
        None => {
            crashln!("Home directory could not be found.");
        }
    }
}

pub fn get_default_kv() -> String {
    match home::home_dir() {
        Some(path) => {
            log::info!("Store dir: {}/.mango", path.display());
            format!("{}/.mango", path.display())
        }
        None => {
            crashln!("Home directory could not be found.");
        }
    }
}

pub fn store_error(path: &String, error: &str, alt: &str) -> String {
    ternary!(path != &get_default_kv(), format!("{error}, {alt}."), format!("{error}."))
}

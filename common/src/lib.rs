use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    env,
    path::Path,
};

pub fn read_arguments() -> (Vec<String>, HashSet<String>) {
    let regex = Regex::new(r"^-{1,2}").unwrap();
    let arguments = env::args()
        .skip(1)
        .filter(|arg| !arg.starts_with('-'))
        .collect();
    let flags = env::args()
        .filter(|arg| arg.starts_with('-'))
        .map(|arg| regex.replace(&arg, "").to_string())
        .collect();

    (arguments, flags)
}

pub fn get_current_directory() -> String {
    env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()
}

pub fn set_current_directory(directory: &str) -> bool {
    let path = Path::new(directory);

    env::set_current_dir(path).is_ok()
}

pub fn get_path_files() -> HashMap<String, String> {
    let path = get_env_path();
    let path_files_names = get_files_from_directories(&path, true, true).unwrap();
    let path_files_paths = get_files_from_directories(&path, false, true).unwrap();
    let path_files = path_files_names.into_iter().zip(path_files_paths).collect();

    path_files
}

pub fn get_env_path() -> Vec<String> {
    let mut env_path: Vec<String> = env::var("PATH")
        .unwrap_or_default()
        .split(":")
        .map(String::from)
        .collect();

    let custom_env_path = env::current_exe().unwrap();
    let custom_env_path = custom_env_path
        .parent()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    env_path.push(custom_env_path);
    env_path
}

pub fn get_files_from_directory(directory: &str, name_only: bool) -> Option<Vec<String>> {
    let directory = if directory.is_empty() {
        get_current_directory()
    } else {
        directory.to_string()
    };

    let full_path = Path::new(&directory);
    if !full_path.exists() {
        return None;
    }

    let mut all_files = Vec::new();
    if let Ok(files) = Path::read_dir(full_path) {
        for file in files {
            let file = file.unwrap();
            let os_string = if name_only {
                file.file_name()
            } else {
                file.path().into_os_string()
            };
            let string = os_string.into_string().unwrap_or_default();

            all_files.push(string);
        }
    }

    Some(all_files)
}

pub fn get_files_from_directories(
    directories: &Vec<String>,
    name_only: bool,
    ignore_non_existing: bool,
) -> Option<Vec<String>> {
    if directories.len() == 0 {
        return get_files_from_directory("", name_only);
    }

    let all_files: Vec<Option<Vec<String>>> = directories
        .iter()
        .map(|directory| get_files_from_directory(directory, name_only))
        .collect();

    let found_all = all_files.iter().all(|result| result.is_some());

    let collected: Vec<String> = all_files
        .into_iter()
        .filter_map(|result| result)
        .flatten()
        .collect();

    if found_all || ignore_non_existing {
        Some(collected)
    } else {
        None
    }
}

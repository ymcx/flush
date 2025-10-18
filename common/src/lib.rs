use std::{collections::HashMap, env, path::Path};

fn get_current_directory() -> String {
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
    let path_files_names = get_files_from_directories(&path, true);
    let path_files_paths = get_files_from_directories(&path, false);
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

pub fn get_files_from_directory(directory: &str, name_only: bool) -> Vec<String> {
    let directory = if directory.is_empty() {
        get_current_directory()
    } else {
        directory.to_string()
    };

    let full_path = Path::new(&directory);
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

    all_files
}

pub fn get_files_from_directories(directories: &Vec<String>, name_only: bool) -> Vec<String> {
    if directories.len() == 0 {
        return get_files_from_directory("", name_only);
    }

    directories
        .iter()
        .flat_map(|directory| get_files_from_directory(directory, name_only))
        .collect()
}

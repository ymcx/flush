use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    env,
    path::Path,
};

pub mod file;

pub fn read_arguments() -> (Vec<String>, HashSet<String>) {
    let arguments = env::args()
        .skip(1)
        .filter(|arg| !arg.starts_with('-'))
        .collect();

    if let Ok(regex) = Regex::new(r"^-{1,2}") {
        let flags = env::args()
            .filter(|arg| arg.starts_with('-'))
            .map(|arg| regex.replace(&arg, "").to_string())
            .collect();

        return (arguments, flags);
    }

    return (arguments, HashSet::new());
}

pub fn get_current_directory() -> Option<String> {
    let current_directory = env::current_dir()
        .ok()?
        .into_os_string()
        .into_string()
        .ok()?;

    Some(current_directory)
}

pub fn set_current_directory(directory: &str) -> bool {
    let path = Path::new(directory);

    env::set_current_dir(path).is_ok()
}

pub fn get_env_path_files(builtin: bool, external: bool) -> HashMap<String, String> {
    let path = get_env_path();
    let path_files_names = get_files_from_directories(&path, true, builtin, external);
    let path_files_paths = get_files_from_directories(&path, false, builtin, external);
    let path_files = path_files_names.into_iter().zip(path_files_paths).collect();

    path_files
}

fn get_custom_env_path() -> Option<String> {
    let custom_env_path = env::current_exe().ok()?.parent()?.to_str()?.to_string();

    Some(custom_env_path)
}

fn get_env_path() -> Vec<String> {
    let mut env_path: Vec<String> = env::var("PATH")
        .unwrap_or_default()
        .split(":")
        .map(String::from)
        .collect();

    if let Some(custom_env_path) = get_custom_env_path() {
        env_path.push(custom_env_path);
    }

    env_path
}

pub fn parse_env_path_commands(path_files: &HashMap<String, String>) -> String {
    path_files
        .keys()
        .map(String::from)
        .collect::<Vec<String>>()
        .join(" ")
}

fn get_files_from_directory(
    directory: &str,
    name_only: bool,
    builtin: bool,
    external: bool,
) -> Vec<String> {
    let path = Path::new(directory);
    let mut all_files = Vec::new();

    if (!builtin && Some(directory.to_string()) == get_custom_env_path())
        || (!external && Some(directory.to_string()) != get_custom_env_path())
    {
        return all_files;
    }

    if let Ok(files) = Path::read_dir(path) {
        for file in files {
            let file = match file {
                Ok(file) => file,
                Err(_) => continue,
            };

            let mode = match file::get_metadata(&file) {
                Some(metadata) => file::get_mode(&metadata),
                None => continue,
            };

            if mode.contains('d') || !mode.contains('x') {
                continue;
            }

            let file_string = if name_only {
                file.file_name()
            } else {
                file.path().into_os_string()
            };

            if let Ok(file_string) = file_string.into_string() {
                all_files.push(file_string);
            }
        }
    }

    all_files
}

fn get_files_from_directories(
    directories: &Vec<String>,
    name_only: bool,
    builtin: bool,
    external: bool,
) -> Vec<String> {
    directories
        .iter()
        .flat_map(|directory| get_files_from_directory(directory, name_only, builtin, external))
        .collect()
}

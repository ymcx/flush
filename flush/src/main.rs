use common;
use reedline::{DefaultPrompt, Reedline, Signal};
use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::process::Command;

fn get_path_files() -> HashMap<String, String> {
    let path = common::get_env_path();
    let path_files_names = common::get_files_from_directories(&path, true);
    let path_files_paths = common::get_files_from_directories(&path, false);
    let path_files = path_files_names.into_iter().zip(path_files_paths).collect();

    path_files
}

fn handle_command_successful(buffer: &str, path_files: &HashMap<String, String>) -> bool {
    let parts: Vec<&str> = buffer.split_ascii_whitespace().collect();

    if let Some((&command, arguments)) = parts.split_first() {
        if command == "cd" {
            let directory = arguments[0];
            let path = Path::new(directory);

            return env::set_current_dir(path).is_ok();
        }

        if let Some(command) = path_files.get(command) {
            let output = Command::new(command).args(arguments).output().unwrap();
            let stdout = String::from_utf8_lossy(&output.stdout);

            print!("{}", stdout);

            return true;
        }
    }

    false
}

fn handle_command(
    reedline: &mut Reedline,
    prompt: &DefaultPrompt,
    path_files: &HashMap<String, String>,
) -> bool {
    match reedline.read_line(prompt) {
        Ok(Signal::Success(buffer)) => handle_command_successful(&buffer, path_files),
        _ => false,
    }
}

fn main() {
    let mut reedline = Reedline::create();
    let prompt = DefaultPrompt::default();
    let path_files = get_path_files();

    while handle_command(&mut reedline, &prompt, &path_files) {}
}

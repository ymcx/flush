fn main() {
    let path_files_external = common::get_path_files(false, true);
    let path_files_builtin = common::get_path_files(true, false);

    let commands_external = common::parse_path_files(&path_files_external);
    let commands_builtin = common::parse_path_files(&path_files_builtin);

    println!("External commands:");
    println!("{}", commands_external);
    println!();
    println!("Built-in commands:");
    println!("{}", commands_builtin);
}

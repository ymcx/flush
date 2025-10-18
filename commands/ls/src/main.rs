fn main() {
    let (arguments, _flags) = common::read_arguments();
    let files = common::get_files_from_directories(&arguments, true, false);

    if let Some(string) = files {
        println!("{}", string.join(" "));
    } else {
        println!("No such file or directory");
    }
}

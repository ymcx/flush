fn main() {
    let path_files = common::get_path_files();
    for path_file in path_files.keys() {
        println!("{}", path_file);
    }
}

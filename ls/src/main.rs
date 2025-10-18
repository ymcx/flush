use common;
use std::env;

fn main() {
    let arguments = env::args().skip(1).collect();
    let files = common::get_files_from_directories(&arguments, true).join(" ");

    println!("{}", files);
}

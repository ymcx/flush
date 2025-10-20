fn main() {
    if let Some(pwd) = common::get_current_directory() {
        println!("{}", pwd);
    } else {
        println!("Couldn't retrieve the current directory");
    }
}

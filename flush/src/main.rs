use common::{self};
use reedline::{DefaultPrompt, Reedline};

mod handle;

fn main() {
    let mut reedline = Reedline::create();
    let prompt = DefaultPrompt::default();
    let path_files = common::get_path_files();

    while handle::command(&mut reedline, &prompt, &path_files) {}
}

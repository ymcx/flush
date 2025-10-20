use reedline::{DefaultPrompt, DefaultPromptSegment, Reedline};

mod handle;

fn main() {
    let mut reedline = Reedline::create();
    let prompt = DefaultPrompt::new(
        DefaultPromptSegment::WorkingDirectory,
        DefaultPromptSegment::Empty,
    );
    let path_files = common::get_path_files(true, true);

    while handle::command(&mut reedline, &prompt, &path_files) {}
}

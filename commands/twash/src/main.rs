use rand::{Rng, distr::Alphanumeric, rngs::ThreadRng};
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
};

fn get_random_string(rng: &mut ThreadRng, length: usize) -> String {
    rng.sample_iter(Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

fn read_file(path: &str) -> Option<String> {
    let mut file = File::open(path).ok()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).ok()?;

    Some(contents)
}

fn write_file(path: &str, contents: &str) -> Option<()> {
    let mut file = OpenOptions::new().write(true).open(path).ok()?;
    file.write_all(contents.as_bytes()).ok()
}

fn trash_file(rng: &mut ThreadRng, path: &str) -> Option<()> {
    let mut contents_new = String::new();

    for line in read_file(path)?.split("\n") {
        let line_new = get_random_string(rng, line.len());
        contents_new.push_str(&line_new);
        contents_new.push('\n')
    }
    contents_new.pop();

    write_file(path, &contents_new)
}

fn main() {
    let (arguments, _) = common::read_arguments();
    let mut rng = rand::rng();

    for argument in arguments {
        if trash_file(&mut rng, &argument).is_none() {
            println!("Unable to trash file {}", argument);
            break;
        }
    }
}

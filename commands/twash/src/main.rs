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

fn read_file(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents
}

fn write_file(path: &str, contents: &str) {
    let mut file = OpenOptions::new().write(true).open(path).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}

fn trash_file(rng: &mut ThreadRng, path: &str) {
    let mut contents_new = String::new();

    for line in read_file(path).split("\n") {
        let line_new = get_random_string(rng, line.len());
        contents_new.push_str(&line_new);
        contents_new.push('\n')
    }
    contents_new.pop();

    write_file(path, &contents_new);
}

fn main() {
    let (arguments, _) = common::read_arguments();
    let mut rng = rand::rng();

    for argument in arguments {
        trash_file(&mut rng, &argument);
    }
}

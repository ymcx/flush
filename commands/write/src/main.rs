use std::io;

fn main() {
    loop {
        let mut input = String::default();
        let _ = io::stdin().read_line(&mut input);

        if input.trim() == "q" {
            break;
        }
    }
}

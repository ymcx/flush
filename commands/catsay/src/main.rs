fn main() {
    let (arguments, _) = common::read_arguments();
    let text = arguments.join(" ");

    println!(
        "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
        format!("               _--{}---_  ", "-".repeat(text.len())),
        format!("              /   {}    \\", " ".repeat(text.len())),
        format!("             |    {}    | ", text),
        format!("    _---_    |  __{}__.-  ", "_".repeat(text.len())),
        format!("  ／＞　 フ   \\/         "),
        format!("  | 　_　_|               "),
        format!("／` ミ＿xノ               "),
        format!("／　　　　 |              "),
        format!("(　 ヽ＿ヽ_)__)           "),
        format!("＼二つ                    "),
    );
}

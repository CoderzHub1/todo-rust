use std::io::{Write, stdin, stdout};
use owo_colors::OwoColorize;

pub fn prompts(msg: &str) -> String {
    let mut cmd = String::new();
    print!("{}", msg.red().bold());
    stdout().flush().expect("failed to flush stdout");
    stdin().read_line(&mut cmd).expect("Cant stdin read_line");

    return cmd.trim().to_string();
}

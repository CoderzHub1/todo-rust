use std::io::{Write, stdin, stdout};

pub fn prompts(msg: &str) -> String {
    let mut cmd = String::new();
    print!("{msg}");
    stdout().flush().expect("failed to flush stdout");
    stdin().read_line(&mut cmd).expect("Cant stdin read_line");

    return cmd.trim().to_string();
}

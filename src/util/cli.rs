use std::process::exit;
use std::io::stdin;

pub fn read_user_input(input_msg: &str) -> String {
    println!("{}", input_msg);
    let mut buf = String::new();
    let _ = stdin().read_line(&mut buf);
    return buf.trim().to_string();
}


pub fn show_usage_and_exit() {
    println!("Usage: rs-todo list|add|finished <args>");
    exit(-1);
}
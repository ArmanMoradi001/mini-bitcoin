use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <op> <text>", args[0]);
        exit(1);
    }

    let op = &args[1];
    let text = &args[2];
    println!("op: {op} text: {text}");
}

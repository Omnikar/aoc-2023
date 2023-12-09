mod days;
mod macros;

use std::io::{stdin, Read};

fn main() {
    let mut args = std::env::args().skip(1).peekable();
    let time = args.peek().map(String::as_ref) == Some("--time");
    if time {
        args.next();
    }
    let day = args.next().expect("missing day");
    let part = args.next().expect("missing part");

    let func = *days::DAYS
        .get(day.as_str())
        .expect("day does not exist")
        .get(part.as_str())
        .expect("part does not exist");

    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let start = std::time::Instant::now();
    let answer = func(&input);
    println!("{}", answer);
    let end = std::time::Instant::now();
    let elapsed = end - start;
    if time {
        eprintln!("took {}μs", elapsed.as_micros());
    }
}

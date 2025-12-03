use std::env;

pub mod days;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Takes 2 arguments");
        return;
    }
    let input = args[1].parse::<u32>().expect("Expected u32 input as argument");

    days::solve(input);
}

use std::env;
mod solve01;
mod solve02;
mod solve03;

fn main() {
    let args: Vec<String> = env::args().collect();
    let problem = args.get(1).expect("Please enter a problem");

    match problem.as_ref() {
        "01" => solve01::main(),
        "02" => solve02::main(),
        "03" => solve03::main(),
        _ => println!("Unknown problem '{}'", problem),
    }
}
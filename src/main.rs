use std::env;
mod solve01;
mod solve02;
mod solve03;
mod solve04;
mod solve05;
mod solve06;
mod solve07;
mod solve08;
mod solve09;
mod solve10;

fn main() {
    let args: Vec<String> = env::args().collect();
    let problem = args.get(1).expect("Please enter a problem");

    match problem.as_ref() {
        "01" => solve01::main(),
        "02" => solve02::main(),
        "03" => solve03::main(),
        "04" => solve04::main(),
        "05" => solve05::main(),
        "06" => solve06::main(),
        "07" => solve07::main(),
        "08" => solve08::main(),
        "09" => solve09::main(),
        "10" => solve10::main(),
        _ => println!("Unknown problem '{}'", problem),
    }
}
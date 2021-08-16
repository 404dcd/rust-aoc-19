use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let file = File::open("input01.txt").unwrap();
    let reader = BufReader::new(file);
    let mut total1 = 0;
    let mut total2 = 0;
    for mass in reader.lines().map(|s| s.unwrap().parse::<i32>().unwrap()) {
        let mut fuel = mass / 3 - 2;
        total1 += fuel;
        while fuel > 0 {
            total2 += fuel;
            fuel = fuel / 3 - 2;
        }
    }
    println!("Part 1: {}", total1);
    println!("Part 2: {}", total2);
}
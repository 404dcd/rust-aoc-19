use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn main() {
    let file = File::open("assets/input04.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    reader.read_line(&mut buf).unwrap();
    let (start, stop) = buf.split_once("-").unwrap();
    let (start, stop): (i32, i32) = (start.trim().parse().unwrap(), stop.trim().parse().unwrap());

    let mut count1 = 0;
    let mut count2 = 0;
    'outer: for mut x in start..stop {
        let mut good1 = false;
        let mut good2 = false;
        let mut seen = 10;
        let mut times = 0;
        while x > 0 {
            let digit = x % 10;
            if seen < digit {
                continue 'outer
            }
            if seen == digit {
                times += 1;
                if times == 2 {
                    good1 = true
                }
            } else {
                if times == 2 {
                    good2 = true
                }
                times = 1;
                seen = digit
            }
            x /= 10;
        }
        if good1 {
            count1 += 1
        }
        if good2 || (times == 2) {
            count2 += 1;
        }
    }
    println!("Part 1: {}", count1);
    println!("Part 2: {}", count2)
}
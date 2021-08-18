use std::collections::HashMap;
use std::fs::read_to_string;

pub fn main() {
    let orbitsraw = read_to_string("assets/input06.txt").unwrap();
    let orbits: HashMap<&str, &str> = orbitsraw.lines().map(|s| s.split_once(")").unwrap()).map(|(a, b)| (b, a)).collect();

    let mut count = 0;

    for &key in orbits.keys() {
        let mut newkey = key;
        while newkey != "COM" {
            newkey = orbits.get(newkey).unwrap();
            count += 1
        }
    }

    let mut pathyou: Vec<&str> = Vec::new();
    let mut pathsan: Vec<&str> = Vec::new();
    let mut newkey = "YOU";
    while newkey != "COM" {
        newkey = orbits.get(newkey).unwrap();
        pathyou.push(newkey);
    }
    let mut newkey = "SAN";
    while newkey != "COM" {
        newkey = orbits.get(newkey).unwrap();
        pathsan.push(newkey);
    }

    while pathsan.pop() == pathyou.pop() {};

    println!("Part 1: {}", count);
    println!("Part 2: {}", pathyou.len() + pathsan.len() + 2)

}
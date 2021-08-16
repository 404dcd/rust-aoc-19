use std::io::{BufRead, BufReader};
use std::fs::File;

fn run(mut program: Vec<usize>) -> Result<usize, &'static str> {
    let mut ptr = 0;
    loop {
        match program[ptr] {
            1 => {
                let result = program[program[ptr+1]] + program[program[ptr+2]];
                let write = program[ptr+3];
                program[write] = result;
            }
            2 => {
                let result = program[program[ptr+1]] * program[program[ptr+2]];
                let write = program[ptr+3];
                program[write] = result;
            }
            99 => {break}
            _ => {return Err("unknown opcode")}
        }
        ptr += 4;
    }
    Ok(program[0])
}

pub fn main() {
    let file = File::open("assets/input02.txt").unwrap();
    let reader = BufReader::new(file);

    let mut program: Vec<usize> = reader.split(b',')
        .map(|s| String::from_utf8(s.unwrap()).unwrap().parse::<usize>().unwrap())
        .collect();
    
    let mut result = (0, 0);
    for noun in 0..100 {
        for verb in 0..100 {
            let mut arg = program.clone();
            arg[1] = noun;
            arg[2] = verb;
            if let Ok(output) = run(arg) {
                if output == 19690720 {
                    result = (noun, verb);
                    break;
                }
            }
        }
    }
    
    program[1] = 12;
    program[2] = 2;
    println!("Part 1: {}", run(program).unwrap());
    println!("Part 2: {}", 100 * result.0 + result.1)
}
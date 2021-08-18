use std::io::{BufRead, BufReader};
use std::fs::File;

fn run(mut program: Vec<i32>, input: Vec<i32>) -> Result<Vec<i32>, String> {
    let mut ptr = 0;
    let mut inptr = 0;
    let mut output = Vec::new();
    loop {
        let opcode = program[ptr] % 100;
        let width = match opcode {
            3 | 4 | 99 => 2,
            5 | 6 => 3,
            1 | 2 | 7 | 8 => 4,
            _ => return Err(format!("unknown opcode {}", opcode))
        };

        let mut prmptrs = Vec::with_capacity(4); // save realloc
        let mut flags: Vec<char> = (program[ptr] / 100).to_string().chars().rev().collect();
        while flags.len() < width-1 {
            flags.push('0')
        }
        for i in 1..width {
            prmptrs.push(match flags[i-1] {
                '0' => program[ptr+i] as usize,
                '1' => ptr+i,
                _ => return Err(format!("unknown parameter mode"))
            })
        }

        ptr += width;

        match opcode {
            1 => {
                program[prmptrs[2]] = program[prmptrs[0]] + program[prmptrs[1]]
            }
            2 => {
                program[prmptrs[2]] = program[prmptrs[0]] * program[prmptrs[1]]
            }
            3 => {
                program[prmptrs[0]] = *input.get(inptr).ok_or("out of input")?;
                inptr += 1
            }
            4 => {
                output.push(program[prmptrs[0]])
            }
            5 => {
                if program[prmptrs[0]] != 0 {
                    ptr = program[prmptrs[1]] as usize
                }
            }
            6 => {
                if program[prmptrs[0]] == 0 {
                    ptr = program[prmptrs[1]] as usize
                }
            }
            7 => {
                program[prmptrs[2]] = if program[prmptrs[0]] < program[prmptrs[1]] {1} else {0}
            }
            8 => {
                program[prmptrs[2]] = if program[prmptrs[0]] == program[prmptrs[1]] {1} else {0}
            }
            99 => {
                break
            }
            _ => {
                return Err(format!("unknown opcode"))
            }
        }
    }
    Ok(output)
}

pub fn main() {
    let file = File::open("assets/input05.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    reader.read_line(&mut buf).unwrap();
    let program: Vec<i32> = buf.split(",").map(|s| s.trim().parse().unwrap()).collect();
    
    println!("Part 1: {}", run(program.clone(), Vec::from([1])).unwrap().last().unwrap());
    println!("Part 2: {}", run(program, Vec::from([5])).unwrap().first().unwrap())
}
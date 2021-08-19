use std::io::{BufRead, BufReader};
use std::fs::File;
use std::sync::mpsc::{channel, Sender, Receiver};

fn run(mut program: Vec<i64>, input: Receiver<i64>, output: Sender<i64>) -> Result<i64, String> {
    let mut ptr = 0;
    let mut base = 0;
    let mut lastout = 0;
    loop {
        let opcode = program[ptr] % 100;
        let width = match opcode {
            99 => 1,
            3 | 4 | 9 => 2,
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
            let addr = match flags[i-1] {
                '0' => program[ptr+i] as usize,
                '1' => ptr+i,
                '2' => (program[ptr+i] + base) as usize,
                _ => return Err(format!("unknown parameter mode"))
            };
            while addr >= program.len() {
                program.push(0)
            }
            prmptrs.push(addr)
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
                program[prmptrs[0]] = input.recv().expect("need input")
            }
            4 => {
                lastout = program[prmptrs[0]];
                // A send operation can only fail if the receiving end of a channel is disconnected
                if let Err(_) = output.send(program[prmptrs[0]]) {
                    break
                }
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
            9 => {
                base += program[prmptrs[0]]
            }
            99 => {
                break
            }
            _ => {
                return Err(format!("unknown opcode"))
            }
        }
    }
    Ok(lastout)
}

pub fn main() {
    let file = File::open("assets/input09.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    reader.read_line(&mut buf).unwrap();
    let program: Vec<i64> = buf.split(",").map(|s| s.trim().parse().unwrap()).collect();

    let (tx_in, rx_in): (Sender<i64>, Receiver<i64>) = channel();
    let (tx_out, rx_out): (Sender<i64>, Receiver<i64>) = channel();
    tx_in.send(1).unwrap();
    run(program.clone(), rx_in, tx_out).unwrap();
    let mut part1 = 0;
    for item in rx_out {
        part1 = item
    }

    let (tx_in, rx_in): (Sender<i64>, Receiver<i64>) = channel();
    let (tx_out, rx_out): (Sender<i64>, Receiver<i64>) = channel();
    tx_in.send(2).unwrap();
    run(program.clone(), rx_in, tx_out).unwrap();
    let mut part2 = 0;
    for item in rx_out {
        part2 = item
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2)
}
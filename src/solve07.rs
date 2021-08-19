use std::io::{BufRead, BufReader};
use std::fs::File;
use std::cmp::max;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;

use itertools::Itertools;

fn run(mut program: Vec<i32>, input: Receiver<i32>, output: Sender<i32>) -> Result<i32, String> {
    let mut ptr = 0;
    let mut lastout = 0;
    loop {
        let opcode = program[ptr] % 100;
        let width = match opcode {
            99 => 1,
            3 | 4 => 2,
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
    let file = File::open("assets/input07.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    reader.read_line(&mut buf).unwrap();
    let program: Vec<i32> = buf.split(",").map(|s| s.trim().parse().unwrap()).collect();

    let mut best1 = 0;
    for perm in (0..5).permutations(5) {
        let mut signal = 0;
        for phase in perm {
            let (tx_in, rx_in): (Sender<i32>, Receiver<i32>) = channel();
            let (tx_out, rx_out): (Sender<i32>, Receiver<i32>) = channel();
            tx_in.send(phase).unwrap();
            tx_in.send(signal).unwrap();
            run(program.clone(), rx_in, tx_out).unwrap();
            signal = rx_out.recv().unwrap()
        }
        best1 = max(best1, signal)
    }

    let mut best2 = 0;
    for perm in (5..10).permutations(5) {
        let mut conduits: Vec<(Option<Sender<i32>>, Option<Receiver<i32>>)> = Vec::with_capacity(5); // skip realloc
        for amp in 0..5 {
            let (tx, rx) = channel();
            tx.send(perm[amp]).unwrap();
            if amp == 0 {
                tx.send(0).unwrap()
            }
            conduits.push((Some(tx), Some(rx)));
        }

        let mut procs = Vec::with_capacity(5);
        for amp in 0..5 {
            let cprogram = program.clone();
            let inp = conduits[amp].1.take().unwrap();
            let out = conduits[(amp+1)%5].0.take().unwrap();
            procs.push(thread::spawn(move|| run(cprogram, inp, out).unwrap()));
        }
        let mut signal = 0;
        for proc in procs {
            signal = proc.join().unwrap();
        }
        best2 = max(best2, signal)
    }


    println!("Part 1: {}", best1);
    println!("Part 2: {}", best2);
}
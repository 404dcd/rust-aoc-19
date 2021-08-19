use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use std::cmp::{min, max};

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

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32
}
impl std::ops::AddAssign<Point> for Point {
    fn add_assign(&mut self, other: Point) -> () {
        self.x += other.x;
        self.y += other.y
    }
}

fn run_paint(program: Vec<i64>, start_white: bool) -> HashMap<Point, bool> {
    let (tx_in, rx_in): (Sender<i64>, Receiver<i64>) = channel();
    let (tx_out, rx_out): (Sender<i64>, Receiver<i64>) = channel();

    thread::spawn(move|| run(program.clone(), rx_in, tx_out).unwrap());

    let mut painted_white: HashMap<Point, bool> = HashMap::new();
    let mut me = Point{x: 0, y: 0};
    if start_white {
        painted_white.insert(me, true);
    }
    let mut me_dir = 0;
    loop { // get colour, paint, turn, move forward
        tx_in.send(match painted_white.get(&me).unwrap_or(&false) {
            true => 1,
            false => 0}).unwrap();
        
        match rx_out.recv() {
            Ok(x) => painted_white.insert(me, x == 1),
            Err(_) => break
        };

        me_dir += 2 * rx_out.recv().unwrap() - 1;
        me_dir = me_dir.rem_euclid(4);
        me += match me_dir {
            0 => Point{x: 0, y: 1},
            1 => Point{x: 1, y: 0},
            2 => Point{x: 0, y: -1},
            3 => Point{x: -1, y: 0},
            _ => panic!("bad direction")
        }
    }
    painted_white
}

pub fn main() {
    let file = File::open("assets/input11.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    reader.read_line(&mut buf).unwrap();

    let program: Vec<i64> = buf.split(",").map(|s| s.trim().parse().unwrap()).collect();
    
    let r1 = run_paint(program.clone(), false).len();

    let image = run_paint(program, true);
    let (mut starty, mut startx, mut endy, mut endx) = (0, 0, 0, 0);
    for key in image.keys() {
        starty = max(starty, key.y);
        startx = min(startx, key.x);
        endy = min(endy, key.y);
        endx = max(endx, key.x);
    }

    println!("Part 1: {}", r1);
    println!("Part 2:");
    for y in (endy..=starty).rev() {
        for x in startx..=endx {
            if *image.get(&Point{x, y}).unwrap_or(&false) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!()
    }
}
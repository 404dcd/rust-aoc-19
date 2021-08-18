use std::io::{BufRead, BufReader};
use std::fs::File;
use std::cmp::{max, min};

pub fn main() {
    let file = File::open("assets/input03.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut wirestring1 = String::new();
    let mut wirestring2 = String::new();
    reader.read_line(&mut wirestring1).unwrap();
    reader.read_line(&mut wirestring2).unwrap();

    let mut wire1: Vec<(i32, i32, i32, i32)> = Vec::with_capacity(400); // saves the realloc
    let mut x = 0;
    let mut y = 0;
    for mv in wirestring1.split(',') {
        let prevx = x;
        let prevy = y;
        let parseme = mv.chars().next().map(|c| &mv[c.len_utf8()..]).unwrap().trim();
        if parseme.len() == 0 {
            continue
        }
        let amount: i32 = parseme.parse().unwrap();
        match mv.chars().next().unwrap() {
            'R' => x += amount,
            'U' => y += amount,
            'L' => x -= amount,
            'D' => y -= amount,
            _ => panic!("unrecognised direction")
        }
        wire1.push((prevx, prevy, x, y))
    }

    x = 0;
    y = 0;
    let mut best1 = std::i32::MAX;
    let mut best2 = std::i32::MAX;
    let mut w2steps = 0;
    for mv in wirestring2.split(',') {
        let prevx = x;
        let prevy = y;
        let parseme = mv.chars().next().map(|c| &mv[c.len_utf8()..]).unwrap().trim();
        if parseme.len() == 0 {
            continue
        }
        let amount: i32 = parseme.parse().unwrap();
        w2steps += amount;
        match mv.chars().next().unwrap() {
            'R' => x += amount,
            'U' => y += amount,
            'L' => x -= amount,
            'D' => y -= amount,
            _ => panic!("unrecognised direction")
        }
        let mut w1steps = 0;
        for line in &wire1 { // 0 and 1 are wire1, 2 and 3 are wire2
            let (mut x0, mut x1, mut x2, mut x3, mut y0, mut y1, mut y2, mut y3) = (line.0, line.2, prevx, x, line.1, line.3, prevy, y);
            if x0 > x1 || y0 > y1 {
                std::mem::swap(&mut x0, &mut x1);
                std::mem::swap(&mut y0, &mut y1);
            }
            if x2 > x3 || y2 > y3 {
                std::mem::swap(&mut x2, &mut x3);
                std::mem::swap(&mut y2, &mut y3);
            }
            w1steps += (x1 - x0) + (y1 - y0);
            if max(x0, x2) <= min(x1, x3) && max(y0, y2) <= min(y1, y3) {
                let mut marchx = x0;
                let mut marchy = y0;
                while marchx <= x1 && marchy <= y1 {
                    if x2 <= marchx && marchx <= x3 && y2 <= marchy && marchy <= y3 {
                        if marchx != 0 || marchy != 0 {
                            best1 = min(best1, marchx.abs() + marchy.abs());
                            let mut steps = w2steps + w1steps;
                            steps -= (x1 - marchx) + (y1 - marchy);
                            steps -= (x3 - marchx) + (y3 - marchy);
                            best2 = min(best2, steps)
                        }
                    }
                    if x0 == x1 {
                        marchy += 1
                    } else {
                        marchx += 1
                    }
                }
            }
        }
    }

    println!("Part 1: {}", best1);
    println!("Part 2: {}", best2)
}
use std::collections::HashSet;
use std::fs::read_to_string;
use std::cmp::min;
use std::mem::swap;

fn gcd(mut u: u32, mut v: u32) -> u32 {
    // function stolen from https://en.wikipedia.org/wiki/Binary_GCD_algorithm#Iterative_version_in_Rust

    if u == 0 {
        return v;
    } else if v == 0 {
        return u;
    }

    let i = u.trailing_zeros(); u >>= i;
    let j = v.trailing_zeros(); v >>= j;
    let k = min(i, j);

    loop {
        if u > v {
            swap(&mut u, &mut v);
        }

        v -= u;
        if v == 0 {
            return u << k;
        }

        v >>= v.trailing_zeros();
    }
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
impl std::ops::Sub<Point> for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl std::ops::Div<i32> for Point {
    type Output = Point;

    fn div(self, other: i32) -> Point {
        Point {
            x: self.x / other,
            y: self.y / other,
        }
    }
}
impl std::fmt::Display for Point {
    fn fmt(self: &Point, _other: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        Ok(print!("({},{})", self.x, self.y))
    }
}

fn detectable(map: &HashSet<Point>, me: Point) -> Vec<(Point, f32)> {
    let mut ret = Vec::new();
    'outer: for &ast in map.iter() {
        if me == ast {
            continue
        }
        let d = ast - me;
        let pdx = d.x.abs() as u32;
        let pdy = d.y.abs() as u32;
        let stop = gcd(pdx, pdy);
        for div in 2..=stop {
            if pdx % div == 0 && pdy % div == 0 {
                let step = d / div as i32;
                let mut changeme = me.clone();
                changeme += step;
                while changeme != ast {
                    if map.contains(&changeme) {
                        continue 'outer
                    }
                    changeme += step;
                }
            }
        }
        ret.push((ast, -(d.x as f32).atan2(d.y as f32)))
    }
    ret
}

pub fn main() {
    let mapraw = read_to_string("assets/input10.txt").unwrap();
    let mut map: HashSet<Point> = HashSet::new();

    for (y, line) in mapraw.lines().enumerate() {
        for (x, chr) in line.chars().enumerate() {
            if chr == '#' {
                map.insert(Point {x: x as i32, y: y as i32});
            }
        }
    }

    let mut bestvis = 0;
    let mut base = Point{x: 0, y: 0};
    for &me in map.iter() {
        let numdet = detectable(&map, me).len();
        if numdet > bestvis {
            bestvis = numdet;
            base = me;
        }
    }

    let mut count = 1;
    let mut part2 = 0;

    'outer: while map.len() > 0 {
        let mut fov = detectable(&map, base);
        fov.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        for x in fov {
            map.remove(&x.0); // this is the count-th asteroid to die
            if count == 200 {
                part2 = x.0.x * 100 + x.0.y;
                break 'outer
            }
            count += 1
        }
    }

    println!("Part 1: {}", bestvis);
    println!("Part 2: {}", part2)

}
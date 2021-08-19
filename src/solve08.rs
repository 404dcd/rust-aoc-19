use std::fs::read_to_string;

pub fn main() {
    let mut fewest0 = i32::MAX;
    let mut result = 0;
    let image = read_to_string("assets/input08.txt").unwrap();

    let mut imageproc = vec![2; 25*6];

    let mut count0 = 0;
    let mut count1 = 0;
    let mut count2 = 0;
    let mut layerind = 0;
    for chr in image.chars() {
        match chr {
            '0' => {
                count0 += 1;
                if imageproc[layerind] == 2 {
                    imageproc[layerind] = 0
                }
            },
            '1' => {
                count1 += 1;
                if imageproc[layerind] == 2 {
                    imageproc[layerind] = 1
                }
            }
            '2' => count2 += 1,
            _ => {}
        }
        layerind += 1;
        if layerind == 25*6 {
            if count0 < fewest0 {
                result = count1 * count2;
                fewest0 = count0
            }
            count0 = 0;
            count1 = 0;
            count2 = 0;
            layerind = 0;
        }
    }

    println!("Part 1: {}", result);
    println!("Part 2:");
    for line in imageproc.chunks(25) {
        for px in line {
            match px {
                0 => print!("."),
                1 => print!("@"),
                2 => print!("T"),
                _ => {}
            }
        }
        println!()
    }
}
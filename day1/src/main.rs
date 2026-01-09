use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Failed to read input file");

    let lines = input.split("\n");

    let mut position: i32 = 50;
    let mut zero_count = 0;
    let mut i = 0;

    for line in lines {
        if line == "" {
            break;
        }

        let direction = line.chars().nth(0).unwrap();
        let mut count: i32 = line[1..].parse().expect("string parse failed");

        // print!("Position: {}, moving {} by {}", position, direction, count);

        while count >= 100 {
            count -= 100;
        }

        if direction == 'R' {
            position += count;
            if position > 99 {
                position -= 100;
            }
        } else if direction == 'L' {
            position -= count;
            if position < 0 {
                position += 100;
            }
        } else {
            println!("WFT!");
        }

        if position == 0 {
            zero_count += 1;
        }
        i += 1;

        // print!(" new position: {}\n", position);
    }

    println!("Parsed {} lines", i);
    println!("Password is {}", zero_count);
}

use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Failed to read input file");
    // let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82\n";
    // let input = "L50\nR100\n";
    // let input = "R50\nL200\n";

    let lines = input.split("\n");

    let mut position: i32 = 50;
    let mut zero_count = 0;

    for line in lines {
        if line == "" {
            break;
        }

        let direction = &line[0..1];
        let count: i32 = line[1..].parse().expect("string parse failed");

        print!("Position: {}, moving {} by {}", position, direction, count);

        let move_out = move_dial(position, direction, count);
        position = move_out.0;
        zero_count += move_out.1;
        // count landing on 0
        if position == 0 {
            zero_count += 1;
        }

        print!(
            " new position: {}, passed 0 {} times, landed: {}\n",
            position,
            move_out.1,
            position == 0
        );
    }

    println!("Password is {}", zero_count);
}

fn move_dial(start_position: i32, direction: &str, mut count: i32) -> (i32, i32) {
    let mut zero_passes = 0;
    let mut position = start_position;

    // start by doing full turns
    while count >= 100 {
        count -= 100;
        zero_passes += 1;
    }

    // move < 1 full turn, landing on 0 does not count as a pass
    if direction == "R" {
        position += count;
        if position > 99 {
            position -= 100;
            if position != 0 && start_position != 0 {
                zero_passes += 1;
            }
        }
    } else if direction == "L" {
        position -= count;
        if position < 0 {
            position += 100;
            if position != 0 && start_position != 0 {
                zero_passes += 1;
            }
        }
    } else {
        println!("WTF!");
    }

    return (position, zero_passes);
}

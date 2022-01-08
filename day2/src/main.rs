use std::fs;

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
    aim: i32,
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong with file");
    let mut position = Position { x: 0, y: 0, aim: 0 };
    
    for command in contents.lines() {
        let command_split: Vec<&str> = command.split(' ').collect();
        let direction = command_split[0];
        let amount: i32 = command_split[1].parse().unwrap();

        match direction {
            "forward" => {
                position.x += amount;
                position.y += amount * position.aim; // Part 2
            },
            "down" => {
                // position.y += amount; // Part 1
                position.aim += amount; // Part 2
            },
            "up" => {
                // position.y -= amount; // Part 1
                position.aim -= amount; // Part 2
            },
            _ => panic!("Incorrect command!")
        }

    }

    println!("{:#?}", position);
}

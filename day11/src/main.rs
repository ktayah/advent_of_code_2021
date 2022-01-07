use std::{fs};

use day11::Environment;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let mut environment = Environment::new(contents);

    let mut count = 0;
    println!("{}", environment.max_flash);

    while environment.step_flash_count != environment.max_flash {
        environment.step(count);
        count += 1;
    }

    println!("All tiles flashed at {} step.", count);
    println!("Step flash count: {}", environment.step_flash_count);
    println!("Total flash count: {}", environment.total_flash_count);
}

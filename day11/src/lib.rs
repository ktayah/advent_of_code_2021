use std::fmt;

pub struct Environment {
    pub step_flash_count: i32,
    pub total_flash_count: i32,
    pub max_flash: i32, 
    floor: Vec<Vec<u32>>
}

impl Environment {
    pub fn new(floor_string: String) -> Environment {
        let floor: Vec<Vec<u32>> = floor_string.lines()
            .map(|line|
                String::from(line).chars().map(|char| 
                    char.to_digit(10).unwrap()
                ).collect()
            ).collect();


        let env = Environment {
            step_flash_count: 0,
            total_flash_count: 0,
            max_flash: (floor.len() * floor[0].len()).try_into().unwrap(),
            floor
        };
        println!("Before any steps:\n{}", env);

        env
    }

    pub fn step(&mut self, step: i32) {
        let mut highlights: Vec<(usize, usize)> = Vec::new();
        self.step_flash_count = 0;

        for (i, row) in self.floor.iter_mut().enumerate() {
            for (j, num) in row.iter_mut().enumerate() {
                if *num == 9 {
                    highlights.push((i, j));
                    self.step_flash_count += 1;
                    self.total_flash_count += 1;
                    *num = 0;
                } else {
                    *num += 1;
                }
            }
        }

        for (i, j) in &highlights {
            self.increment_adjacent(*i, *j);
        }

        println!("Step {}:\n{}", step + 1, self);
    }

    fn increment_adjacent(&mut self, x: usize, y: usize) {
        // Make sure we don't go out of bounds
        let x_1 = if x == 0 {x} else {x-1};
        let x_2 = if x == self.floor.len() - 1 {x} else {x + 1};
        let y_1 = if y == 0 {y} else {y-1};
        let y_2 = if y == self.floor[0].len() - 1 {y} else {y + 1};

        for i in x_1..=x_2 {
            for j in y_1..=y_2 {
                if self.floor[i][j] == 9 {
                    self.floor[i][j] = 0;
                    self.step_flash_count += 1;
                    self.total_flash_count += 1;
                    self.increment_adjacent(i, j); // New flash, adjacent tiles need to be incremented
                } else if self.floor[i][j] != 0 {
                    self.floor[i][j] += 1;
                }
            }
        }
    }
}

impl fmt::Debug for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Environment")
         .field("floor", &self.floor)
         .finish()
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let d: String = self.floor.iter().map(|line| {
            let mut line_string = String::new();

            for c in line {
                line_string.push(char::from_digit(*c, 10).unwrap());
            }
            line_string.push('\n');

            line_string
        }).collect::<Vec<String>>().concat();

        write!(f, "\n{}", d)
    }
}
use std::fs;

const OPENINGS: &[char] = &['<', '{', '[', '('];

fn calculate_score(contents: &str) -> (i32, Vec<i128>) {
    let mut stack: Vec<char> = Vec::new();
    let mut incorrect_score = 0;
    let mut unbalanced_score: Vec<i128> = Vec::new();

    for line in contents.lines() {   
        let mut incorrect = false;

        for char in line.chars() {
            if OPENINGS.contains(&char) {
                stack.push(char);
            } else {
                let popped = stack.pop().expect("Incorrect input string");

                let expected_char  = match popped {
                    '<' => '>',
                    '{' => '}',
                    '[' => ']',
                    '(' => ')',
                    _ => break
                };
    
                if char != expected_char {
                    // println!("Expected '{}', but found '{}' instead.", expected_char, char);
                    incorrect_score += match char {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => 0,
                    };
                    incorrect = true;
                    break;
                }
            }
        }

        // If are stacked is not emptied, our line is not "balanced"
        if !stack.is_empty() && !incorrect {
            let mut running_score: i128 = 0;
            let stack = stack.iter().rev();

            for val in stack {
                let s = match val {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => 0
                };

                running_score = 5 * running_score + s;
            }

            unbalanced_score.push(running_score);
        }

        stack.clear();
    }

    (incorrect_score, unbalanced_score)
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let (syntax_score, mut unbalanced_scores) = calculate_score(&contents);

    unbalanced_scores.sort_unstable();
    println!("{} {}", unbalanced_scores.len(), unbalanced_scores.len() / 2);
    let middle_unbalanced_score = unbalanced_scores[unbalanced_scores.len() / 2];

    println!("Syntax score: {}", syntax_score);
    println!("Balance score: {:?}", middle_unbalanced_score);
}

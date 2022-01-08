use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong with file");
    
    let bits: Vec<Vec<u32>> = contents.lines()
        .map(|line|
            String::from(line).chars().map(|char| 
                char.to_digit(10).unwrap()
            ).collect()
        ).collect();

    let bits_transposed: Vec<Vec<u32>> = (0..bits[0].len()).map(|i| bits
        .iter()
        .map(|c| c[i])
        .collect()
      ).collect();

    let mut gamma_bits = String::new();
    let mut epsilon_bits = String::new();

    for bit_t_row in bits_transposed {
        let bit_count = bit_t_row.iter().fold((0, 0), |acc, bit| { // first value represents # of 0s, second for 1s
            if *bit == 0_u32 {
                (acc.0 + 1, acc.1)
            } else {
                (acc.0, acc.1 + 1)
            }
        });

        if bit_count.0 > bit_count.1 {
            gamma_bits.push('0'); // Gamma gets more frequent bit
            epsilon_bits.push('1'); // Vice versa
        } else {
            gamma_bits.push('1');
            epsilon_bits.push('0');
        }
    }

    let gamma_rate = isize::from_str_radix(&gamma_bits, 2).unwrap();
    let epsilon_rate = isize::from_str_radix(&epsilon_bits, 2).unwrap();

    println!("Gamma Rate: {}", gamma_rate);
    println!("Epsilon Rate: {}", epsilon_rate);
    println!("Power consumption: {}", gamma_rate * epsilon_rate)
}

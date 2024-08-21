use std::io::{stdin,BufRead};

fn main() {
    
    let input = stdin().lock();
    let lines = input.lines();

    let mut cards_total: u32 = 0;

    for line in lines {
        
        let line_str = line.unwrap().to_string();

        let colon_tokens = line_str
            .split(": ")
            .collect::<Vec<&str>>();
        let pipe_tokens = colon_tokens[1]
            .split(" | ")
            .collect::<Vec<&str>>();
        
        let left_col = pipe_tokens[0]
            .split(" ")
            .map(|x| x.trim())
            .collect::<Vec<&str>>();
        let right_col = pipe_tokens[1]
            .split(" ")
            .map(|x| x.trim())
            .collect::<Vec<&str>>();
        
        let mut multiplier: u32 = 1;
        let mut point_val: u32 = 0; 

        for num in left_col  {
            if num.len() == 0 {
                continue;
            }
            if right_col.contains(&num) {
                point_val = multiplier;
                multiplier *= 2;
                println!("Match {num} {point_val}");
            }
        }
        cards_total += point_val;
    }

    println!("{cards_total}");
}

use std::io::{stdin,BufRead};

fn main() {
    let mut greatest_so_far = 0;
    let mut k = 0; // line index

    let mut in_group = false; // are we currently in a group
    
    let input = stdin().lock();
    let lines = input.lines();

    let mut group_total: u32 = 0;
    
    for line in lines {

        let line_str = line.unwrap().to_string();
        
        if line_str.len() == 0 {
            // we've reached a blank line 
            if in_group {
                // we're ending a group
                in_group = false;
                if group_total > greatest_so_far {
                    greatest_so_far = group_total;
                }
                group_total = 0;
            }
        } else {
            // else we are reading a number
            let num: u32 = line_str
                .parse::<u32>()
                .unwrap();
            group_total += num;
            if !in_group {
                in_group = true;
            }
        }
        println!("Line {k} {greatest_so_far}");
        k += 1;
    }
    println!("{}", greatest_so_far);
}

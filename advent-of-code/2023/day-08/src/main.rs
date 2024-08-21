use std::io::{stdin,BufRead,Result,Error};
use std::collections::HashMap;
use regex::Regex;

fn main() {
    
    let input = stdin().lock();
    let lines = input.lines();

    let mut steps: u32 = 0;

    let parsed_lr : bool = false;

    let mut lines_vec : Vec<String> = Vec::new();

    for line in lines {
        match line {
            Ok(line) => lines_vec.push(line),
            Err(err) => println!("Error {err} while reading line"),
        }
    }
    
    // Parse LR line
    let lr_line = lines_vec.get(0).unwrap();
    let lr_count = lr_line.len();
    println!("{lr_count} steps found in LR directions.");
    
    let mut node_map : HashMap<&str,(&str,&str)> = HashMap::new();

    let re = Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").unwrap();

    // Empty line between LR line and nodes
    assert!(lines_vec.get(1).map_or(false, |line| line.len() == 0),
      "Expected second line to be empty.");

    for line in &lines_vec[2..] {

        for (_, [node, left, right]) in re.captures_iter(line).map(|c| c.extract()) {
            println!("{node} {left} {right}");
            /* try to check for existing value
            let result = node_map.get(node);
            match result {
                Ok(&(&left,&right)) => {
                    println!("Duplicate existing value {left},{right} found for {node} ");
                    continue;
                },
                Err(_) => '',
            }
            */
            node_map.insert(node, (left, right));
        }
    }

    let mut curr : &str = "AAA";

    loop {
        for c in lr_line.chars() {
            let (left, right) = node_map.get(curr).unwrap();
            println!("Found ( {left} , {right} )");
            curr = match c {
                'L' => left,
                'R' => right,
                _ => {
                    println!("Unknown character {c} received");
                    ""
                },
            };
            steps += 1;
            println!("New curr {curr} after {c} and step {steps}");
        }

        if curr == "ZZZ" {
            break;
        }
    } 

    println!("{steps}");
}

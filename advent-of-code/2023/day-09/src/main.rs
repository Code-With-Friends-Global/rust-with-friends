use std::io::{stdin,BufRead};

// Recurse to all zero diffs, then subtract to get a previous value
fn extrapolate_prev_helper(seq: &Vec<i32>) -> i32 {
  let mut all_zeroes: bool = true;

  let mut diffs : Vec<i32> = Vec::new();

  for (i, &num) in seq.iter().enumerate() {
      if num != 0 {
          all_zeroes = false;
      }
      if i > 0 {
          let new_diff = num - seq[i-1];
          println!("Pushing new diff {new_diff}");
          diffs.push(new_diff);
      }
  }
  if all_zeroes {
      println!("Found all zeroes, returning zero");
      0
  } else {
    let diff_str: String = diffs.iter().fold(String::new(), |acc, x| acc + &x.to_string() ).to_string();
    println!("Recursing with {diff_str}");
    let prev = extrapolate_prev_helper(&diffs);
    println!("Extrapolated {prev}");
    seq[0] - prev 
  }
}

fn extrapolate_seq_helper(seq: &Vec<i32>) -> i32 {
  let mut all_zeroes: bool = true;

  let mut diffs : Vec<i32> = Vec::new();

  for (i, &num) in seq.iter().enumerate() {
      if num != 0 {
          all_zeroes = false;
      }
      if i > 0 {
          let new_diff = num - seq[i-1];
          println!("Pushing new diff {new_diff}");
          diffs.push(new_diff);
      }
  }
  if all_zeroes {
      println!("Found all zeroes, returning zero");
      0
  } else {
    let diff_str: String = diffs.iter().fold(String::new(), |acc, x| acc + &x.to_string() ).to_string();
    println!("Recursing with {diff_str}");
    let next = extrapolate_seq_helper(&diffs);
    println!("Extrapolated {next}");
    seq[seq.len()-1] + next
  }
}

fn process_stdin() {

    let input = stdin().lock();
    let lines = input.lines();

    let mut sum : i32 = 0;

    for line in lines {
       
        let line_str = line.unwrap().to_string();

        println!("{line_str}");
        let nums = line_str
            .split(" ")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let count = nums.len();
        println!("{count} numbers found");
        let next = extrapolate_prev_helper(&nums);             
        sum += next;
        println!("Extrapolated {next} sum {sum}");
    }

    println!("{sum}");
}

fn main() {
    /*
    // One-off tests
    let nums: Vec<i32> = vec![3, 6, 9, 12, 15];
    let next = extrapolate_prev_helper(&nums);             
    assert!(next == 0);
    println!("Extrapolated {next}");

    let nums: Vec<i32> = vec![10, 13, 16, 21, 30, 45];
    let next = extrapolate_prev_helper(&nums);             
    assert!(next == 5);
    println!("Extrapolated {next}");
    */
    process_stdin();
}

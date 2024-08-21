use std::io;
use std::io::Error;
use std::option::Option;

fn main() -> Result<(), Error> {

    let words: Vec<&str> = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
    ];
    let mut slices: Vec<&str> = Vec::new();

    for word in words.iter() {
        slices.push(&word[..]);
    }
   
    // Advance the slice and return an updated word, advanced by one character, or reset to the 
    // original word
    pub fn next_slice<'a>(slice: &'a str, c: char, word: &'a str) -> &'a str {
        let curr_char = match slice {
               "" => '\0',
               _ => slice[0..1].chars().next().unwrap(),
        };
        let first_char = word[0..1].chars().next().unwrap();
        // println!(" curr_char {curr_char} =? {c}");
        if curr_char == c {
           let new_slice = match slice {
               "" => &"",
               _ => &slice[1..],
           };
           new_slice
        } else if first_char == c {
           // special case if first letter of a digit word is repeated,
           // we reset to the second letter of the word instead of all the way to the beginning
           &word[1..]
        } else {
           word
        }
    }

    // Takes in a next character
    // Returns true if any word has just matched
    // false otherwise
    pub fn advance<'a>(slices: &mut Vec<&'a str>, c: char, words: &Vec<&'a str>) -> Option<u32> {
        // Collect and advance all digit words first, only then return if any have completed
        let mut result: Option<u32> = None;
        for (i, slice) in slices.iter_mut().enumerate() {
            *slice = next_slice(*slice, c, words[i]);
            if slice.len() == 0 {
               // if we've advanced all the way through a word
               let u32_value: Result<u32,_> = i.try_into(); 
               result = match u32_value {
                   Ok(num) => Some(num),
                   Err(_) => None // this shouldn't happen but if so our caller will continue
                                   // unaware
               }
            }
        }
        result
    }

    let mut total = 0;
    let mut k = 0;
    loop {
        let mut input  = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error while reading input line.");
       
        if input.len() == 0 {
            break;
        }
        //let lines: Vec<&str> = input.split("\n").collect();
        //let total_length = input.len();
        //println!("Length of all input {total_length}");
        //let count = lines.len();
        //println!("Number of lines {count}");
       
        let mut calib_num: u32 = 0;
        let mut first: bool = false;
        let mut last: u32 = 0;

        let mut slices = words.clone();

        for (i, c) in input.trim().chars().enumerate() {
            let completed_digit = advance(&mut slices, c, &words);
            match completed_digit {
                Some(word_digit) => {
                    //println!("At {i} word digit {word_digit}");
                    if !first {
                        calib_num = word_digit;
                        first = true;
                    }
                    last = word_digit;
                },
                None => {
                    // only match for c being a numeric character if we didn't just finish a word
                    // digit
                    if c.is_numeric() {
                        if !first {
                            calib_num = c.to_digit(10).unwrap();
                            first = true;
                        }
                        // the last and first digit can be the same
                        last = c.to_digit(10).unwrap();
                    }
                }
            }
        }
        calib_num = (calib_num * 10) + last;

        //let calib_num : u32 = calib_utils::sum_digits_in_line(&input.trim())
        //    .expect("Parse error on line {k} for match {i}: {digit}");
        println!("{k} {input} {calib_num}");
        total += calib_num;
        k += 1;
    }
    println!("{}", total);
    Ok(())
}

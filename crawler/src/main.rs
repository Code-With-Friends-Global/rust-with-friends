use error_chain::error_chain;
use std::io::Read;
use std::fs::File;
use std::io::{Write,BufReader,BufRead,Error};
use reqwest::blocking::{get, Response};

use std::io;
use reqwest::Result as ReqwestResult;
use std::Result as StdResult;

fn write_file(filename: String, data: String) -> StdResult<(), Error> {
    let mut output = File::create(filename)?;
    
    write!(output, "{}", data.to_string());

    let input = File::open(filename)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line?);
    }

    Ok(())
}

fn main() -> StdResult<(), Error> {
    let mut res: Result<Response,String> = reqwest::blocking::get("https://adventofcode.com/2023/day/1/input");
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    write_file("day-01-input.txt".to_string(), body);

    Ok(())
}

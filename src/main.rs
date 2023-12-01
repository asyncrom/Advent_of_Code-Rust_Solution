mod day1;

use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::ops::Add;
use std::path::Path;
use log::error;

fn main() {
    println!("Advent of code");
    let result = test();
    match result {
        Ok(r) => println!("result:\n {}", r),
        Err(e) => println!("error:\n {}", e)
    }
}

fn test() -> Result<String, Box<dyn Error>> {
    Ok("".to_string())
}

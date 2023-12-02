mod day1;
mod day2;

use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::ops::Add;
use std::path::Path;
use log::error;
use crate::day1::result2;
use crate::day2::day2_b;

fn main() {
    println!("Advent of code");
    let result = day2_b();
    match result {
        Ok(r) => println!("result:\n {}", r),
        Err(e) => println!("error:\n {}", e)
    }
}

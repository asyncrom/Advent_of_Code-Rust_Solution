mod day1;
mod day2;
mod day3;

use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::ops::Add;
use std::path::Path;
use log::error;
use crate::day1::day1_b;
use crate::day2::day2_b;
use crate::day3::{day3_a, day3_b, extract_lines_in_pos};

fn main() {
    println!("Advent of code");
    let path = "/tmp/1.txt";
    let result = day3_b(path);
    match result {
        Ok(r) => println!("result:\n {}", r),
        Err(e) => println!("error:\n {}", e)
    }
}

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn result1() -> Result<String, Box<dyn Error>> {
    let digit_mapping: Vec<(&str, i32)> = vec![
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ];
    let file = File::open("/tmp/1.txt")?;
    let reader = BufReader::new(file);

    let mut total:u32 = 0;

    let mut first:String = "".into();
    let mut temp: String = "".into();

    let mut temp_i: usize = 1000;

    for line in reader.lines() {
        let line = line?;
        println!("{}", line);

        temp_i = 1000;
        for (word, digit) in digit_mapping.iter() {
            if let Some(index) = line.clone().find(word) {
                if index < temp_i {
                    first = digit.to_string();
                    temp_i = index
                }
            }
        }
        temp_i = 0;
        for (word, digit) in digit_mapping.iter().rev() {
            if let Some(index) = line.clone().rfind(word) {
                if index >= temp_i {
                    temp = digit.to_string();
                    temp_i = index
                }
            }
        }

        let number = format!("{}{}",first,temp);
        println!("number: {}", number);
        if number.parse::<u32>().is_ok() {
            total += number.parse::<u32>().unwrap();
        } else {
            println!("nt ok: {}", line.clone())
        }

        first = "".to_string();
        temp = "".to_string();
    }
    Ok(total.to_string())
}

pub fn result2() -> Result<String, Box<dyn Error>> {
    let num = "123456789";
    let digit_mapping: Vec<(&str, i32)> = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ];
    let file = File::open("/tmp/1.txt")?;
    let reader = BufReader::new(file);

    let mut total:u32 = 0;

    let mut first:String = "".into();
    let mut temp: String = "".into();

    let mut temp_i: usize = 1000;

    for line in reader.lines() {
        let line = line?;
        println!("{}", line);

        temp_i = 1000;
        for (word, digit) in digit_mapping.iter() {
            if let Some(index) = line.clone().find(word) {
                if index < temp_i {
                    first = digit.to_string();
                    temp_i = index
                }
            }
        }
        temp_i = 0;
        for (word, digit) in digit_mapping.iter().rev() {
            if let Some(index) = line.clone().rfind(word) {
                if index >= temp_i {
                    temp = digit.to_string();
                    temp_i = index
                }
            }
        }

        let number = format!("{}{}",first,temp);
        println!("number: {}", number);
        if number.parse::<u32>().is_ok() {
            total += number.parse::<u32>().unwrap();
        } else {
            println!("nt ok: {}", line.clone())
        }

        first = "".to_string();
        temp = "".to_string();
    }
    Ok(total.to_string())
}
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use log::error;

struct Game {
    pub id:u32,
    pub sets: Vec<(u32, u32, u32)> // Vec<(red, blue, green)>
}

pub fn day2_a() -> Result<String, Box<dyn Error>> {
    let file = File::open("/tmp/2.txt")?;
    let reader = BufReader::new(file);

    // Define consts
    const MAX_RED: u32 = 12;
    const MAX_BLUE: u32 = 14;
    const MAX_GREEN: u32 = 13;

    // Vec<Game {id, (red, blue, green)}>
    let mut games: Vec<Game> = Vec::new();
    // Parse and fill the games sets from the buffer
    for line in reader.lines() {
        let line = line?;
        // Split the id from the sets
        let (id, cubes_sets) = line.split_once(": ").expect("Failed to split game from cubes sets");
        // Get the game id
        let id: u32 = id.split_ascii_whitespace().nth(1).expect("Can't split Game id").parse().expect("Failed to parse game id");
        // Get the sets: Vec<(red, blue, green)>
        let mut sets = Vec::new();
        for set in cubes_sets.split("; ") {
            let (mut red, mut green, mut blue) = (0u32, 0u32, 0u32);
            for color_cube in set.split(", ") {
                let (number, color) = color_cube.split_once(' ').expect("Failed to split num from color");
                let number: u32 = number.parse().expect("Failed to parse cube number");
                match color {
                    "red"=> red = number,
                    "green" => green = number,
                    "blue" => blue = number,
                    _=> error!("Failed to match color !")
                }
            }
            sets.push((red, blue, green))
        }
        games.push(Game {id, sets});
        println!("Added Game id:{}", id)
    }

    // filter possible games and sum their id
    let mut total_sum = 0u32;

    for game in games {
        let mut valid = true;

        for set in game.sets {
            println!("{}, {}, {}", set.0, set.1, set.2);
            if set.0 > MAX_RED || set.1 > MAX_BLUE || set.2 > MAX_GREEN {
                valid = false;
            }
        }
        if valid {
            total_sum += game.id;
            println!("Game {} valid, sum = {}", game.id, total_sum)
        } else {
            println!("Game {} NOT valid ! \n \n", game.id)
        }
    }



    Ok(total_sum.to_string())
}

pub fn day2_b() -> Result<String, Box<dyn Error>> {
    let file = File::open("/tmp/2.txt")?;
    let reader = BufReader::new(file);

    // Define consts
    const MAX_RED: u32 = 12;
    const MAX_BLUE: u32 = 14;
    const MAX_GREEN: u32 = 13;

    // Vec<Game {id, (red, blue, green)}>
    let mut games: Vec<Game> = Vec::new();
    // Parse and fill the games sets from the buffer
    for line in reader.lines() {
        let line = line?;
        // Split the id from the sets
        let (id, cubes_sets) = line.split_once(": ").expect("Failed to split game from cubes sets");
        // Get the game id
        let id: u32 = id.split_ascii_whitespace().nth(1).expect("Can't split Game id").parse().expect("Failed to parse game id");
        // Get the sets: Vec<(red, blue, green)>
        let mut sets = Vec::new();
        for set in cubes_sets.split("; ") {
            let (mut red, mut green, mut blue) = (0u32, 0u32, 0u32);
            for color_cube in set.split(", ") {
                let (number, color) = color_cube.split_once(' ').expect("Failed to split num from color");
                let number: u32 = number.parse().expect("Failed to parse cube number");
                match color {
                    "red"=> red = number,
                    "green" => green = number,
                    "blue" => blue = number,
                    _=> error!("Failed to match color !")
                }
            }
            sets.push((red, blue, green))
        }
        games.push(Game {id, sets});
        println!("Added Game id:{}", id)
    }

    // filter possible games and sum their max power
    let mut total_sum = 0u32;

    for game in games {
        let total_red = game.sets.iter().map(|s| s.0).max().unwrap();
        let total_blue = game.sets.iter().map(|s| s.1).max().unwrap();
        let total_green = game.sets.iter().map(|s| s.2).max().unwrap();

        let power = total_red * total_blue * total_green;
        total_sum += power
    }



    Ok(total_sum.to_string())
}
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("Hello, world!");

    let file_path = &args[1];
    let mut total = 0;

    let mut max_dice = HashMap::from([
        ("red", 0),
        ("green", 0),
        ("blue", 0),
    ]);


    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(game) = line {
            //dbg!(current_game_nb);
            
            for (key, _value) in max_dice.clone().into_iter() {
                let regex_string = r"(\d*) ".to_owned() + &key.to_owned(); 
                let re_color = Regex::new(&regex_string).unwrap();
                //println!("{} / {}", key, value);
                let mut min = 0;
                if let Some(entry) = max_dice.get_mut(key) {
                    *entry = 0;
                }

                for tmp_color in re_color.captures_iter(&game) {
                    let tmp = tmp_color.get(1).unwrap().as_str();
                    let current_number :i32 = tmp.parse().unwrap();
                    
                    if current_number > min {
                        min = current_number;
                    }    
                }
                //max_dice[key] = max_dice[key] + value;
                if let Some(entry) = max_dice.get_mut(key) {
                    *entry = *entry + min;
                }
            }                
            total = total + max_dice["green"] * max_dice["blue"] * max_dice["red"]; 
            println!("{} {} {}", max_dice["green"], max_dice["blue"], max_dice["red"]);
            }
        }
    }
    println!("{}", total);

}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part1()
{
    let args: Vec<String> = env::args().collect();
    //println!("Hello, world!");

    let file_path = &args[1];
    let re_game_number = Regex::new(r"Game (\d*)").unwrap();

    let mut total = 0;

    let max_dice = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);


    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(game) = line {
            let mut overflow = false;
            let current_game = re_game_number.captures(&game).unwrap().get(1).unwrap().as_str();
            let current_game_nb :i32 = current_game.parse().unwrap();
            //dbg!(current_game_nb);
            
            for (key, value) in max_dice.clone().into_iter() {
                let regex_string = r"(\d*) ".to_owned() + &key.to_owned(); 
                let re_color = Regex::new(&regex_string).unwrap();
                //println!("{} / {}", key, value);
                
                for tmp_color in re_color.captures_iter(&game) {
                    let tmp = tmp_color.get(1).unwrap().as_str();
                    let current_number :i32 = tmp.parse().unwrap();
                    
                    if current_number > value {
                        overflow = true;
                    }
                        
                }
            }
            if !overflow {
                total = total + current_game_nb;
            }
                
            //println!("{}", current_game);


            }
        }
    }
    println!("{}", total);    
}

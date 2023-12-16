use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("Hello, world!");
    let file_path = &args[1];

    let mut previous;
    let mut current = String::new();
    let mut next = String::new();

    let mut total = 0;

    // loop through the lines of the file
    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            previous = current.clone();
            current = next.clone();
            next = line.unwrap();
            if current.len() > 0 {
                total = total + check_adjacent_symbol(&previous, &current, &next);
            }
        }
    }
    total = total + check_adjacent_symbol(&current, &next, &String::new());
    println!("{}", total);
}



/// takes 3 lines in entry, find a * and if there are numbers around 
pub fn check_adjacent_symbol(previous_line:&String, current_line:&String, next_line:&String) -> i32 {
    let re_symbol = Regex::new(r"\*+").unwrap();
    
    let mut res = 0;
    //println!("{}", current_line);


    // loop on each number in the file and getting the position in the line
    for symbol in re_symbol.find_iter(current_line) {
        //dbg!(symbol);
        let mut operandes : Vec<i32> = Vec::new(); // to store the adjacent values

        let first_char:i32 = symbol.start() as i32 - 1 ;
        let last_char:i32 =  symbol.end() as i32;
       // println!("start : {}, stop : {}, string : {}", first_char, last_char, symbol.as_str());
        
        //  to check if there's a number nearby the operator
        operandes.extend(check_number_around(previous_line, first_char, last_char));
        operandes.extend(check_number_around(current_line, first_char, last_char));
        operandes.extend(check_number_around(next_line, first_char, last_char));
        //dbg!(&operandes);
        if operandes.len() == 2 {
            //println!("operande 1 : {}, operande 2 : {}", operandes[0] , operandes[1]);

            res = res + operandes[0] * operandes[1];
        }
    }
    return res;
}

/// check if there's a number in a substring starting at firstchar and ending at lastchar 
/// return the number if exist or -1
fn check_number_around(line:&String, first_char:i32, last_char:i32) -> Vec<i32> {
    let re_number = Regex::new(r"\d+").unwrap();
    let mut res : Vec<i32> = Vec::new();

    if !line.is_empty() {
        for number in re_number.find_iter(line) {
            //println!("intersection start : {}, stop : {}, string : {}", number.start(), number.end(), number.as_str());

            // check intersection
            if (((number.start() as i32) <= first_char) && ((number.end() as i32) -1  >= first_char) && ((number.end() as i32) -1  < last_char)) ||
                (((number.start() as i32) >= first_char) && ((number.start() as i32) <= last_char))
            {
                res.push(number.as_str().parse::<i32>().unwrap());
            }

        }
    }
    return res;
}
// 59557944 is too low :(
// 9219388 is very too low :(
// 42713436 is too low too :(
// 78930055 is too low too :(
// 81463996 

/// The output is wrapped in a Result to allow matching on errors
/// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

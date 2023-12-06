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



/// takes 3 lines in entry and check that there's no adjacent symbol 
pub fn check_adjacent_symbol(previous_line:&String, current_line:&String, next_line:&String) -> i32 {
    let re_number = Regex::new(r"\d+").unwrap();
    
    let mut res = 0;
    println!("{}", current_line);


    // loop on each number in the file and getting the position in the line
    for number in re_number.find_iter(current_line) {
        //dbg!(number);
        let mut adjacent;
        let first_char:usize = if number.start() == 0 { 0 } else { number.start() - 1 };
        let last_char:usize = if number.end() > current_line.len() - 1 { current_line.len() - 1 } else { number.end() + 1 };
        //println!("start : {}, stop : {}, string : {}, len : {}", first_char, last_char, number.as_str(), current_line.len());


        // take the adjacent substring on the previous line to check if there's a symbol
        adjacent = check_symbol(previous_line, first_char, last_char);
        adjacent = check_symbol(current_line, first_char, last_char)|| adjacent;
        adjacent = check_symbol(next_line, first_char, last_char) || adjacent;

        if adjacent {
            //let number_to_add :i32 = number.parse().unwrap();
            //parse::<i32>
            println!("nombre {} est adjacent", number.as_str());            
            res = res + number.as_str().parse::<i32>().unwrap();
        }
    }
    return res;
}

/// check if symbol exist in a substring starting at firstchar and ending at lastchar and return bol 
fn check_symbol(line:&String, first_char:usize, last_char:usize) ->bool {
    let substr;
    let re_symbol = Regex::new(r"[^\.0-9]+").unwrap();

    if !line.is_empty() {
        substr = &line[first_char..last_char];
        if re_symbol.is_match(substr) {
            return true;
        }
    }
    return false;
}


/// The output is wrapped in a Result to allow matching on errors
/// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

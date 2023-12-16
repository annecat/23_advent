use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use std::collections::HashMap;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("Hello, world!");
    if args.len() < 2 { println!("oops you forgot the argument which should be a file"); exit(1);}
    let file_path = &args[1];

    let mut recursive_mem: Vec<usize> = Vec::new();
    let mut previous_won_one_card = 1;
    let mut total = 0;

    // loop through the lines of the file
    let lines = read_lines(file_path).expect("can't read entry file");
    // Consumes the iterator, returns an (Optional) String
    for line in lines {
        let line_str = line.expect("error in line reading");
        
        if line_str.len() <= 0 {continue;}

        let winning_numbers = &line_str[line_str.find(':').unwrap()..line_str.find('|').unwrap()];
        let mut winning_hash = HashMap::new();
        let mut total_won_one_card = 0;

        let candidate_numbers = &line_str[line_str.find('|').unwrap()..line_str.len()];


        println!("winning numbers : {} et les candidates : {}", winning_numbers, candidate_numbers);
        
        let re_number = Regex::new(r"\d+").expect("regexp creation issue");
        
        //insertion of the winners number into a hash
        re_number.find_iter(&winning_numbers).for_each(|num| {winning_hash.insert(num.as_str(), 1); });
        
        //looping the candidate numbers to check is they are in the hash and then do the math
        re_number.find_iter(&candidate_numbers).for_each(|num| {total_won_one_card += check_and_add(&winning_hash, &mut recursive_mem, total_won_one_card, previous_won_one_card, num.as_str())});
        println!("Previous value : {}, Total value : {}  et Vector: {:?}", previous_won_one_card, total_won_one_card, recursive_mem);

        if recursive_mem.len() > 0 { 
            previous_won_one_card = recursive_mem[0];
            recursive_mem.remove(0);
        }
        else {
            previous_won_one_card = 1
        }
        total = total + previous_won_one_card;
        
        winning_hash.clear();
    }
 
    //total = total + check_adjacent_symbol(&current, &next, &String::new());
    println!("{}", total);
}

/// check if the map contains the key and cal the result
fn check_and_add(hash:&HashMap<&str, usize>, recursive_mem: &mut Vec<usize>, current_res:usize, previous_res:usize, num:&str) -> usize {

    if hash.contains_key(num) {
        if recursive_mem.len() <= current_res {
            recursive_mem.push(previous_res + 1);
        }
        else {
            recursive_mem[current_res] = recursive_mem[current_res] + previous_res;
        }
        return 1; 

    }
    return 0;
}

/// The output is wrapped in a Result to allow matching on errors
/// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/*fn _part1() {
    let args: Vec<String> = env::args().collect();
    //println!("Hello, world!");
    if args.len() < 2 { println!("oops you forgot the argument which should be a file"); exit(1);}
    let file_path = &args[1];

    let mut total = 0;

    // loop through the lines of the file
    let lines = read_lines(file_path).expect("can't read entry file");
        // Consumes the iterator, returns an (Optional) String
    for line in lines {
        let line_str = line.expect("error in line reading");
        if line_str.len() > 0 {

            let winning_numbers = &line_str[line_str.find(':').unwrap()..line_str.find('|').unwrap()];
            let mut winning_hash = HashMap::new();
            let mut subtotal = 0;
            let candidate_numbers = &line_str[line_str.find('|').unwrap()..line_str.len()];
            println!("winning numbers : {} et les candidates : {}", winning_numbers, candidate_numbers);
            
            let re_number = Regex::new(r"\d+").expect("regexp creation issue");
            
            //insertion of the winners number into a hash
            re_number.find_iter(&winning_numbers).for_each(|num| {winning_hash.insert(num.as_str(), 1); });
            
            //looping the candidate numbers to check is they are in the hash and then do the math
            re_number.find_iter(&candidate_numbers).for_each(|num| {subtotal = check_and_add(&winning_hash, subtotal, num.as_str())});
            total = total + subtotal;
            winning_hash.clear();
        }
    }
    //22608 too high
    //total = total + check_adjacent_symbol(&current, &next, &String::new());
    println!("{}", total);
}*/
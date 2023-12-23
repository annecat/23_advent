use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
//use std::collections::HashMap;
use std::process::exit;




fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("Hello, world!");
    
    if args.len() < 2 { println!("oops you forgot the argument which should be a file"); exit(1);}
    let file_path = &args[1];
    let mut total = 0; 

    let re_line = Regex::new(r"(\w) (\d*) (\(.*\))").expect("regexp creation issue");



    // parsing of the file and working at the same time
    // loop through the lines of the file
    let lines = read_lines(file_path).expect("can't read entry file");
    let mut current = (0,0);
    let mut next = (0,0);

    // let's try the shoelace algo
    let mut sum1 : i32 = 0;
    let mut sum2 : i32 = 0;

    for line in lines {
        let line_str = line.expect("error in line reading");
        if line_str.len() == 0 {
            continue;
        }
        let capture = re_line.captures(&line_str).unwrap();
        let direction : char = capture.get(1).unwrap().as_str().chars().nth(0).unwrap();
        let number : i32 = capture.get(2).unwrap().as_str().parse().unwrap();

        match direction {
            'R' => next = (current.0, current.1 + number),
            'D' => next = (current.0 + number, current.1),
            'L' => next = (current.0, current.1 - number),
            'U' => next = (current.0 - number, current.1),
            _ => {}
        };
        //println!("current {} {}, next {} {}", current.0, current.1, next.0, next.1);
        
        total += number;

        sum1 = sum1 + current.0 as i32 * next.1 as i32;
        sum2 = sum2 + current.1 as i32 * next.0 as i32;
        current = next;
    }
    
    let area = (sum1 - sum2).abs() / 2;

    println!("Area : {}", area);
    println!("perimetre : {}", total);
   // println!("answer part1 : {}", total / 2);
     println!("Total Area : {}", area + (total / 2) + 1);
   //println!("answer part2 : {}", area - (total / 2 - 1));
    
}



/// The output is wrapped in a Result to allow matching on errors
/// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

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

    let re_line = Regex::new(r"(\w) (\d*) \(#(.*)(\d)\)").expect("regexp creation issue");
    let mut coord:Vec<(i32, i32)> = Vec::new();


    // parsing of the file and working at the same time
    // loop through the lines of the file
    let lines = read_lines(file_path).expect("can't read entry file");
    let mut current : (i32, i32) = (0,0);
    let mut next : (i32, i32)= (0,0);

    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;

    // Get the data and also the minx and the miny so we don't overflow later    
    for line in lines {
        let line_str = line.expect("error in line reading");
        if line_str.len() == 0 {
            continue;
        }
        let capture = re_line.captures(&line_str).unwrap();
        let direction : char = capture.get(4).unwrap().as_str().chars().nth(0).unwrap();
        
        let number_hexa = capture.get(3).unwrap().as_str();
        let number = i32::from_str_radix(number_hexa, 16).unwrap();

        //println!("{:?}", capture);
        match direction {
            '0' => next = (current.0, current.1 + number),
            '1' => next = (current.0 + number, current.1),
            '2' => next = (current.0, current.1 - number),
            '3' => next = (current.0 - number, current.1),
            _ => {}
        };
       // println!("current {} {}, next {} {}", current.0, current.1, next.0, next.1);
       
        if next.0 < min_y {
            min_y = next.0
        }
        if next.1 < min_x {
            min_x = next.1
        }
        total += number;

        coord.push(current);
        current = next;
    }
    
    //println!("Shoooooooooooooooolace {:?}", coord);
    // let's try the shoelace algo
    let mut sum1 : u128 = 0;
    let mut sum2 : u128 = 0;

    for next in coord.iter() {
        if *next == current {
            continue;
        }
        // we translate the polygon to not overflow the multiplication with minx and miny
        sum1 = sum1 + (current.0 - min_y) as u128 * (next.1 - min_x) as u128;
        sum2 = sum2 + (current.1 - min_x) as u128 * (next.0 - min_y) as u128;
        //println!("current {} {}, next {} {}", current.0, current.1, next.0, next.1);
        //println!("sum 1 {} et sum 2 {}", sum1, sum2);
        current = *next;
    }
    next = (0, 0);
    sum1 = sum1 + (current.0 - min_y) as u128 * (next.1 - min_x) as u128;
    sum2 = sum2 + (current.1 - min_x) as u128 * (next.0 - min_y) as u128;

    let area = if sum1 > sum2 {
        (sum1 - sum2) / 2
    } else {
        (sum2 - sum1) / 2
    };

    println!("Area : {}", area);
    println!("perimetre : {}", total);
     println!("Total Area : {}", area + (total as u128 / 2) + 1);
    
}



/// The output is wrapped in a Result to allow matching on errors
/// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

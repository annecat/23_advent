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
    let re_number = Regex::new(r"(-?\d+)").expect("regexp creation issue");
    let mut total = 0;


    // parsing of the file and working at the same time
    // loop through the lines of the file
    let lines = read_lines(file_path).expect("can't read entry file");
    for line in lines {
        let line_str = line.expect("error in line reading");

        //println!("{}", line_str);
        if line_str.len() == 0 {continue;}

        let capture: Vec<i32> = re_number.find_iter(&line_str).map(|x| x.as_str().parse::<i32>().unwrap()).collect();
       // println!("{:?}", capture);
        //let &temp: Vec<i32> = Vec::new();
        total = total + calc_prev(&capture);
      //  println!("total intermediaire : {}", total);
    }
    println!("total : {}", total);
}

// part 1
fn _calc_next(capture: &Vec<i32>, i:usize ) ->i32 {
   //if capture: &Vec<i32>
    
    if capture[i] - capture[i -1] == 0 {
        return capture[i];
    }
   // let temp: Vec<i32> = capture.iter().map(|&x| x + 1).collect::<Vec<_>>();
    let temp: Vec<_> = capture.windows(2).map(|pair| pair[1] - pair[0]).collect();
    //println!("{:?}", temp);
    //println!(" {} + {} + reste", capture[i], temp[i - 1]);
    return capture[i]  + _calc_next(&temp, i - 1);
}

// part 2
fn calc_prev(capture: &Vec<i32>) ->i32 {
    //if capture: &Vec<i32>
     
     if capture.iter().all(|&x| x == 0) {
         return capture[0];
     }
     let temp: Vec<_> = capture.windows(2).map(|pair| pair[1] - pair[0]).collect();
    // println!("{:?}", temp);
     //println!(" {} + {} + reste", capture[0], temp[0]);
     return capture[0] - calc_prev(&temp);
 }

/// The output is wrapped in a Result to allow matching on errors
/// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


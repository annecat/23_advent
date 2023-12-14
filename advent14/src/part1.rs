use std::env;
use std::fs::File;
use std::io::{self, BufRead};

//use std::path::Path;
//use std::collections::HashMap;
//use std::collections::HashSet;
use std::process::exit;
//use std::cmp::max;

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("Hello, world!");
    
    if args.len() < 2 { println!("oops you forgot the argument which should be a file"); exit(1);}
    let file_path = &args[1];

    // parsing of the file and working at the same time
    // loop through the lines of the file
    let file = File::open(file_path).unwrap();
    let buffered = io::BufReader::new(file);
    let line_count = buffered.lines().count() - 1;
    println!("line_count {}", line_count);
    let input = File::open(file_path).unwrap();
    let buffered = io::BufReader::new(input);

    let lines = buffered.lines(); // read_lines(file_path).expect("can't read entry file");

    let mut obstacle_array : Vec<usize> = Vec::new();
    let mut total = 0;
    let mut i = 1;

    for line in lines {
        let line_str = line.expect("error in line reading");
        if line_str.len() == 0 { 
            break; 
        }
        // init the array at the beginning
        if obstacle_array.len() == 0 {
            for _ in 0..line_str.len() {
                obstacle_array.push(0);
            }
        }
        let mut j = 0;
        for c in line_str.chars() {
            match c {
                '#' => {obstacle_array[j] = i;},
                'O' => {
                        total += line_count - obstacle_array[j]; 
                        println!("add : {}, total {}",line_count - obstacle_array[j], total);
                        obstacle_array[j] = obstacle_array[j] + 1;
                    },
                _ => {}
            }
            j += 1;
        }
        i += 1;
    }

    println!("{}", total);

}



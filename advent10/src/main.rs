use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//use regex::Regex;
use std::collections::HashMap;
use std::process::exit;




fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("Hello, world!");
    
    if args.len() < 2 { println!("oops you forgot the argument which should be a file"); exit(1);}
    let file_path = &args[1];
    let mut total;

    let mut y = 0;
    let mut departure = (0, 0);
    let mut pipe_map : HashMap<(usize, usize), char> = HashMap::new();



    // parsing of the file and working at the same time
    // loop through the lines of the file
    let lines = read_lines(file_path).expect("can't read entry file");
    for line in lines {
        let line_str = line.expect("error in line reading");

        for (x, c) in line_str.chars().enumerate() {
            if c == 'S' {departure = (x, y);}
            pipe_map.insert((x, y),c);
        }
        
        y += 1;
    }

    let mut current = (0, 0);

    // init the first direction
    
    if (pipe_map[&(departure.0 - 1, departure.1)] == '-') || 
    (pipe_map[&(departure.0 - 1 , departure.1)] == 'L') ||
    (pipe_map[&(departure.0 - 1 , departure.1)] == 'F') {
     current = (departure.0 - 1 , departure.1);
    }
    if (pipe_map[&(departure.0, departure.1 + 1)] == '|') || 
    (pipe_map[&(departure.0 , departure.1 + 1)] == 'L') ||
    (pipe_map[&(departure.0 , departure.1 + 1)] == 'J') {
     current = (departure.0, departure.1 + 1);
    }
    if (pipe_map[&(departure.0 + 1 , departure.1)] == '-') || 
       (pipe_map[&(departure.0 + 1 , departure.1)] == 'J') ||
       (pipe_map[&(departure.0 + 1 , departure.1)] == '7') {
        current = (departure.0 + 1 , departure.1);
    }

    let mut prev = departure;

    let mut next;

    
    // let's try the shoelace algo
    let mut sum1 : i32 = prev.0 as i32 * current.1 as i32;
    let mut sum2 : i32 = prev.1 as i32 * current.0 as i32;

    total = 1;
    // loop through the pipe
    while pipe_map[&current] != 'S' {
        
         next = find_next(&pipe_map, prev, current);
         total += 1;
         // Green theoreme : x dy − y dx,
         //area += current.1 as i32 * (next.0 as i32 - current.0 as i32);
         sum1 = sum1 + current.0 as i32 * next.1 as i32;
         sum2 = sum2 + current.1 as i32 * next.0 as i32;
        // println!("current char : {}, current {:?} et next {:?} et area : {}", pipe_map[&current], current, next, area);
         prev = current;
         current = next;
    }
    // still shoelace algo
    let area = (sum1 - sum2).abs() / 2;

    println!("Area : {}", area);
    println!("perimetre : {}", total);
    println!("answer part1 : {}", total / 2);
//  println!("Total Area : {}", area + (total / 2) + 1);
    println!("answer part2 : {}", area - (total / 2 - 1));
    // 628 too high
    // 535 is wrong    
    // 501 is the solution     

}

fn find_next(pipe_map : &HashMap<(usize, usize), char>, prev : (usize, usize),  current : (usize, usize)) -> (usize, usize) {
    
    
    match pipe_map[&current] {
        '-' => {
            if prev.0 == current.0 - 1 {return (current.0 + 1, current.1)} else {return (current.0 -1, current.1)}
        },
        '|' => {
            if prev.1 == current.1 - 1 {return (current.0, current.1 + 1)} else {return (current.0, current.1 -1)}
        },
        'F' => {
            if prev.1 == current.1 + 1 {return (current.0 + 1, current.1)} else {return (current.0, current.1 + 1)}
        },
        'J' => {
            if prev.1 == current.1 - 1 {return (current.0 - 1, current.1)} else {return (current.0, current.1 - 1)}
        },
        'L' => {
            if prev.1 == current.1 - 1 {return (current.0 + 1, current.1)} else {return (current.0, current.1 - 1)}
        },
        '7' => {
            if prev.1 == current.1 + 1 {return (current.0 - 1, current.1)} else {return (current.0, current.1 + 1)}
        },
        _ => return (0,0)
    }
}


/// The output is wrapped in a Result to allow matching on errors
/// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

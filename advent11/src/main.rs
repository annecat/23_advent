use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//use regex::Regex;
//use std::collections::HashMap;
use std::collections::HashSet;
use std::process::exit;


fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("Hello, world!");
    
    if args.len() < 2 { println!("oops you forgot the argument which should be a file"); exit(1);}
    let file_path = &args[1];
    let mut total = 0;

    let mut x = 0;
    let mut max_x = 0;
    let mut y = 0;

    let mut constellation_map : Vec<(usize, usize)> = Vec::new();

    let mut empty_row : HashSet<usize> = HashSet::new();


    // parsing of the file and working at the same time
    // loop through the lines of the file
    let lines = read_lines(file_path).expect("can't read entry file");
    for line in lines {
        let line_str = line.expect("error in line reading");
        if line_str.len() == 0 { continue; }
        let mut empty_line = true;
            for c in line_str.chars() {
                if c == '#' {
                    constellation_map.push((y, x));
                    empty_line = false;
                    empty_row.insert(x);
                    if max_x < x {max_x = x;}                    
                }
                x += 1;
            }
        if empty_line { y += 1000000; } else  { y += 1; }
        x = 0;
    }
    println!("constellation before {:?}", constellation_map);
    //println!("hashset {:?}", empty_row);

    // add an extra colum 
    let mut inc = 0;
    for i in 0..max_x
    {
        if !empty_row.contains(&i) {
            println!("i : {}",  i);
            for key in &mut constellation_map {
               
                if key.1 as i32 - inc as i32 > i as i32 {
                    key.1 += 999999;
                }
            }
            inc += 999999;
        }
    }

    println!("constellation before {:?}", constellation_map);
    
    for i in 0..constellation_map.len() {
        let key1 = constellation_map[i];
        for j in (i + 1)..constellation_map.len() {
            let key2 = constellation_map[j];
            total += (key1.0 as i32 - key2.0 as i32).abs() as usize + (key1.1 as i32 - key2.1 as i32).abs() as usize;
        }
    }
 
    println!("total {}", total);
    // 82000292 too low
    // 752936886232 is too high
    // 323038563202 is too low
    // 752936133304
    // 82 d'écart sur l'exemple avec *10 et *1000 soit un écart fixe
}

/// The output is wrapped in a Result to allow matching on errors
/// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use std::collections::HashMap;
use std::process::exit;
use num_integer::lcm;


fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("Hello, world!");
    
    if args.len() < 2 { println!("oops you forgot the argument which should be a file"); exit(1);}
    let file_path = &args[1];
    // loop through the lines of the file
    let lines = read_lines(file_path).expect("can't read entry file");
    // Consumes the iterator, returns an (Optional) String

    let re_letter = Regex::new(r"([0-9A-Z]+) = \(([0-9A-Z]+), ([0-9A-Z]+)\)").expect("regexp creation issue");
    
    let mut instruction : String = String::new();

    let mut destination_map : HashMap<String, (String,String)> = HashMap::new();
    let mut total = 0;
    let mut current = Vec::new();
    let mut res = Vec::new();

    // parsing of the file and working at the same time
    for line in lines {
        let line_str = line.expect("error in line reading");
        if line_str.len() == 0 { continue; }
        //println!("{}", line_str);

        //if the source is empty then it is the first line we need the seeds : get the seed
        if instruction.len() == 0 {
            instruction = line_str.clone();
        }
        
        match re_letter.captures(&line_str) {
            Some(capture) => {
                // Do something with the capture
                destination_map.insert(capture.get(1).unwrap().as_str().to_string(), (capture.get(2).unwrap().as_str().to_string(), capture.get(3).unwrap().as_str().to_string()));
                if capture.get(1).unwrap().as_str().chars().nth(2) == Some('A') {
                    current.push(capture.get(1).unwrap().as_str().to_string());
                }

                // println!("Found letters: {:?}", capture);
            }
            None => {
                // Handle the case where there is no match
                //println!("No letters found");
            }
        }
    }
    //println!("instruction: {}", instruction);
    //println!("destination_map: {:?}", destination_map);
    println!("current: {:?}", current);

    

    for cur in &current {

        let mut tmp = cur.as_str().to_string();
        let mut i = 0; // iterator on instruction
        while tmp.as_str().chars().nth(2) != Some('Z')
        {
        
           // println!("tmp: {} and i :{} and total : {} ", tmp, i, total);
            
            match instruction.chars().nth(i) {
                Some('R') => { tmp = destination_map[&tmp].1.as_str().to_string(); total += 1;}
                Some('L') => { tmp = destination_map[&tmp].0.as_str().to_string(); total += 1; }
                _ => { }//println!("instruction error") ;} 
            }
            if i == instruction.len() {i = 0;} else {i += 1;}  
        }
        println!("cur {}, tmp {}, total {}", cur, tmp, total);
        res.push(total);
        total = 0;
    }
    //let max_value = usize::MAX;
    //println!("Maximum value for usize: {}", max_value);

    for i in res {
        println!(" total {}, i {}", total, i);
        if total == 0 { total = i; } else { total = lcm(i as usize, total as usize);}
    }
    //calc the least comon multiple  
    println!("{}", total);

    //    println!("current: {:?}", current);
  //  println!("{}", total);
}
// MAX usize : 18446744073709551615
// sol :             11283670395017

fn _is_it_the_end(current:&Vec<String>) -> bool{
    for i in current {
        if i.as_str().chars().nth(2) != Some('Z') {
            return false;
        }
    }
    return true;
}


/// The output is wrapped in a Result to allow matching on errors
/// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


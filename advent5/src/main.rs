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
    // loop through the lines of the file
    let lines = read_lines(file_path).expect("can't read entry file");
    // Consumes the iterator, returns an (Optional) String

    let re_number = Regex::new(r"(\d+)").expect("regexp creation issue");


    let mut source_list :Vec<(usize, usize)> = Vec::new();
    
    let mut destination_map : Vec<(usize, usize, usize)> = Vec::new();


    // parsing of the file and working at the same time
    for line in lines {
        let line_str = line.expect("error in line reading");

        //println!("{}", line_str);

        //line is empty, it's the end of something
        if line_str.len() <= 0 {
            if destination_map.len() == 0 { continue;}

            //sort the destination map to facilitate 
            destination_map.sort_by(|a, b| a.1.cmp(&b.1));


            println!("Destination map {:?}", destination_map);
            source_list = translate_source_destination(&mut source_list, &destination_map);
            source_list.sort_by(|a, b| a.0.cmp(&b.0));
            println!("Source after translation {:?}", source_list);
            destination_map.clear();
            continue; 
        }


        let capture: Vec<usize> = re_number.find_iter(&line_str).map(|x| x.as_str().parse::<usize>().unwrap()).collect();

        //if the source is empty then it is the first line we need the seeds : get the seed
        if source_list.len() == 0 {
            let mut i = 0;
            while i < capture.len() {
                source_list.push((capture[i], capture[i + 1]));
                i += 2;
            }
            source_list.sort_by(|a, b| a.0.cmp(&b.0));
            println!("{:?}", source_list);
            continue;
        }
        
        //if it is not the seed, then it is a map for source/destination so we parse it and put in in destination map
        //dbg!(capture);
        if capture.len() > 0 {
            destination_map.push((capture[0],capture[1],capture[2]))
        }
        else {
            println!("{}", line_str);
        }
        //(|num| {source_list.push(num.as_str().parse::<usize>().unwrap());});
        // too low : 86473941
        //now        251346198,
    }
    println!("{}", source_list[0].0);
}



/// populate the destination
/// both our entry are sorted by source to lower the complexity. Let's do the magic
fn translate_source_destination(source_list: &mut Vec<(usize, usize)>, destination_map : &Vec<(usize, usize, usize)>) ->  Vec<(usize, usize)> {
    let mut destination_list = Vec::new();
    let mut current_index_destination = 0;
    
    

    while source_list.len() > 0 {
        
        let source =  source_list[0];
        let source_int = source.0;
        let range_int = source.1;
        source_list.remove(0);
        println!("range_int : {}", range_int);
        
        // as the lists are sorted if my source first number is lower than my destination first number 
        while current_index_destination < destination_map.len() && source_int + range_int < destination_map[current_index_destination].1 {
            current_index_destination += 1;
        }
        
        while current_index_destination < destination_map.len() && source_int >= destination_map[current_index_destination].1 + destination_map[current_index_destination].2
        {
//          destination_list.push((source_int, range_int));
            current_index_destination += 1;
        }


        
        if current_index_destination < destination_map.len() {
            
            println!("source_int : {}:{}, current_index_destination : {}, range : {}:{} ", source_int, source_int + range_int, current_index_destination, destination_map[current_index_destination].1, destination_map[current_index_destination].1 + destination_map[current_index_destination].2);
       }
       else {
           println!("source_int : {}:{}, current_index_destination : {}", source_int, source_int + range_int, current_index_destination);
       }
        

        // if I arrived at the end of my map then not in a range I just push the source
        if current_index_destination == destination_map.len() {
            destination_list.push((source_int, range_int));
            continue;
        }


        if source_int < destination_map[current_index_destination].1 && source_int + range_int < destination_map[current_index_destination].1 {
            destination_list.push((source_int, range_int));
            continue;
        }

        if source_int < destination_map[current_index_destination].1 {
            destination_list.push((source_int, destination_map[current_index_destination].1 - source_int));
            if range_int - (destination_map[current_index_destination].1 - source_int) > 0 {
                source_list.insert(0, (destination_map[current_index_destination].1, range_int - (destination_map[current_index_destination].1 - source_int)));
            }
            continue;
        }


        // si la source est plus grande que la destination + range alors il n'y a pas d'appartenance on passe au suivant

        // Cela veut donc dire que maintenant nous sommes dans le cas d'une intersection entre la source et la destination.

        let overlap;
        // calcul de l'overlap
        if source_int + range_int < destination_map[current_index_destination].1 + destination_map[current_index_destination].2 {
            overlap = range_int;
        } else {
            overlap = destination_map[current_index_destination].1 + destination_map[current_index_destination].2 - source_int;
            //if range_int - overlap > 0 { source_list.insert(0, (source_int + overlap, range_int - overlap)); }
        }

        if range_int - overlap > 0 { source_list.insert(0, (source_int + overlap, range_int - overlap)); }

        //println!("overlap : {}, source_int : {}, destination_int : {}", overlap, source_int, destination_map[current_index_destination].1);
       
        destination_list.push((destination_map[current_index_destination].0 + (source_int - destination_map[current_index_destination].1), overlap));
        //destination_list.push(destination_map[current_index_destination].0 + (source_int - destination_map[current_index_destination].1));        
    }

    return destination_list;
}

// 1514000396 is too high
// 206073363 is too high
// 22394533 is wrong
// 72263011 is just

/// The output is wrapped in a Result to allow matching on errors
/// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}



fn _part1_main() {
    let args: Vec<String> = env::args().collect();
    //println!("Hello, world!");
    
    if args.len() < 2 { println!("oops you forgot the argument which should be a file"); exit(1);}
    let file_path = &args[1];
    // loop through the lines of the file
    let lines = read_lines(file_path).expect("can't read entry file");
    // Consumes the iterator, returns an (Optional) String

    let re_number = Regex::new(r"(\d+)").expect("regexp creation issue");


    let mut source_list = Vec::new();
    
    let mut destination_map : Vec<(usize, usize, usize)> = Vec::new();


    // parsing of the file and working at the same time
    for line in lines {
        let line_str = line.expect("error in line reading");

        //println!("{}", line_str);

        //line is empty, it's the end of something
        if line_str.len() <= 0 {
            if destination_map.len() == 0 { continue;}

            //sort the destination map to facilitate 
            destination_map.sort_by(|a, b| a.1.cmp(&b.1));


            //println!("Destination map {:?}", destination_map);
            source_list = _part1_translate_source_destination(&source_list, &destination_map);
            source_list.sort();
            println!("Source {:?}", source_list);
            destination_map.clear();
            continue; 
        }

        //if the source is empty then it is the first line we need the seeds : get the seed
        if source_list.len() == 0 {
            re_number.find_iter(&line_str).for_each(|num| {source_list.push(num.as_str().parse::<usize>().unwrap());});
            source_list.sort();
            println!("{:?}", source_list);
            continue;
        }
        
        //if it is not the seed, then it is a map for source/destination so we parse it and put in in destination map
        let capture: Vec<usize> = re_number.find_iter(&line_str).map(|x| x.as_str().parse::<usize>().unwrap()).collect();
        //dbg!(capture);
        if capture.len() > 0 {
            destination_map.push((capture[0],capture[1],capture[2]))
        }
        else {
            println!("{}", line_str);
        }
        //(|num| {source_list.push(num.as_str().parse::<usize>().unwrap());});
        // too low : 86473941
        //now        251346198,
    }
}



/// populate the destination
/// both our entry are sorted by source to lower the complexity. Let's do the magic
fn _part1_translate_source_destination(source_list: &Vec<usize>, destination_map : &Vec<(usize, usize, usize)>) ->  Vec<usize> {
    let mut destination_list = Vec::new();
    let mut current_index_destination = 0;

    for source in source_list {
        
        let source_int = *source;

        
        // as the lists are sorted if my source noumber is bigger than my range then I have to get to the next range
        while current_index_destination < destination_map.len() && source_int >= destination_map[current_index_destination].1 + destination_map[current_index_destination].2 {
            current_index_destination += 1;
        }
        
/*
        if current_index_destination < destination_map.len() {
            println!("source_int : {}, current_index_destination : {}, range : {}:{} ", source_int, current_index_destination, destination_map[current_index_destination].1, destination_map[current_index_destination].1 + destination_map[current_index_destination].2);
       }
       else {
           println!("source_int : {}, current_index_destination : {}", source_int, current_index_destination);
       }
*/

        // if I arrived at the end of my map then not in a range I just push the source
        if current_index_destination == destination_map.len() {
            destination_list.push(source_int);
            continue;
        }

        // if the source is under the map current then it's not in my map, I push the source and go to the next value.
        if source_int < destination_map[current_index_destination].1 {
            destination_list.push(source_int);
        } 
        else { // the source is in the map and yala we calculate and put the new value
        
            destination_list.push(destination_map[current_index_destination].0 + (source_int - destination_map[current_index_destination].1));
        }
  

       

    }

    return destination_list;
}

use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

//use std::path::Path;
use std::collections::HashMap;
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
    let input = File::open(file_path).unwrap();
    let buffered = io::BufReader::new(input);
    let lines = buffered.lines(); // read_lines(file_path).expect("can't read entry file");

    let mut data_hash : HashMap<u64, usize> = HashMap::new();
    let mut data_map : Vec<Vec<char>> = Vec::new();


    for line in lines {
        let line_str = line.expect("error in line reading");
        if line_str.len() == 0 { 
            break; 
        }
        data_map.push(line_str.chars().collect());
    }

    //let mut hasher = hash_vec(&data_map);
    //data_hash.insert(hasher, 0);

    let mut direction = (0, -1);
    let mut occurence = 0;

    //data_map.iter().for_each(|element| println!("Element: {:?}", element)); 
    
   

//    for i in 1..1000000000 {
    for i in 0..4000000000 {
        //tilt in a direction
        tilt(&mut data_map, direction);
        
        if i == 0 
        {
            println!("start answer to part1: {}", calc_res(&data_map));
            println!("____________________________________________________");
        }


        // N W S E : I don't like this imbrication of if, but I am lazy today ^^
        // Could put the value in a tab and loop in the tab :)
        if (direction.0 == 0) && (direction.1 == -1) { direction = (-1, 0);} 
            else { if (direction.0 == -1) && (direction.1 == 0) { direction = (0, 1);} 
                else { if (direction.0 == 0) && (direction.1 == 1) { direction = (1, 0);} 
                    else { if (direction.0 == 1) && (direction.1 == 0) { 
                        // this is the end of a cycle

                        direction = (0, -1); 
                        //data_map.iter().for_each(|element| println!("Element: {:?}", element)); 
                        //println!("intermediary : {} et i : {}", calc_res(&data_map), (i + 1) / 4);
                        //println!("____________________________________________________");

                        //check if we are in a loop
                        let hasher = hash_vec(&data_map);
                        if occurence == 0 { // not in a loop yet
                            if !data_hash.contains_key(&hasher) {
                                data_hash.insert(hasher, i);
                            } else {
                                
                                let cycle_start = (data_hash[&hasher] + 1) / 4;
                                let duration_of_cycle = (i + 1) / 4 - cycle_start;
                                //println!("we are in a loop with {} duration : {} ", cycle_start,  duration_of_cycle);

                                //We need to find wich occurence of the loop for the the 1000000000th cycle
                                occurence = (1000000000 - cycle_start) % duration_of_cycle;
                                //println!("calc of good occurence {} ", occurence); 
                                if occurence == 0 {
                                        break;
                                }
                                //break; // we are in a loop !
                            }
                        }
                        else 
                        {
                            occurence -= 1;
                            if occurence == 0 {
                                break;
                            }
                        }
                    } 
                } 
            } 
        }
    }
    
    // 93093 too low
    println!("total part 2 : {}", calc_res(&data_map));

}


fn tilt(data_map : &mut Vec<Vec<char>>, direction : (i32,i32)) -> () {

    //println!("direction {} {}", direction.0, direction.1);
    let vec_y : Vec<usize> = if direction.1 > 0 {
        (0..=data_map.len() -1).rev().collect()
    } else {
        (0..=data_map.len() -1).collect()
    };
    
    let vec_x : Vec<usize> = if direction.0 > 0 {
        (0..=data_map[0].len() -1).rev().collect()
    } else {
        (0..=data_map[0].len() -1).collect()
    };
    
    for y in vec_y.iter() {
        for x in vec_x.iter() {
            if data_map[*y][*x] == 'O' {
                //println!("{} {}", *x, *y);
                data_map[*y][*x] = '.';
               
                if direction.0 == 0 { // we move up or down
                    let mut tmp :  i32 = (*y as i32) + direction.1;
                    while (tmp < data_map.len() as i32) && (tmp >= 0) && (data_map[tmp as usize][*x] == '.') {
                        tmp = tmp + direction.1;
                    }
                    data_map[(tmp - direction.1) as usize][*x] = 'O';
                }
                if direction.1 == 0 { // we left or right
                    let mut tmp :  i32 = (*x as i32) + direction.0;
                    while (tmp < data_map[0].len() as i32) && (tmp >= 0) && (data_map[*y][tmp as usize] == '.') {
                        tmp = tmp + direction.0;
                    }
                    data_map[*y][(tmp - direction.0) as usize] = 'O';
                }

            }
        }
    }
}   

fn calc_res(data_map : &Vec<Vec<char>>) -> usize {
    
    let line_count = data_map.len();
    //println!("line_count {}", line_count);
    let mut obstacle_array : Vec<usize> = Vec::new();
    let mut total = 0;
    let mut i = 0;

    for tmp in data_map.iter() {
        
        if obstacle_array.len() == 0 {
            for _ in 0..data_map[0].len() {
                obstacle_array.push(0);
            }
        }

        for c in tmp.iter() {
            if *c == 'O' {
                total += line_count - i;
            }

        }
        i += 1;
    }

    //println!("{}", total);
    return total;
}

fn hash_vec<T: Hash>(vec: &Vec<T>) -> u64 {
    let mut hasher = DefaultHasher::new();
    vec.hash(&mut hasher);
    hasher.finish()
}

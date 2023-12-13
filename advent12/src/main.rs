use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use std::collections::HashMap;
//use std::collections::HashSet;
use std::process::exit;


fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("Hello, world!");
    
    if args.len() < 2 { println!("oops you forgot the argument which should be a file"); exit(1);}
    let file_path = &args[1];
    let mut total = 0;
    let re_left = Regex::new(r"[\.\#\?]+").expect("regexp creation issue");
    let re_number = Regex::new(r"[0-9]+").expect("regexp creation issue");

    // parsing of the file and working at the same time
    // loop through the lines of the file
    let lines = read_lines(file_path).expect("can't read entry file");
    for line in lines {
        let line_str = line.expect("error in line reading");
        if line_str.len() == 0 { continue; }
        let numbers: Vec<i32> = re_number.find_iter(&line_str).map(|x| x.as_str().parse::<i32>().unwrap()).collect();
        let sauna_left = re_left.find(&line_str).unwrap().as_str();
        let five_sauna_left = format!("{}?{}?{}?{}?{}", sauna_left, sauna_left,sauna_left,sauna_left,sauna_left);
        let five_numbers_right: Vec<i32> = numbers.iter().cloned().cycle().take(numbers.len() * 5).collect();
        
        let mut memory: HashMap<(usize, usize, i32), usize> = HashMap::new();

        println!("{} {:?}", five_sauna_left, five_numbers_right);
        total += calc_possibilities(&five_sauna_left, 0, &five_numbers_right, 0, numbers[0], &mut memory);
        //total += calc_possibilities(&sauna_left, 0, &numbers, 0, numbers[0], &mut memory);
        println!("{} and total : {}", line_str, total);
    }
    println!("{}", total);
    // part1 8922 too high
}

//....###.?##?...

fn calc_possibilities(sauna_left: &str, index_left:usize, numbers: &Vec<i32>, mut index_right:usize, mut current_num:i32, memory: &mut HashMap<(usize, usize, i32), usize> ) -> usize {

    let tmp;
    let mut tmp2 = 0;
    if memory.contains_key(&(index_left, index_right, current_num)) {
        return memory[&(index_left, index_right, current_num)];
    }

    //println!("index left : {}, index right : {}, current : {}", index_left, index_right, current_num);
    // we are at the end of the string :)
    if index_left == sauna_left.len() {
        // if we are at the end of the number array too then it's a win :)
        if (index_right >= numbers.len() - 1) && (current_num <= 0) {
            tmp = 1;
        } else {
            tmp = 0;
        }
    } else {
        match sauna_left.chars().nth(index_left) {
            Some('#') => {
                if current_num <= 0 { // if there a # then we need a number if no number then out
                    tmp = 0
                } else {
                    tmp = calc_possibilities(sauna_left, index_left + 1, numbers, index_right, current_num - 1, memory);
                    memory.insert((index_left + 1, index_right, current_num - 1), tmp);
                }
            },
            Some('.') => {
                if (current_num > 0) && (current_num != numbers[index_right]) {
                    tmp = 0;
                } else {
                    // we are at the end of a number 
                    if current_num == 0 {
                        index_right += 1;
                        if numbers.len() == index_right {
                            current_num = -1;
                        } else {
                            current_num = numbers[index_right];
                        }        
                    }
                    tmp =  calc_possibilities(sauna_left, index_left + 1, numbers, index_right, current_num, memory);
                    memory.insert((index_left + 1, index_right, current_num), tmp);
                }
            },
            Some('?') => {
                //We combine what was done before
                //either the ? is a # 
               
                if current_num <= 0 { 
                    tmp = 0; 
                } else { 
                    tmp = calc_possibilities(sauna_left, index_left + 1, numbers, index_right, current_num - 1, memory) ;
                    memory.insert((index_left + 1, index_right, current_num - 1), tmp);
                }

                //either the ? is a .
                if (current_num > 0) && (current_num != numbers[index_right]) {
                    tmp2 = 0;
                } else {
                    if current_num == 0 {
                        index_right += 1;
                        if numbers.len() == index_right {
                            current_num = -1;
                        } else {
                            current_num = numbers[index_right];
                        }        
                    }
                    tmp2 = calc_possibilities(sauna_left, index_left + 1, numbers, index_right, current_num, memory);
                    memory.insert((index_left + 1, index_right, current_num), tmp2);

                }

            },
            _ => {
                println!("should never go here except if the input is corrupt");
                exit(1);
            }
        }
    }
    return tmp + tmp2;
}


/// The output is wrapped in a Result to allow matching on errors
/// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


use std::env;
use std::fs::File;
use std::io::{self, BufRead};


//use std::path::Path;
//use std::collections::HashMap;
//use std::collections::HashSet;
use std::process::exit;
use std::cmp::max;

struct OneCase {
    case_char: char,
    went_up: bool,
    went_down: bool,
    went_left: bool,
    went_right: bool,
}
 


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

    let mut data_map : Vec<Vec<OneCase>> = Vec::new();

    let mut y = 0;
    for line in lines {
        let line_str = line.expect("error in line reading");
        if line_str.len() == 0 { 
            break; 
        }
        if data_map.len() == y {
            data_map.push(Vec::new());
        }
        for c in line_str.chars() {
            data_map[y].push(
                OneCase {
                    case_char: c,
                    went_up: false,
                    went_down: false,
                    went_left: false,
                    went_right: false,
                }       
            );
        }
        y += 1;
    }
    display_data(&data_map);

    let direction = (0, 1);
    // from top to bottom
    let mut total = 0;
    for i in 0..data_map.len() {
        let position = (i as i32, 0);
        move_beam(&mut data_map, position, direction);
        total = max(total, calc_res_and_erase(&mut data_map));
    }

    let direction = (0, -1);
    for i in 0..data_map.len() {
        let position = (i as i32, (data_map[0].len() -1) as i32);
        move_beam(&mut data_map, position, direction);
        total = max(total, calc_res_and_erase(&mut data_map));
    }

    let direction = (1, 0);
    for i in 0..data_map[0].len() {
        let position = (0, i as i32);
        move_beam(&mut data_map, position, direction);
        total = max(total, calc_res_and_erase(&mut data_map));
    }

    let direction = (-1, 0);
    for i in 0..data_map[0].len() {
        let position = ((data_map.len() -1) as i32, i as i32);
        move_beam(&mut data_map, position, direction);
        total = max(total, calc_res_and_erase(&mut data_map));
    }

    //move_beam(&mut data_map, position, direction);
    println!("total : {}", total);

}


fn display_data(data_map : &Vec<Vec<OneCase>>)
{
    data_map.iter().for_each(|element| {
        element.iter().for_each(|x|  print!("{}", x.case_char));
        println!("");
    }); 
}

fn move_beam(data_map : &mut Vec<Vec<OneCase>>, position : (i32,i32), direction : (i32,i32)) -> () {

    
    //out of bound
    if (position.0 < 0) 
       || (position.1 < 0) 
       || (position.0 >= data_map.len() as i32) 
       || (position.1 >= data_map[0].len() as i32) {
        return ;
    }

    let x = position.1 as usize;
    let y = position.0 as usize;


    match direction {
        (0, 1) => if data_map[y][x].went_right { return ; } else {data_map[y][x].went_right = true},
        (0, -1) => if data_map[y][x].went_left { return ; } else {data_map[y][x].went_left = true},
        (-1, 0) => if data_map[y][x].went_up { return ; } else {data_map[y][x].went_up = true},
        (1, 0) => if data_map[y][x].went_down { return ; } else {data_map[y][x].went_down = true},
        _ => {}
    }

   // println!("y: {}, x: {}, direction :({},{}), char : {}", position.0, position.1, direction.0, direction.1, data_map[y][x].case_char );

    match data_map[y][x].case_char {
        '.' => {
            // no change of direction, it is the easiest case
            move_beam(data_map, (position.0 + direction.0, position.1 + direction.1), direction);
            return;
        },
        '-' => {
            // if it the direction is left or right we go ignore
            if direction.0 == 0 {return  move_beam(data_map, (position.0 + direction.0, position.1 + direction.1), direction);}
            
            // we split the light
            move_beam(data_map, (position.0 , position.1 + 1), (0, 1));
            move_beam(data_map, (position.0 , position.1 - 1), (0, -1));
            return;
        },
        '|' => {
            // if it the direction is up or down we go ignore
            if direction.1 == 0 {return  move_beam(data_map, (position.0 + direction.0, position.1 + direction.1), direction);}
            
            //we split the light
            move_beam(data_map, (position.0 + 1, position.1), (1, 0));
            move_beam(data_map, (position.0 - 1, position.1), (-1, 0));
            return;    
        },
        '\\' => {
            // if (0, 1) -> (1, 0) ; if (0, -1) -> (-1, 0) ; if (1, 0) -> (0, 1); if (-1, 0) -> (0, -1)
            move_beam(data_map, (position.0 + direction.1, position.1 + direction.0), (direction.1, direction.0));
            return;
        },
        '/' => {
            // if (0, 1) -> (-1, 0) ; if (0, -1) -> (1, 0) ; if (1, 0) -> (0, -1); if (-1, 0) -> (0, +1)
            move_beam(data_map, (position.0 - direction.1, position.1 - direction.0), (-direction.1, -direction.0));
            return;
        },
        _ => {println!("Joker ?");}
    }
}   

fn calc_res_and_erase(data_map : &mut Vec<Vec<OneCase>>) -> usize {
    
let mut total = 0;
    for tmp in data_map.iter_mut() {
        
        for case in tmp.iter_mut() {
            if case.went_down || case.went_up || case.went_right || case.went_left {
                total += 1;
            }
            case.went_down = false;
            case.went_up = false;
            case.went_right = false;
            case.went_left = false;
        }
    }
    return total;
}

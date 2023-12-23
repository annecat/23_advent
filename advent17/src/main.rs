use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::process::exit;
use std::collections::VecDeque; 
use std::collections::HashMap;


struct OneCase {
    case_num: i8,
    min_score: usize,
    been_there : usize,
}
 
#[derive(Debug, PartialEq, Eq)]
struct MoveMemory {
    position: (i32, i32),
    direction: (i8, i8),
    score : usize,
    straight_line : i8,
}


impl Ord for MoveMemory {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for MoveMemory {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

const MIN_STRAIGHT_LINE : i8 = 3;
const MAX_STRAIGHT_LINE : i8 = 9;

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
    let mut memory : HashMap<((i32, i32),(i8, i8), i8), usize> = HashMap::new();

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
                    case_num: c.to_digit(10).expect("I want a number !") as i8,
                    min_score : 9999,
                    been_there: 0
                }       
            );
        }
        y += 1;
    }
    display_data(&data_map);
    let mut next_move_list : VecDeque<MoveMemory> = VecDeque::new();

    next_move_list.push_back(MoveMemory { 
        position : (0,0), 
        direction : (0, 1), 
        score : data_map[0][0].case_num as usize,
        straight_line: 1
    });
    next_move_list.push_back(MoveMemory { 
        position : (0,0), 
        direction : (1, 0), 
        score : data_map[0][0].case_num as usize,
        straight_line: 1
    });
    memory.insert(((0,0), (0,1), 0),data_map[0][0].case_num as usize);
    memory.insert(((0,0), (1,0), 0),data_map[0][0].case_num as usize);

    while next_move_list.len() > 0
    {
        let current_move = next_move_list.pop_front().expect("What ?!?");

/*        if current_move.position.0 == 0 || current_move.position.0 == 1 {
            println!("position courrante {} {}, direction : {} {}, straight_line : {}, score : {}", current_move.position.0, current_move.position.1,
            current_move.direction.0, current_move.direction.1,
            current_move.straight_line, current_move.score);
        }  */ 

        //Let's go forward if we are under MAX_STRAIGHT_LINE
        if  current_move.straight_line < MAX_STRAIGHT_LINE {
            //println!("go straight");
            let new_position = (current_move.position.0 + current_move.direction.0 as i32, current_move.position.1 + current_move.direction.1 as i32);
            calc_and_insert_move(&mut memory, &mut data_map ,&mut next_move_list,  current_move.score, current_move.straight_line + 1, new_position, current_move.direction );
        }

       
        if current_move.straight_line >= MIN_STRAIGHT_LINE { //  can not turn
            //println!("turn right");
            let mut new_direction = turn_right(current_move.direction);
            let mut new_position = (current_move.position.0 + new_direction.0 as i32, current_move.position.1 + new_direction.1 as i32);
            calc_and_insert_move(&mut memory, &mut data_map , &mut next_move_list, current_move.score, 0, new_position, new_direction );
            
            // println!("turn left");
            new_direction = turn_left(current_move.direction);
            new_position = (current_move.position.0 + new_direction.0 as i32, current_move.position.1 + new_direction.1 as i32);
            calc_and_insert_move(&mut memory, &mut data_map , &mut next_move_list, current_move.score, 0, new_position, new_direction);
        }
        
        data_map[current_move.position.0 as usize][current_move.position.1 as usize].been_there += 1;
    }
   
    //println!("been there ?");
    //_display_been_there(&data_map);
    println!("score final");
    display_score_hash(memory, &mut data_map);
    //display_score(&data_map);

//    println!("total : {}", total);
    // part 2: 1429 too high
    // 1362

}

fn calc_and_insert_move(memory: &mut HashMap<((i32, i32), (i8, i8),i8), usize>, data_map : &mut Vec<Vec<OneCase>>, next_move_list: &mut VecDeque<MoveMemory>, old_score : usize, steps : i8, new_position : (i32, i32), new_direction : (i8, i8)) -> () {
    
    let x = new_position.1 as usize;
    let y = new_position.0 as usize;
    
    //out of bound check
    if (new_position.0 < 0) || (new_position.1 < 0) 
    || (new_position.0 >= data_map.len() as i32) || (new_position.1 >= data_map[0].len() as i32) {
        return;
    }

    let new_move = MoveMemory { 
            position : new_position, 
            direction : new_direction, 
            score : old_score + data_map[y][x].case_num as usize,
            straight_line: steps
    };

    // best score check
    if (!memory.contains_key(&(new_position, new_direction, steps))) || 
    (memory[&(new_position, new_direction, steps)] > new_move.score) {

/*        if (new_position.0 == 1) && (new_position.1 == 3) {
            println!("position d'après {} {}, direction : {} {}, straight_line : {}, score : {}, data {}, min_score {}", new_position.0, new_position.1,
            new_direction.0, new_direction.1,
            steps, old_score,
            data_map[y][x].case_num,
            new_move.score);
        }*/
        memory.entry((new_position, new_direction, steps)).or_insert(new_move.score);
        insert_sorted(next_move_list, new_move);
    }
    
}


fn turn_right(direction : (i8, i8)) -> (i8, i8) {
    match direction {
        (0,1) => return (1,0), // East to South
        (1,0) => return (0,-1), // South to West
        (0,-1) => return (-1,0), // West to North
        (-1,0) => return (0,1), // North to East
        _ => (0,0)
    }  
}

fn turn_left(direction : (i8, i8)) -> (i8, i8) {
    match direction {
        (0,1) => return (-1,0), // East to North
        (-1,0) => return (0,-1), // North to West
        (0,-1) => return (1,0), // West to South
        (1,0) => return (0,1), // South to East
        _ => (0,0)
    }
}


fn insert_sorted(deque: &mut VecDeque<MoveMemory>, value: MoveMemory) {
    // Binary search to find the index where the value should be inserted
    let index_to_insert = match deque.binary_search(&value) {
        Ok(index) => index,  // Value already exists, insert at the found index
        Err(index) => index, // Value doesn't exist, insert at the index where it would be
    };
    // Insert the value at the determined index
    deque.insert(index_to_insert, value);
}




fn display_data(data_map : &Vec<Vec<OneCase>>)
{
    data_map.iter().for_each(|element| {
        element.iter().for_each(|x|  print!("{}", x.case_num));
        println!("");
    }); 
}

fn display_score(data_map : &Vec<Vec<OneCase>>)
{
    for i in 0..data_map.len() {
        for j in 0..data_map[0].len() {
            print!("{} ", data_map[i][j].min_score);
        }    
        println!("");
    }
}


fn display_score_hash(memory: HashMap<((i32, i32), (i8, i8),i8), usize>, data_map : &mut Vec<Vec<OneCase>>)
{
    let mut min = (data_map.len() -1 + data_map[0].len() -1) *9;
    memory.iter().for_each(|(key, element)| {
        if data_map[key.0.0 as usize][key.0.1 as usize].min_score > *element {
            data_map[key.0.0 as usize][key.0.1 as usize].min_score = *element;
        }
        if (key.0.0 as usize == data_map.len() -1) && (key.0.1 as usize == data_map[0].len() -1) {
            if (*element < min) && (key.2 >= MIN_STRAIGHT_LINE) {
                min = *element;
            }
           // println!("{:?} {}", key, element);
        }
    }); 
    println!("final score : {}", min - data_map[0][0].min_score);

}


fn _display_been_there(data_map : &Vec<Vec<OneCase>>)
{
    data_map.iter().for_each(|element| {
        element.iter().for_each(|x|  print!(" {} ", x.been_there));
        println!("");
    }); 
}



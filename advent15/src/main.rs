use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::process::exit;
use std::collections::HashMap;



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

    let mut boxes : HashMap<usize, Vec<(String, u32)>> = HashMap::new();
    //let mut lens : HashMap<(String, u32), usize> = HashMap::new();

   
    let mut box_number = 0;
    let mut to_add :bool = false;
    let mut lens_value = 0;
    let mut current_string = String::new();      

    for line in lines {
        let line_str = line.expect("error in line reading");
        if line_str.len() == 0 { 
            break; 
        }
       // parsing my file and at the same time computing the box !
        for c in line_str.chars() {
            match c {
                '-' => to_add = false,
                '=' => to_add = true,
                '0'..='9' => lens_value = c.to_digit(10).unwrap(),// It's a number 
                ',' => { // begin the work !
                   // println!("{:?}", boxes);
                  //  println!("box number : {}, current String : {}, lens value : {}, adding : {}", box_number, current_string, lens_value, to_add);
                    computebox(box_number , current_string , lens_value, to_add, &mut boxes );        
                    box_number = 0;
                    current_string = String::new();
                    lens_value = 0;
                },
                _ => { 
                    current_string.push(c);
                    let ascii_char = c as u8;
                    box_number += ascii_char as usize;
                    box_number = (box_number * 17) % 256;
                }
            }
        }
    }

   // println!("{:?}", boxes);
   // println!("box number : {}, current String : {}, lens value : {}, adding : {}", box_number, current_string, lens_value, to_add);
   // There is no ',' at the end of the file so here we are computing a last time
    computebox(box_number , current_string , lens_value, to_add, &mut boxes );
    println!("{:?}", boxes);

    // loop to count the point :)
    let mut total = 0;
    for (key, one_box) in boxes {
        let mut i = 1;
        for (_, lens_number) in one_box {
            total += (key + 1) * i * lens_number as usize;
            i += 1;
        }
    }
    println!("{}", total)

}


fn computebox(box_number : usize, current_string : String, lens_value: u32, to_add :bool, boxes : &mut HashMap<usize, Vec<(String, u32)>> ) -> () {
    if to_add {                
        // Check if the key exists in the HashMap
        if let Some(vec) = boxes.get_mut(&box_number) {
            // If the key exists, check if the lens exists in the vector
            let mut found = false;
            for (key, value) in &mut *vec {
                if *key == current_string {
                    *value = lens_value;
                    found = true;
                }
            }
            if !found {
                vec.push((current_string.clone(), lens_value));
            }

        } else {
            // If the key doesn't exist, insert a new vector
            boxes.insert(box_number, vec![(current_string.clone(), lens_value)]);
        }

        //lens.insert((current_string, lens_value), box_number);
    } else {

        if let Some(vec) = boxes.get_mut(&box_number) {
            vec.retain(|x| (x.0 != current_string) && (x.1 != lens_value));
        }
    }
}

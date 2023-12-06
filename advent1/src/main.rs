use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;


fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(args);

    let file_path = &args[1];
    let re1 = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let re2 = Regex::new(r".*(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let mut total = 0 ;

        if let Ok(lines) = read_lines(file_path) {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(ip) = line {
                    println!("{}", &ip);
                    //let cap = first.find_iter(&ip);
                    let matches: Vec<_> = re1.find_iter(&ip).map(|m| m.as_str()).collect();
                    let test2 = re2.captures(&ip);
//                    dbg!(test2.unwrap().len());
                    //println!("{}", &test2[1]);
                    //println!("{}", test2[0].unwrap().as_str());
                    //println!("{:?}", re2.captures(&ip).unwrap().get(size).unwrap().as_str());


                    if matches.len() > 0 {

                        let size = test2.unwrap().len() -1;
                    
                        let tmp = re2.captures(&ip).unwrap().get(size).unwrap().as_str();

                        let first: i32 = trans_to_int(matches[0].to_string());
                        let last: i32 = trans_to_int(tmp.to_string());
                        println!("{} {}", matches[0], first);
                        println!("{}",  last);
                                                
                        total = total + first * 10 + last;
                    }
                }
            }
        }
    println!("{}", total);

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn trans_to_int(str : String) -> i32 {
    match str.as_str() {
        //"zero" => return 0,
        "one" => return 1,
        "two" => return 2,
        "three" => return 3,
        "four" => return 4,
        "five" => return 5,
        "six" => return 6,
        "seven" => return 7,
        "eight" => return 8,
        "nine" => return 9,
        _ => return str.parse().unwrap(),
    } 
 //54871 is too high   
}
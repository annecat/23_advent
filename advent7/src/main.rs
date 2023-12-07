use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::exit;
use regex::Regex;
use std::cmp::Ordering;
use lazy_static::lazy_static;
use std::collections::HashMap;


// Define the HashMap type
lazy_static! {
    static ref CARDS_VALUE: HashMap<char, usize> = {
        let mut map = HashMap::new();
        map.insert('2', 1);
        map.insert('3', 2);
        map.insert('4', 3);
        map.insert('5', 4);
        map.insert('6', 5);
        map.insert('7', 6);
        map.insert('8', 7);
        map.insert('9', 8);
        map.insert('T', 9);
        map.insert('J', 0);
        map.insert('Q', 11);
        map.insert('K', 12);
        map.insert('A', 13);
        map
    };
}

const FIVE_OF_KIND: usize = 7;
const FOUR_OF_KIND: usize = 6;
const FULLHOUSE: usize = 5;
const THREE_OF_KIND: usize = 4;
const TWO_PAIR: usize = 3;
const PAIR: usize = 2;
const NONE: usize = 1;

fn main() {
    

    // parse entries from a file in argument
    let mut entry_data: Vec<(String, usize)> = Vec::new(); 

    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 { println!("oops you forgot the argument which should be a file"); exit(1);}
    let file_path = &args[1];
    // loop through the lines of the file
    let lines = read_lines(file_path).expect("can't read entry file");
    let regex_line = Regex::new(r"(.+) (\d+)").expect("regexp creation issue");


    for line in lines {
        let line_str = line.expect("error in line reading");

        //println!("{}", line_str);

        //line is not empty
        if line_str.len() > 0 {
            let capture = regex_line.captures(&line_str).unwrap();
            //dbg!(capture);
            //println!("{} {}", capture.get(1).unwrap().as_str(), capture.get(2).unwrap().as_str());
            entry_data.push((capture.get(1).unwrap().as_str().to_string(),capture.get(2).unwrap().as_str().parse::<usize>().unwrap()))
        }

    }
    entry_data.sort_by(|a, b| sort_hand(&a.0, &b.0));


    println!("{:?}", entry_data);

    let mut total = 0;
    let mut i = 1;
    for entry in entry_data {
        total = i * entry.1 + total;
        i += 1;
    }
    


    println!("{}", total);
    // 254507498 : too high
    // 254640550
 }

fn identify_hand(hand: &String) -> usize {

    let mut cards : HashMap<char, usize> = HashMap::new();
    let mut max_value = 0;

    for c in hand.chars() { 
        *cards.entry(c).or_insert(0) += 1;
        if max_value < cards[&c] && c != 'J' {
            max_value = cards[&c];
        }
        // do something with `c`
    }

 
    let nb_joker = if cards.contains_key(&'J') { cards[&'J']} else {0} ;
    
    //println!("hand:{}, {:?}, joker {}", hand, cards, nb_joker);

    if nb_joker + max_value == 5 { return FIVE_OF_KIND; }
    if nb_joker + max_value == 4 { return FOUR_OF_KIND; }

    // fullhouse : AAAJB + AAJBB + AAABB + AJJBB + AAJJBB +AAAJJ
    // three : AAABC  + AAJBC + ABC
    if nb_joker + max_value == 3 { 
        if ((nb_joker == 0) && (cards.len() == 2)) ||((nb_joker != 0) && (cards.len() == 3)) {
            return FULLHOUSE;
        }
        return THREE_OF_KIND;
    }

    if nb_joker + max_value == 2 { 
        if ((nb_joker == 0) && (cards.len() == 3)) || ((nb_joker != 0) && (cards.len() == 4)) {
            return TWO_PAIR;
        }
        return PAIR;
    }

    return NONE;
}

fn tie_breaker(hand1 : &String, hand2: &String) -> Ordering {
    let mut i = 0;

    for c1 in hand1.chars() { 
        let c2 = hand2.chars().nth(i).unwrap();

        if CARDS_VALUE.get(&c1).unwrap() == CARDS_VALUE.get(&c2).unwrap() {
            i += 1;
            continue;
        }
        if CARDS_VALUE.get(&c1).unwrap() > CARDS_VALUE.get(&c2).unwrap() {
            return Ordering::Greater;
        }
        else {
            return Ordering::Less;
        }
    }
    return Ordering::Equal;

}


fn sort_hand(hand1 : &String, hand2: &String) -> Ordering {

    let tmp = identify_hand(hand1);

    let tmp2 = identify_hand(hand2);

    if tmp == tmp2 {
        return tie_breaker(hand1, hand2);
    }
    
    if tmp > tmp2 {
        return Ordering::Greater;
    }

    return Ordering::Less;
}


/// The output is wrapped in a Result to allow matching on errors
/// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn _identify_hand_part1(hand: &String) -> usize {

    let mut cards : HashMap<char, usize> = HashMap::new();
    let mut max_value = 0;

    for c in hand.chars() { 
        *cards.entry(c).or_insert(0) += 1;
        if max_value < cards[&c] {
            max_value = cards[&c];
        }
        // do something with `c`
    }

    //println!("{:?}", cards);
   
    
    if cards.len() == 1 { return FIVE_OF_KIND; }

    if cards.len() == 2 { //either a fullhouse or four of a kind
        return if max_value == 4 { FOUR_OF_KIND } else { FULLHOUSE };
    }

    if cards.len() == 3 { // three of a kind or pair 
        return if max_value == 3 { THREE_OF_KIND } else { TWO_PAIR };
    }

    if cards.len() == 4 { return PAIR; }

    return NONE;
}
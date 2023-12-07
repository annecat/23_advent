//use regex::Regex;

fn main() {
    

    // data with a pair of time and distance
    //let entry_data = vec![(7,9),(15,40),(30,200)]; 
    // Part 1 : let entry_data = vec![(40,277),(82,1338),(91,1349),(66,1063)]; 
    //Part 2 :
    let entry_data = vec![(40829166,277133813491063)]; 

    let mut total = 0;

    for entry in entry_data {
        if total == 0 {
            total = nb_winnning_solution_bruteforce(entry);
        } else {
            total = total * nb_winnning_solution_bruteforce(entry);
        }
    }

    println!("{}", total);

}

fn nb_winnning_solution_bruteforce(race : (usize, usize)) -> usize {

    let mut res = 0;
    let mut i = 1;

    while i < race.0 {
        let tmp = i * (race.0 - i);
        //println!("{}", tmp);
        if tmp > race.1 {
            res += 1;
        }
        i += 1;
    }
    return res;
}


fn _nb_winnning_solution(race : (usize, usize)) -> usize {

    // Toto : Do with the math instead of bruteforce
}
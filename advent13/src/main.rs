use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//use std::collections::HashMap;
//use std::collections::HashSet;
use std::process::exit;
use std::cmp::max;

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("Hello, world!");
    
    if args.len() < 2 { println!("oops you forgot the argument which should be a file"); exit(1);}
    let file_path = &args[1];

    // parsing of the file and working at the same time
    // loop through the lines of the file
    let lines = read_lines(file_path).expect("can't read entry file");

    let mut valley_map = Vec::new();

    let mut possible_start = Vec::new();

    let mut previous = String::new();

    let mut i = 0;

    let mut total = 0;

    for line in lines {
        let line_str = line.expect("error in line reading");
        if line_str.len() == 0 { 
           let mut tmp = 100 * find_pattern_number(&valley_map, &possible_start);
           

           if tmp == 0 {
           // we did the work horizontally now for the vertical
           let transposed_valley_map = transpose_matrix(valley_map.clone());
           previous = String::new();
           possible_start.clear();
           i = 0;

           transposed_valley_map.iter().for_each(|element| {
            if calc_nb_smudge(&previous, &element) <= 1 {
                possible_start.push(i);
            }
            previous = element.clone();
            i += 1;
           });
          
                tmp = find_pattern_number(&transposed_valley_map, &possible_start);
           }

           previous = String::new();
           possible_start.clear();
           valley_map.clear();
           i = 0;
           total += tmp;
           //println!(" ________________________________________ temp {} et total {}", tmp, total);

            continue; 
        }

        valley_map.push(line_str.clone());


        if calc_nb_smudge(&previous, &line_str) <= 1 {
            possible_start.push(i);
        }

        previous = line_str.clone();

        i +=1;
    }

    println!("{}", total);

}

fn calc_nb_smudge(str1:&String, str2:&String) -> usize {
    let mut nb_smudge = 0;
    if str1.len() != str2.len() {
        return max(str1.len(), str2.len())
    }
    for i in 0..str1.len() {
        if str1.chars().nth(i) != str2.chars().nth(i) {
            nb_smudge += 1;
        }
    }
    return nb_smudge;
}

fn find_pattern_number(valley_map :&Vec<String>, possible_start :&Vec<usize>) -> usize {
    //valley_map.iter().for_each(|element| println!("Element: {}", element)); 
    //println!("possible_start {:?}", possible_start);
   
    for start in possible_start.iter() {
        let mut smudge_used = false;
        let mut j = 1;
        for i in (*start as usize)..valley_map.len() {
           
            if i < j  { // end of the tab so all lines were perfect :)
                if smudge_used {
                        return *start;
                    }
                    else {
                        break;
                    }
            }
           //println!("i {}, j {}, &valley_map[i] {}, &valley_map[i - j] {}", i, j, &valley_map[i], &valley_map[i - j]);
       
            let nb_smudge = calc_nb_smudge(&valley_map[i],&valley_map[i - j]);
            if nb_smudge == 1 && smudge_used {
                break;
            }
            if nb_smudge == 1 && !smudge_used {
                smudge_used = true;
            }
            if nb_smudge <= 1 {
                j += 2;
                if i == valley_map.len() - 1 {// end of the tab so all lines were perfect :)
                    if smudge_used {
                        return *start;
                    }
                    else {
                        break;
                    }

                }
            }
            else {
                break;
            }
        }
       // if j >= valley_map.len() { // end of the tab so all lines were perfect :)
         //   return *start;
       // }
    }
    return 0;

}

/// The output is wrapped in a Result to allow matching on errors
/// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn transpose_matrix(matrix: Vec<String>) -> Vec<String> {
    // Check if the matrix is empty
    if matrix.is_empty() {
        return Vec::new();
    }

    // Determine the number of rows and columns in the matrix
    let num_rows = matrix.len();
    let num_cols = matrix[0].len();

    // Create a new vector to store the transposed matrix
    let mut transposed_matrix = vec![String::with_capacity(num_rows); num_cols];

    // Iterate through the original matrix and fill the transposed matrix
    for row in matrix {
        // Ensure that the number of columns is consistent in the original matrix
        if row.len() != num_cols {
            panic!("The number of columns is not consistent in the matrix.");
        }

        // Iterate through characters in the row and append them to the corresponding columns
        for (col_index, char_value) in row.chars().enumerate() {
            transposed_matrix[col_index].push(char_value);
        }
    }

    transposed_matrix
}
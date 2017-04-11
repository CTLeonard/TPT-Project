use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::error::Error;
use std::collections::HashMap;
use std::path::Path;

//#[derive(PartialEq, Eq, Hash)]
type Index = (String, String);

fn frequency_matrix(file: &Path) -> (HashMap<Index, f32>, HashMap<String, f32>) {
    // function to create a HashMap representation of a frequency matrix
    
    // create a file struct and open and pass to buffer reader
    let display = file.display();
    let f = match File::open(&file) {
        Err(why) => panic!("couldn't open {}: {}",
                           display,
                           why.description()),
        Ok(file) => file,
    };
    let reader = BufReader::new(f);
    
    // create a row total and frequency matrix HashMap
    let mut total: HashMap<String, f32> = HashMap::new();
    let mut f_matrix: HashMap<Index, f32> = HashMap::new();
    for line in reader.lines() {
        // iterate through each line of the file and unwrap it
        let line_str = match line {
            Err(why) => panic!("{}", why.description()),
            Ok(ln) => ln,
        };
        
        // split the string by commas and then record the from to state
        let states: Vec<&str> = line_str.split(',').collect();
        for pair in states.windows(2) {
            let key: Index = (pair[0].to_string(), pair[1].to_string());
            let r_name: String = pair[0].to_string();
            if !f_matrix.contains_key(&key) {
                f_matrix.insert(key, 1f32);
            } else {
                *f_matrix.get_mut(&key).unwrap() += 1f32;
            }
            
            if !total.contains_key(&r_name) {
                total.insert(r_name, 1f32);
            } else {
                *total.get_mut(&r_name).unwrap() += 1f32;
            }
        }
    }

    (f_matrix, total)
 }

fn write_transition_matrix(f_matrix: HashMap<Index, f32>, totals: HashMap<String, f32>) {
    // computes the transition probabilities and writes the results to a 
    // csv
    
    // name the path to write results to and create the file
    let file_path = Path::new("transition.tsv");
    let display = file_path.display();
    
    let mut file = match File::create(&file_path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           why.description()),
        Ok(file) => file,
    };
    
    // iterate through hash and divide by row totals before writing to csv
    for (k, v) in f_matrix.iter() {
        let prob = { v / totals.get(&k.0).unwrap() };
        
        let line = format!("{} {} {}\n", k.0, k.1, v);
        
        match file.write(line.as_bytes()){
            Err(why) => panic!("An error has occured: {}!", why.description()),
            Ok(_) => {},
        };
    }
}

fn main() {
    let path = Path::new("/Users/edelsonc/Documents/Documents/School/Data_Science/Distributed_Comp/teachers/rust_explore/user_paths.csv");
    let (frequency, totals) = frequency_matrix(&path);
    write_transition_matrix(frequency, totals);
}

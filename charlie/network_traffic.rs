extern crate csv;
extern crate time;

use std::collections::HashMap;
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;

type Line = (String, time::Tm, String, String, String, String, String);

#[derive(Debug)]
struct TimeAndPath {
    time: time::Tm,
    path: String,
}

fn parse_csv(file: &Path) -> Vec<Line> {
    // Parses the newcollege_query_data.csv and put results into a vector
    let mut rdr = csv::Reader::from_file(&file).unwrap().has_headers(true);

    // create string formater, parse csv, and convert times to Tm structures
    let mut rows: Vec<Line> = Vec::new();
    let frmt_str = "%Y-%m-%d %H:%M:%S %Z";

    for row in rdr.decode() {
        let (id, r_time, s_path, price,grade, device, search):
                (String, String, String, String, String, String, String)
                = row.unwrap();

        // change format string to Tm structure
        let time_strct = time::strptime(&r_time, &frmt_str);

        // check to make sure the time converts correctly
        if time_strct.is_ok() {
            let row_i: Line = (id, time_strct.unwrap(), s_path, price,grade, device, search);
            rows.push(row_i);
        } 
    }
    rows
}

fn hashmap_users(data: Vec<Line>) -> HashMap<String, Vec<TimeAndPath>> {
    let mut users = HashMap::new();
    for row in data {
        let time_path = TimeAndPath { time: row.1, path: row.2 };
        
        if !users.contains_key(&row.0) {
            let v: Vec<TimeAndPath> = Vec::new();
            users.insert(row.0.to_string(), v);
        }
        users.get_mut(&row.0).unwrap().push(time_path);
        
    }
    users
}

fn get_paths(user_map: HashMap<String, Vec<TimeAndPath>>) {
    let file_path = Path::new("user_paths.csv");
    let display = file_path.display();
    
    let mut file = match File::create(&file_path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           why.description()),
        Ok(file) => file,
    };


    for vals in user_map.values() {
        let len_vals = vals.len() - 1;
        let mut new_path: String = String::new();
        new_path = new_path + &vals[0].path;
        for i in 0..len_vals {
            if (vals[i].time - vals[i+1].time).num_minutes() > 120 {
                new_path = new_path + "\n";
                
                match file.write(new_path.as_bytes()) {
                    Err(why) => {
                        panic!("couldn't write to {}: {}", display,
                                                            why.description())
                        },
                        Ok(_) => {},
                }
                
                new_path = String::new();
            }
            
            if new_path.is_empty() {
                new_path = new_path + &vals[i+1].path;
            } else {
                new_path = new_path + "," + &vals[i+1].path;
            }
        }
    }
}

fn main() {
    let path = Path::new("/Users/edelsonc/Documents/Documents/School/Data_Science/Distributed_Comp/teachers/newcollege_query_data.csv");
    let rows = parse_csv(&path);
    let user_hash = hashmap_users(rows);
    get_paths(user_hash);
}

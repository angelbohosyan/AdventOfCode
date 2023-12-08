use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::Hash;
use std::io;

///559667

fn main() -> Result<(), io::Error> {
    let file_path = "input.txt";
    let result = fs::read_to_string(file_path)?;

    let mut is_valid:HashMap<usize,HashSet<usize>> = HashMap::new();

    is_valid.insert(0,HashSet::new());
    for (row,content) in result.lines().enumerate() {
        is_valid.insert(row+1,HashSet::new());
        for (column,value) in content.char_indices() {
           if value != '.' && !(value >= '0' && value <= '9') {
               is_valid.get_mut(&(row-1)).unwrap().insert(column-1);
               is_valid.get_mut(&(row-1)).unwrap().insert(column);
               is_valid.get_mut(&(row-1)).unwrap().insert(column+1);
               is_valid.get_mut(&(row)).unwrap().insert(column-1);
               is_valid.get_mut(&(row)).unwrap().insert(column+1);
               is_valid.get_mut(&(row+1)).unwrap().insert(column-1);
               is_valid.get_mut(&(row+1)).unwrap().insert(column);
               is_valid.get_mut(&(row+1)).unwrap().insert(column+1);
           }
        }
    }

    let mut sum: i32 = 0;

    for (row,content) in result.lines().enumerate() {
        let mut number = String::new();
        let mut is_number_valid: bool = false;
        for (column,value) in content.char_indices() {
            if value >= '0' && value <= '9' {
                number.push(value);
                if is_valid.get_mut(&(row)).unwrap().contains(&(column)) {
                    is_number_valid = true
                }
            } else if number != "" {
                if is_number_valid {
                    println!("{}",number);
                    sum += number.parse::<i32>().unwrap();
                    is_number_valid = false
                }

                number.clear();
            }
        }

        if number != "" && is_number_valid {
            println!("{}",number);
            sum += number.parse::<i32>().unwrap();
            is_number_valid = false
        }
    }

    println!("{}",sum);

    Ok(())
}
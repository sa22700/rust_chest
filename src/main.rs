#![allow(dead_code, unused_variables, unused_mut)]

use std::fs::File;
use std::io::{self, Read, ErrorKind};

fn load_items() -> Result<Vec<String>, io::Error> {
    let mut file = File::open("chest.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut items = Vec::new();
    for line in contents.lines() {
        items.push(String::from(line));
    }
    Ok(items)
}

fn impossible_state() {
    panic!("Impossible game state!");
}

fn check_slot(items: &Vec<String>, index: usize) -> Option<&String> {
    items.get(index)
}

fn main() {
    let items = match load_items() {
        Ok(items) => items,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("No chest file found; using the standard supplies.");
                vec![
                    String::from("Rope"),
                    String::from("Torch"),
                    String::from("Potion"),
                ]
            }
            _ => {
                eprintln!("Error loading chest: {error}");
                return;
            }
        },
    };
    println!("The chest contains:");
    for item in &items {
        println!("- {item}");
    }
    for index in [1, 99] {
        match check_slot(&items, index) {
            Some(item) => println!("Slot {index} contains {item}."),
            None => println!("There is no item in slot {index}."),
        }
    }
    // Uncomment to test an unrecoverable error.
    // A backtrace helps locate the call path that led to the panic.
    // impossible_state();
    // Accessing an out-of-range index also causes a panic.
    // let invalid_item = &items[99];
}

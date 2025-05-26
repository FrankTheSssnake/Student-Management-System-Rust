use std::io::{self, Write};
use std::env;

mod extra;
mod student;

use student::*;

fn load_file() {
    println!("File loaded.");
}

fn save_file() {
    println!("File saved.");
}

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let filename: &String = &args[1];
        // TODO
        // Open File

        println!("File provided: {}", filename);
        load_file();
    }

    let mut students: Vec<student::Student> = Vec::new();

    let menu: &str = "\n--- Student Management System ---\n1. Add Student\n2. Display All Students\n3. Search for a Student\n4. Update Student Details\n5. Delete a Student\n6. Exit\nEnter your choice: ";

    loop {
        print!("{}", menu);
        io::stdout().flush().unwrap();

        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();

        
        match input.trim().chars().next {
            Some('1') => add_student(&mut students),
            Some('2') => display_students(&students),
            Some('3') => search_student(&students),
            Some('4') => update_student(&mut students),
            Some('5') => delete_student(&mut students),
            Some('6') => {
                save_file();
                println!("Exiting.");
                std::process::exit(0);
            },
            _ => eprintln!("Invalid Choice.")
        }
    }
    // TODO
    // Close File
}

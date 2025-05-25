use std::io::{self, Write};
use std::env;

mod extra;
mod student;


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

        println!("File provided: {}", filename);
        load_file();
    }

    // define students
    // file_load logic

    let menu: &str = "\n--- Student Management System ---\n1. Add Student\n2. Display All Students\n3. Search for a Student\n4. Update Student Details\n5. Delete a Student\n6. Exit\nEnter your choice: ";

    loop {
        print!("{}", menu);
        io::stdout().flush().unwrap();

        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().chars().next() {
            Some(ch) => {

                match ch {
                    '1' => println!("Add students"),
                    '2' => println!("Display students"),
                    '3' => println!("Search students"),
                    '4' => println!("Update students"),
                    '5' => println!("Delete students"),
                    '6' => {
                        save_file();
                        println!("Exit");
                        std::process::exit(1);
                    },
                    _ => eprintln!("Invalid Choice."),
                }
            },

            None => println!("No input provided!"),
        }
    }

    // close file
}

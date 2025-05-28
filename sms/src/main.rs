use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, Write, BufRead};

mod extra;
mod student;

use student::Student;
use student::*;

use extra::{calculate_widths, fprint_head};

fn load_file(filename: &String, students: &mut Vec<Student>) -> Result<(), String> {

    let file = File::open(&filename).expect("Failed to open file.");

    let reader = io::BufReader::new(file);

    for (ind, line) in reader.lines().enumerate() {
        let line = line.expect("Failed to read file.");

        if ind == 1 {
            calculate_widths(line.as_str());
            continue;
        } else if ind == 2 {
            continue;
        }
        
        let mut parts = line.split("| ");
    
        let id: i32 = parts.next().unwrap().trim().parse().expect("Invalid ID");
        let name: String = parts.next().unwrap().trim().to_string();
        let marks: f32 = parts.next().unwrap().trim().parse().expect("Invalid Marks");
        
        let student: Student = Student{
            id,
            name,
            marks
        };

        students.push(student);
        
    }
    Ok(())
}

fn save_file(filename: &String, students: &[Student]) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)
        .expect("Failed to open file.");

    fprint_head(&mut file);

    for student in students {
        let entry: String = format_entry(student);
        writeln!(file, "{}", entry).expect("Internal Error: Write Operation Failed.");
    }

    println!("File saved.");

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut students: Vec<student::Student> = Vec::new();

    let mut filename: Option<String> = None;

    if args.len() > 1 {
        filename = Some(args[1].clone());
        load_file(&args[1], &mut students)
            .unwrap_or_else(|e| eprintln!("{}\nFailed to load file.", e));
    }

    let menu: &str = "\n--- Student Management System ---\n1. Add Student\n2. Display All Students\n3. Search for a Student\n4. Update Student Details\n5. Delete a Student\n6. Exit\nEnter your choice: ";

    loop {
        print!("{}", menu);
        io::stdout().flush().unwrap();

        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().chars().next() {
            Some('1') => add_student(&mut students),
            Some('2') => display_students(&students),
            Some('3') => search_student(&students),
            Some('4') => update_student(&mut students),
            Some('5') => delete_student(&mut students),
            Some('6') => {
                if let Some(f) = filename {
                    save_file(&f, &students).unwrap_or_else(|e| eprintln!("{}\nFailed to save file.", e))
                }
                println!("Exiting.");
                std::process::exit(0);
            }
            _ => eprintln!("Invalid Choice."),
        }
    }
}

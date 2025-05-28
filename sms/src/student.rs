use std::io::{self, Write};
use std::cell::RefCell;

use crate::extra::NO_OF_COLUMNS;
use crate::extra::WIDTHS;

pub struct Student {
    pub id: i32,
    pub name: String,
    pub marks: f32,
}

fn update_widths(student: &Student, widths: &RefCell<[usize; NO_OF_COLUMNS]>) {

    let mut len: f64;
    let mut new_widths: [usize; NO_OF_COLUMNS] = *widths.borrow();

    len = (student.id as f64).log10();
    if len > new_widths[0] as f64 {
        new_widths[0] = len as usize + 1 as usize;
    }

    len = student.name.len() as f64;
    if len > new_widths[1] as f64 {
        new_widths[1] = len as usize + 1 as usize;
    }
    
    len = (student.marks as f64).log10() + 3 as f64;
    if len > new_widths[2] as f64 {
        new_widths[2] = len as usize + 1 as usize;
    }

}

pub fn format_entry(student: &Student) -> String {

    let widths: [usize; NO_OF_COLUMNS] = WIDTHS.with(|w| *w.borrow());

    format!("{:<x$} | {:<y$} | {:<z$}\n", 
        student.id,
        student.name,
        student.marks,
        x = widths[0],
        y = widths[1],
        z = widths[2]
    )
}

pub fn display_students(students: &[Student]) {
    for student in students {
        println!("{}, {}, {}", student.id, student.name, student.marks);
    }
}

fn search_by_id(students: &[Student]) -> Result<usize, String> {
    let id: i32 = read_id()?;

    students.iter()
        .enumerate()
        .find(|(_, s)| s.id == id)
        .map(|(i, _)| i)
        .ok_or_else(|| "Student not found.".to_string())
}



fn search_by_name(students: &[Student]) -> Result<usize, String> {
    let name: String = read_name()?;

    students.iter()
        .enumerate()
        .find(|(_, s)| s.name == name)
        .map(|(i, _)| i)
        .ok_or_else(|| "Student not found.".to_string())
}

pub fn search_student(students: &[Student]) {

    print!("Search By\n1. ID\n2. Name\n>: ");
    io::stdout().flush().unwrap();

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let index: usize = match input.chars().next() {

        Some('1') => match search_by_id(students) {
            Ok(i) => i,
            Err(e) => {
                eprintln!("{}\nCould not complete search.", e);
                return;
            }
        },

        Some('2') => match search_by_name(students) {
            Ok(i) => i,
            Err(e) => {
                eprintln!("{}\nCould not complete search.", e);
                return;
            }
        },

        _ => {
            eprintln!("Invalid Choice.");
            return;
        }
    };

    display_students(&students[index..(index + 1 as usize)]);
}

fn read_id() -> Result<i32, String> { 

    print!("Enter ID: ");
    io::stdout().flush().unwrap();

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim()
        .parse::<i32>()
        .map_err(|_| String::from("Error: Invalid ID provided."))
}

fn read_name() -> Result<String, String> {

    print!("Enter Name: ");
    io::stdout().flush().unwrap();

    let mut name: String = String::new();
    io::stdin().read_line(&mut name).unwrap();

    name = name.trim().to_string();

    if name.chars().all(|c| c.is_alphabetic() || c == '\n' || c == ' ') {
        Ok(name)
    } else {
        Err("Error: Invalid Name provided.".to_string())
    }
}

fn read_marks() -> Result<f32, String> {

    let error: &str = "Error: Invalid Marks provided.";

    print!("Enter Marks: ");
    io::stdout().flush().unwrap();
    
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let marks: f32 = input.trim().parse().expect(error);

    if 0.0 <= marks && marks <= 100.0 {
        Ok(marks)
    } else {
        Err(error.to_string())
    }
}

fn read_student() -> Result<Student, String> {

    let id: i32 = read_id()?;
    let name: String = read_name()?;
    let marks: f32 = read_marks()?;

    let new_student: Student = Student {
        id,
        name,
        marks
    };

    Ok(new_student)
}

pub fn add_student(students: &mut Vec<Student>) {

    match read_student() {
        Ok(new_student) => {
            WIDTHS.with(|w| update_widths(&new_student, w));

            students.push(new_student);
            
            students.sort_by_key(|s| s.id);

        },

        Err(err) => {
           eprintln!("{}\nStudent record could not be added.", err);
           return;
        }
    }

}

pub fn delete_student(students: &mut Vec<Student>) {
    
    match search_by_id(students) {
        Ok(i) => {
            students.remove(i);
            students.sort_by_key(|s| s.id);
        },

        Err(err) => {
            eprintln!("{}\nCould not complete Deletion.", err);
            return;
        }
    }

}

pub fn update_student(students: &mut Vec<Student>) { 

    let index: usize = match search_by_id(students) {
        Ok(i) => i,
        Err(err) => {
            eprintln!("{}\nCould not complete updating.", err);
            return;
        }
    };

    println!("Current Details: ");
    display_students(&students[index..(index + 1 as usize)]);

    println!("Enter New Details: ");
    let name: String = match read_name() {
        Ok(name) => name,
        Err(err) => {
            eprintln!("{}\nCould not complete updating.", err);
            return;
        }
    };

    let marks: f32 = match read_marks() {
        Ok(n) => n,
        Err(err) => {
            eprintln!("{}\nCould not complete updating", err);
            return;
        }
    };

    students[index].name = name;
    students[index].marks = marks;

    WIDTHS.with(|w| update_widths(&students[index], w));
}


use std::io::{self, Write};

static mut COUNT: i32 = 0;

pub struct Student {
    id: i32,
    name: String,
    marks: f32,
}

fn update_widths(student: &Student) {}

pub fn format_entry(student: &Student) -> String {}

pub fn display_students(students: &[Student]) {
    for student in students {
        println!("{}, {}, {}", student.id, student.name, student.marks);
    }
}

fn search_by_id(students: &[Student]) -> Result<usize, String> {
    let id: i32 = read_id()?;

    students.iter().enumerate().find(|(_, s)| s.id == id).map(|(i, _)| i).ok_or_else(|| "Student not found.".to_string())
}



fn search_by_name(students: &[Student]) -> Result<usize, String> {
    let name: String = read_name()?;

    students.iter().enumerate().find(|(_, s)| s.name == name).map(|(i, _)| i).ok_or_else(|| "Student not found.".to_string())
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

    name = name.trim();

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

    let new_student: Student = match read_student() {
        Ok(std) => std,
        Err(err) => {
           eprintln!("{}\nStudent record could not be added.", err);
           return;
        },
    };

    students.push(new_student);

    unsafe {
        COUNT += 1;
    }

    students.sort_by_key(|s| s.id);
}

pub fn delete_student(students: &mut Vec<Student>) {
    
    match search_by_id(students) {
        Ok(i) => {
            students.remove(i);
            unsafe {
                COUNT -= 1;
            }
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
}


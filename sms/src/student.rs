static mut COUNT: i32 = 0;

struct Student {
    id: i32,
    name: String,
    marks: f64
}

fn update_widths(student: &Student) {}

pub fn format_entry(student: &Student) -> String {}

// compare_by_id?

fn read_name() -> String {}

fn read_student() -> Student {}

pub fn add_students(students: &Student) {}

pub fn del_students(students: &Student) {}

pub fn update_student(students: &Student) {}

fn search_by_id(students: &Student, id: i32) -> i32{}

fn search_by_name(students: &Student, name: &str) -> i32 {}

pub fn search_student(students: &Student) {}

fn display_students(students: &Student, student_count: i32) {}

pub fn display_all() {}


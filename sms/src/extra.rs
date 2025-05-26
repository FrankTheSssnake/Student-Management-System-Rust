use std::fs::File;


const NO_OF_COLUMNS: usize = 3;

pub static mut WIDTHS: [i32; NO_OF_COLUMNS] = [5, 5, 5];
static HEADERS: [&str; NO_OF_COLUMNS] = ["ID", "Name", "Marks"];


// TODO

fn get_pattern() -> String {}

pub fn calculate_widths(header: &String) {}

fn format_header() -> String {}

fn format_seperator() -> String {}

pub fn fprint_head(file: &mut File) {}

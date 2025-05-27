use std::fs::File;
use std::cell::RefCell;

pub const NO_OF_COLUMNS: usize = 3;

static HEADERS: [&str; NO_OF_COLUMNS] = ["ID", "Name", "Marks"];

thread_local! {
    pub static WIDTHS: RefCell<[usize; NO_OF_COLUMNS]> = RefCell::new([5, 5, 5]);
}


// TODO

fn get_pattern() -> String {}

pub fn calculate_widths(header: &String) {}

fn format_header() -> String {}

fn format_seperator() -> String {}

pub fn fprint_head(file: &mut File) {}

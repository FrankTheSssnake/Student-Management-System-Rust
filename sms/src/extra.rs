use std::io::Write;
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

fn format_header(widths: &[usize; NO_OF_COLUMNS]) -> String {
    
    let mut output: String = String::new();

    for i in 0..NO_OF_COLUMNS {
        output.push_str(format!("{:<width$} | ", HEADERS[i], width = widths[i]).as_str());
    }

    output.truncate(output.len() - 3);

    output
}

fn format_seperator(widths: &[usize; NO_OF_COLUMNS]) -> String {

    let mut total_witdh: i32 = widths.iter().sum::<usize>() as i32;
    total_witdh += 3 * (NO_OF_COLUMNS as i32 - 1);

    "-".repeat(total_witdh as usize)
}

pub fn fprint_head(file: &mut File) {

    let widths: [usize; NO_OF_COLUMNS] = WIDTHS.with(|w| *w.borrow());

    let header: String = format_header(&widths);
    let seperator: String = format_seperator(&widths);

    write!(file, "{}\n{}\n", header, seperator);

}

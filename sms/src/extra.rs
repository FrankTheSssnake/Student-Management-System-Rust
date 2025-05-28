use std::io::Write;
use std::fs::File;
use std::cell::RefCell;
use regex::Regex;


pub const NO_OF_COLUMNS: usize = 3;

static HEADERS: [&str; NO_OF_COLUMNS] = ["ID", "Name", "Marks"];

thread_local! {
    pub static WIDTHS: RefCell<[usize; NO_OF_COLUMNS]> = RefCell::new([5, 5, 5]);
}


fn get_pattern() -> String {
    
    let mut pattern: String = String::from("^");

    for header in &HEADERS {
        pattern.push_str(header);
        pattern.push_str("(\\s*)\\| ");
    }

    pattern.truncate(pattern.len() - 3);
    pattern.push_str("$");

    pattern
}

pub fn calculate_widths(header: &str) {
    let pattern: String = get_pattern();

    let re = Regex::new(pattern.as_str()).expect("Internal Error: Could not compile regex.\n");

    let captures = re.captures(header).expect("Internal Error: Regex failed to match header.");

    WIDTHS.with(|widths| {
        let mut widths = widths.borrow_mut();

        for i in 0..NO_OF_COLUMNS {
            if let Some(m) = captures.get(i + 1) {
                widths[i] = m.end() - m.start() + HEADERS[i].len() - 1;
            }
        }
    });
}

fn format_header(widths: &[usize; NO_OF_COLUMNS]) -> String {
    
    let mut output: String = String::new();

    for i in 0..NO_OF_COLUMNS {
        output.push_str(format!("{:<width$} | ", HEADERS[i], width = widths[i]).as_str());
    }

    output.truncate(output.len() - 2);

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

    write!(file, "{}\n{}\n", header, seperator).expect("Failed to write to file.");

}

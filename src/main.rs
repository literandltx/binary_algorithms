use std::env;

fn main() {
    match env::args().nth(1).as_deref() {
        Some("lab1") => lab1::cli::run(),
        Some("lab2") => lab2::cli::run().unwrap(),
        Some("lab3") => lab3::cli::run(),
        Some("lab4") => lab4::cli::run(),
        Some("lab5") => lab5::cli::run(),
        _ => {
            eprintln!("Usage: cargo run <lab1|lab2|lab3|lab4|lab5>");
            eprintln!("Examples:");
            eprintln!("  cargo run lab1");
            eprintln!("  cargo run lab2 [--keep [<filename>]]");
        }
    }
}

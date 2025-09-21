use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};

use crate::base64::encode;

pub fn encode_file(input_file: &str, output_file: &str) -> io::Result<()> {
    let input: File = File::open(input_file)?;
    let mut reader: BufReader<File> = BufReader::new(input);

    let output: File = File::create(output_file)?;
    let mut writer: BufWriter<File> = BufWriter::new(output);

    let mut buffer: Vec<u8> = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let encoded: String = encode(&buffer);

    for chunk in encoded.as_bytes().chunks(76) {
        writer.write_all(chunk)?;
        writer.write_all(b"\n")?;
    }

    Ok(())
}

fn read_file_args_encode() -> Option<(String, String)> {
    print!("Enter: input_file [output_file]: ");
    io::stdout().flush().unwrap();

    let mut line: String = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let parts: Vec<&str> = line.trim().split_whitespace().collect();

    if parts.is_empty() {
        println!("No input file provided!");
        return None;
    }

    let input_file: String = parts[0].to_string();
    let output_file: String = if parts.len() > 1 {
        parts[1].to_string()
    } else {
        format!("{}.base64", input_file)
    };

    Some((input_file, output_file))
}

fn handle_encode() {
    if let Some((input_file, output_file)) = read_file_args_encode() {
        println!("Encoding from {} -> {}", input_file, output_file);

        if let Err(e) = encode_file(&input_file, &output_file) {
            eprintln!("Encoding failed: {}", e);
        } else {
            println!("Done.");
        }
    }
}

fn handle_decode() {
    println!("Not implemented yet!");
}

pub(crate) fn run() {
    loop {
        println!("Options:");
        println!("1. Encode");
        println!("2. Decode");
        print!("Choose option (or q to quit): ");
        io::stdout().flush().unwrap();

        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input: &str = input.trim();

        match input {
            "1" => handle_encode(),
            "2" => handle_decode(),
            "q" | "Q" => {
                println!("Exit!");
                break;
            }
            _ => {
                println!("Invalid option, try again.");
            }
        }
    }
}

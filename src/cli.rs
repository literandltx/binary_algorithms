use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};

use crate::base64::{encode, decode, BASE64_ALPHABET};

fn read_file_args() -> Option<(String, String)> {
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

pub fn encode_file(input_file: &str, output_file: &str) -> io::Result<usize> {
    let input: File = File::open(input_file)?;
    let mut reader: BufReader<File> = BufReader::new(input);

    let output: File = File::create(output_file)?;
    let mut writer: BufWriter<File> = BufWriter::new(output);

    let mut buffer: Vec<u8> = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let encoded: String = encode(&buffer);

    // metadata
    writer.write_all(b"_ encoded with Base64, max line length 76\n\n")?;

    for chunk in encoded.as_bytes().chunks(76) {
        writer.write_all(chunk)?;
        writer.write_all(b"\n")?;
    }

    Ok(encoded.len())
}

fn handle_encode() {
    if let Some((input_file, output_file)) = read_file_args() {
        println!("Encoding from {} -> {}", input_file, output_file);

        match encode_file(&input_file, &output_file) {
            Ok(written) => {
                println!("Done. Written {} bytes to {}\n", written, output_file);
            }
            Err(e) => {
                eprintln!("Encoding failed: {}", e);
            }
        }
    }
}

fn handle_decode() {
    if let Some((input_file, output_file)) = read_file_args() {
        println!("Decoding from {} -> {}", input_file, output_file);

        let output_file = if output_file.is_empty() {
            if input_file.ends_with(".base64") {
                input_file.trim_end_matches(".base64").to_string()
            } else {
                "decoded.bin".to_string()
            }
        } else {
            output_file
        };

        let content = match std::fs::read_to_string(&input_file) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to read {}: {}", input_file, e);
                return;
            }
        };

        let mut decoded: Vec<u8> = Vec::new();
        let mut ended: bool = false;

        for (line_num, line) in content.lines().enumerate() {
            let line_no: usize = line_num + 1;
            let line: &str = line.trim();

            if line.starts_with('_') {
                continue;
            }

            if ended {
                eprintln!(
                    "Warning (line {}): Data found after end of message",
                    line_no
                );
                break;
            }

            if line.len() != 76 && !line.contains('=') {
                eprintln!(
                    "Error (line {}): Incorrect line length {}",
                    line_no,
                    line.len()
                );
                return;
            }

            for (pos, c) in line.chars().enumerate() {
                if c == '=' {
                    ended = true;
                    continue;
                }
                if !BASE64_ALPHABET.contains(&(c as u8)) {
                    eprintln!(
                        "Error (line {}, symbol {}): Invalid character '{}'",
                        line_no,
                        pos + 1,
                        c
                    );
                    return;
                }
            }

            match decode(line) {
                Ok(mut bytes) => decoded.append(&mut bytes),
                Err(err) => {
                    eprintln!("Error (line {}): {}", line_no, err);
                    return;
                }
            }
        }

        if let Err(e) = std::fs::write(&output_file, &decoded) {
            eprintln!("Failed to write {}: {}", output_file, e);
        } else {
            println!("Done. Written {} bytes to {} \n", decoded.len(), output_file);
        }
    }
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

use std::fs::{create_dir_all, remove_file};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::bit_stream::{BitStreamReader, BitStreamWriter};

fn tmp_path() -> std::io::Result<PathBuf> {
    let ts: u128 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let crate_root: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let assets: PathBuf = crate_root.join("assets");
    create_dir_all(&assets)?;

    Ok(assets.join(format!("lab2_demo_{}.bin", ts)))
}

pub fn run() -> std::io::Result<()> {
    println!("lab2 demo");
    let keep: bool = std::env::args().any(|a: String| a == "--keep");
    let path: PathBuf = tmp_path()?;

    let mut writer: BitStreamWriter = BitStreamWriter::create(&path)?;
    let a1: [u8; 2] = [0xE1, 0x01];
    let a2: [u8; 2] = [0xEE, 0x00];
    writer.write_bit_sequence(&a1, 9)?;
    writer.write_bit_sequence(&a2, 9)?;
    writer.finish()?;

    let (b1, b2): (Vec<u8>, Vec<u8>) = {
        let mut reader: BitStreamReader = BitStreamReader::open(&path)?;
        (reader.read_bit_sequence(11)?, reader.read_bit_sequence(7)?)
    };

    println!("file: {}", path.display());
    println!("b1 = {:02X?}", b1);
    println!("b2 = {:02X?}", b2);

    if keep {
        if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
            println!("Kept: labs/lab2/assets/{name}");
        }
    } else {
        let _ = remove_file(&path);
    }

    Ok(())
}

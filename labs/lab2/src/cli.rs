use std::fs::{create_dir_all, remove_file};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::bit_stream_reader::BitStreamReader;
use crate::bit_stream_writer::BitStreamWriter;

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
    let data1: [u8; 2] = [0xE1, 0x01];
    let data2: [u8; 2] = [0xEE, 0x00];
    writer.write_bit_sequence(&data1, 9)?;
    writer.write_bit_sequence(&data2, 9)?;
    writer.finish()?;

    let (bytes1, bytes2, bytes): (Vec<u8>, Vec<u8>, Vec<u8>) = {
        let mut reader: BitStreamReader = BitStreamReader::open(&path)?;
        (
            reader.read_bit_sequence(11)?,
            reader.read_bit_sequence(7)?,
            reader.read_all()?,
        )
    };

    println!("file: {}", path.display());
    println!("bytes1 = {:02X?}", bytes1);
    println!("bytes2 = {:02X?}", bytes2);
    println!("bytes  = {:02X?}", bytes);

    if keep {
        if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
            println!("Kept: labs/lab2/assets/{name}");
        }
    } else {
        let _ = remove_file(&path);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use lab2::bit_stream_reader::BitStreamReader;
    use lab2::bit_stream_writer::BitStreamWriter;

    use std::fs;

    const EXAMPLE_FILE: &str = "bit_stream_example.bin";
    const EMPTY_FILE: &str = "bit_stream_empty.bin";
    const BYTES_FILE: &str = "bit_stream_bytes.bin";

    #[test]
    fn example_from_task() {
        let a1: [u8; 2] = [0xE1, 0x01];
        let a2: [u8; 2] = [0xEE, 0x00];
        let mut writer = BitStreamWriter::create(EXAMPLE_FILE).unwrap();
        writer.write_bit_sequence(&a1, 9).unwrap();
        writer.write_bit_sequence(&a2, 9).unwrap();

        let mut reader = BitStreamReader::open(EXAMPLE_FILE).unwrap();
        assert_eq!(reader.read_bit_sequence(11).unwrap(), [0xE1, 0x05]);
        assert_eq!(reader.read_bit_sequence(7).unwrap(), [0x3B]);

        cleanup(EXAMPLE_FILE);
    }

    #[test]
    fn empty_writes_produce_no_output() {
        let a1: [u8; 2] = [0xE1, 0x01];
        let a2: [u8; 2] = [0xEE, 0x00];
        let mut writer = BitStreamWriter::create(EMPTY_FILE).unwrap();
        writer.write_bit_sequence(&a1, 0).unwrap();
        writer.write_bit_sequence(&a2, 0).unwrap();

        let mut reader = BitStreamReader::open(EMPTY_FILE).unwrap();
        let out: Vec<u8> = reader.read_bit_sequence(64).unwrap();
        assert!(out.is_empty());

        cleanup(EMPTY_FILE);
    }

    #[test]
    fn reading_full_bytes_is_exact() {
        let a1: [u8; 2] = [0xE1, 0x01];
        let a2: [u8; 2] = [0xEE, 0x77];
        let mut writer = BitStreamWriter::create(BYTES_FILE).unwrap();
        writer.write_bit_sequence(&a1, 8).unwrap();
        writer.write_bit_sequence(&a2, 16).unwrap();

        let mut reader = BitStreamReader::open(BYTES_FILE).unwrap();
        assert_eq!(reader.read_bit_sequence(16).unwrap(), [0xE1, 0xEE]);
        assert_eq!(reader.read_bit_sequence(8).unwrap(), [0x77]);

        cleanup(BYTES_FILE);
    }

    fn cleanup(path: &str) {
        let _ = fs::remove_file(path);
    }
}

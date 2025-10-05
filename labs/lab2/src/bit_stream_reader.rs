use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

#[derive(Debug)]
pub struct BitStreamReader {
    file: File,
    bit_len: u64,
    read_position: u64,
}

impl BitStreamReader {
    pub fn open(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let file: File = File::open(path)?;
        let bit_len: u64 = file.metadata()?.len().saturating_mul(8);
        Ok(Self {
            file,
            bit_len,
            read_position: 0,
        })
    }

    pub fn read_all(&mut self) -> std::io::Result<Vec<u8>> {
        self.read_bit_sequence(self.bit_len)
    }

    pub fn read_bit_sequence(&mut self, bits: u64) -> std::io::Result<Vec<u8>> {
        let available: u64 = self.bit_len.saturating_sub(self.read_position);
        let to_read: usize = bits.min(available) as usize;
        if to_read == 0 {
            return Ok(Vec::new());
        }

        let start_byte: u64 = self.read_position / 8;
        let start_bit_offset: usize = (self.read_position % 8) as usize;
        let end_bit: u64 = self.read_position + to_read as u64;
        let end_byte: u64 = (end_bit + 7) / 8;
        let read_len: usize = (end_byte - start_byte) as usize;

        let mut buf: Vec<u8> = vec![0u8; read_len];
        self.file.seek(SeekFrom::Start(start_byte))?;
        self.file.read_exact(&mut buf)?;

        let mut out: Vec<u8> = vec![0u8; (to_read + 7) / 8];
        for i in 0..to_read {
            let bit_idx: usize = start_bit_offset + i;
            let b: u8 = (buf[bit_idx / 8] >> (bit_idx % 8)) & 1;
            if b != 0 {
                out[i / 8] |= 1u8 << (i % 8);
            }
        }

        self.read_position += to_read as u64;
        Ok(out)
    }

    pub fn get_bit_len(&self) -> u64 {
        self.bit_len
    }
}

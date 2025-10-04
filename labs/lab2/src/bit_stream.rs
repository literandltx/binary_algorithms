use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

#[derive(Debug)]
pub struct BitStreamWriter {
    file: File,
    byte_len: u64,
    current_byte: u8,
    bit_count: u8,
}

impl BitStreamWriter {
    pub fn create(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let file: File = File::create(path)?;
        Ok(Self {
            file,
            byte_len: 0,
            current_byte: 0,
            bit_count: 0,
        })
    }

    pub fn write_bit_sequence(&mut self, data: &[u8], bits: usize) -> std::io::Result<()> {
        let to_take: usize = bits.min(data.len() * 8);

        let had_partial_before: bool = self.bit_count > 0;
        if had_partial_before && self.byte_len > 0 {
            self.file
                .seek(SeekFrom::Start(self.byte_len.saturating_sub(1)))?;
        } else {
            self.file.seek(SeekFrom::Start(self.byte_len))?;
        }

        let mut overwrite_first_full: bool = had_partial_before;

        for i in 0..to_take {
            let src_byte: usize = i / 8;
            let src_bit: usize = i % 8;
            let bit: u8 = (data[src_byte] >> src_bit) & 1;

            if bit != 0 {
                self.current_byte |= 1 << self.bit_count;
            }

            self.bit_count += 1;

            if self.bit_count == 8 {
                let one: [u8; 1] = [self.current_byte];
                if overwrite_first_full {
                    self.file.write_all(&one)?;
                    overwrite_first_full = false;
                } else {
                    self.file.write_all(&one)?;
                    self.byte_len += 1;
                }
                self.current_byte = 0;
                self.bit_count = 0;
            }
        }

        if self.bit_count > 0 {
            let one: [u8; 1] = [self.current_byte];
            if overwrite_first_full {
                self.file.write_all(&one)?;
            } else {
                self.file.write_all(&one)?;
                self.byte_len += 1;
            }
        }

        Ok(())
    }

    pub fn finish(&mut self) -> std::io::Result<()> {
        self.file.flush()
    }
}

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

    pub fn read_bit_sequence(&mut self, bits: usize) -> std::io::Result<Vec<u8>> {
        let available: u64 = self.bit_len.saturating_sub(self.read_position);
        let to_read: usize = (bits as u64).min(available) as usize;
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
}

use std::fs::File;
use std::io::{Seek, SeekFrom, Write};
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

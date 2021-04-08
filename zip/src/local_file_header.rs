use std::fs::File;
use std::io::Read;
use std::io::Result;

#[derive(Debug)]
pub struct LocalFileHeader {
    pub min_version: u16,
    pub bit_flags: LocalFileHeaderBitFlags,
    pub compression_method: u16,
    pub last_modification_time: u16,
    pub last_modification_date: u16,
    pub crc32: u32,
    pub compressed_size: u32,
    pub uncomporessed_size: u32,
    pub file_name_length: u16,
    pub extra_field_length: u16,
    pub file_name: String,
}
impl LocalFileHeader {
    pub fn new(file: &mut File) -> Result<Self> {
        let mut local_file_header_buffer = [0; 26];
        file.read(&mut local_file_header_buffer)?;

        let bit_flags = LocalFileHeaderBitFlags {
            flags: to_u16(&local_file_header_buffer, 2),
        };
        if bit_flags.is_encrypted() {
            panic!("This file is encrypted");
        }

        let file_name_length = to_u16(&local_file_header_buffer, 22);
        let mut file_name_buffer = vec![0u8; file_name_length as usize];
        file.read(&mut file_name_buffer)?;
        let file_name = String::from_utf8(
            file_name_buffer
                .iter()
                .map(|character| *character)
                .collect(),
        )
        .unwrap();

        let extra_field_length = to_u16(&local_file_header_buffer, 24);
        // Ignore since not yet needed.
        let mut extra_data_buffer = vec![0u8; extra_field_length as usize];
        file.read(&mut extra_data_buffer)?;

        Ok(LocalFileHeader {
            min_version: to_u16(&local_file_header_buffer, 0),
            bit_flags,
            compression_method: to_u16(&local_file_header_buffer, 4),
            last_modification_time: to_u16(&local_file_header_buffer, 6),
            last_modification_date: to_u16(&local_file_header_buffer, 8),
            crc32: to_u32(&local_file_header_buffer, 10),
            compressed_size: to_u32(&local_file_header_buffer, 14),
            uncomporessed_size: to_u32(&local_file_header_buffer, 18),
            file_name_length,
            extra_field_length,
            file_name,
        })
    }
}

#[derive(Debug)]
pub struct LocalFileHeaderBitFlags {
    pub flags: u16,
}
impl LocalFileHeaderBitFlags {
    pub fn is_encrypted(&self) -> bool {
        return self.flags & 0b0000_0000_0000_0001 != 0;
    }
}

#[inline(always)]
fn to_u16(buffer: &[u8; 26], index: usize) -> u16 {
    return u16::from_le_bytes([buffer[index], buffer[index + 1]]);
}

#[inline(always)]
fn to_u32(buffer: &[u8; 26], index: usize) -> u32 {
    return u32::from_le_bytes([
        buffer[index],
        buffer[index + 1],
        buffer[index + 2],
        buffer[index + 3],
    ]);
}

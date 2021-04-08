use std::fs::File;
use std::io::Read;
use std::io::Result;

#[derive(Debug)]
pub struct CentralDirectoryFileHeader {
    pub version_made_by: u16,
    pub min_version: u16,
    pub bit_flags: CentralDirectoryFileHeaderBitFlags,
    pub compression_method: u16,
    pub last_modification_time: u16,
    pub last_modification_date: u16,
    pub crc32: u32,
    pub compressed_size: u32,
    pub uncomporessed_size: u32,
    pub file_name_length: u16,
    pub extra_field_length: u16,
    pub file_comment_length: u16,
    pub disk_number: u16,
    pub internal_file_attributes: u16,
    pub external_file_attributes: u32,
    pub relative_offset_to_local_file_header: u32,
    pub file_name: String,
    pub file_comment: String,
}
impl CentralDirectoryFileHeader {
    pub fn new(file: &mut File) -> Result<Self> {
        let mut central_directory_header = [0; 42];
        file.read(&mut central_directory_header)?;

        let bit_flags = CentralDirectoryFileHeaderBitFlags {
            flags: to_u16(&central_directory_header, 4),
        };
        if bit_flags.is_encrypted() {
            panic!("This file is encrypted");
        }

        let file_name_length = to_u16(&central_directory_header, 24);
        let mut file_name_buffer = vec![0u8; file_name_length as usize];
        file.read(&mut file_name_buffer)?;
        let file_name = String::from_utf8(
            file_name_buffer
                .iter()
                .map(|character| *character)
                .collect(),
        )
        .unwrap();

        let file_comment_length = to_u16(&central_directory_header, 28);
        let mut file_comment_buffer = vec![0u8; file_comment_length as usize];
        file.read(&mut file_comment_buffer)?;
        let file_comment = String::from_utf8(
            file_comment_buffer
                .iter()
                .map(|character| *character)
                .collect(),
        )
        .unwrap();

        let extra_field_length = to_u16(&central_directory_header, 26);
        // Ignore since not yet needed.
        let mut extra_data_buffer = vec![0u8; extra_field_length as usize];
        file.read(&mut extra_data_buffer)?;

        Ok(CentralDirectoryFileHeader {
            version_made_by: to_u16(&central_directory_header, 0),
            min_version: to_u16(&central_directory_header, 2),
            bit_flags,
            compression_method: to_u16(&central_directory_header, 6),
            last_modification_time: to_u16(&central_directory_header, 8),
            last_modification_date: to_u16(&central_directory_header, 10),
            crc32: to_u32(&central_directory_header, 12),
            compressed_size: to_u32(&central_directory_header, 16),
            uncomporessed_size: to_u32(&central_directory_header, 20),
            file_name_length,
            extra_field_length,
            file_comment_length,
            disk_number: to_u16(&central_directory_header, 30),
            internal_file_attributes: to_u16(&central_directory_header, 32),
            external_file_attributes: to_u32(&central_directory_header, 34),
            relative_offset_to_local_file_header: to_u32(&central_directory_header, 38),
            file_name,
            file_comment,
        })
    }
}

#[derive(Debug)]
pub struct CentralDirectoryFileHeaderBitFlags {
    pub flags: u16,
}
impl CentralDirectoryFileHeaderBitFlags {
    pub fn is_encrypted(&self) -> bool {
        return self.flags & 0b0000_0000_0000_0001 != 0;
    }
}

#[inline(always)]
fn to_u16(buffer: &[u8; 42], index: usize) -> u16 {
    return u16::from_le_bytes([buffer[index], buffer[index + 1]]);
}

#[inline(always)]
fn to_u32(buffer: &[u8; 42], index: usize) -> u32 {
    return u32::from_le_bytes([
        buffer[index],
        buffer[index + 1],
        buffer[index + 2],
        buffer[index + 3],
    ]);
}

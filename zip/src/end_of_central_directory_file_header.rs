use std::fs::File;
use std::io::Read;
use std::io::Result;

#[derive(Debug)]
pub struct EndOfCentralDirectoryRecord {
    pub number_of_disks: u16,
    pub disk_with_central_directory_start: u16,
    pub number_of_central_directories_in_disk: u16,
    pub total_number_of_central_directories: u16,
    pub size_of_central_directory: u32,
    pub offset_start_of_central_directory: u32,
    pub comment_length: u16,
    pub comment: String,
}
impl EndOfCentralDirectoryRecord {
    pub fn new(file: &mut File) -> Result<Self> {
        let mut local_file_header_buffer = [0; 26];
        file.read(&mut local_file_header_buffer)?;

        let comment_length = to_u16(&local_file_header_buffer, 22);
        let mut comment_buffer = vec![0u8; comment_length as usize];
        file.read(&mut comment_buffer)?;
        let comment =
            String::from_utf8(comment_buffer.iter().map(|character| *character).collect()).unwrap();

        Ok(EndOfCentralDirectoryRecord {
            number_of_disks: to_u16(&local_file_header_buffer, 0),
            disk_with_central_directory_start: to_u16(&local_file_header_buffer, 2),
            number_of_central_directories_in_disk: to_u16(&local_file_header_buffer, 4),
            total_number_of_central_directories: to_u16(&local_file_header_buffer, 6),
            size_of_central_directory: to_u32(&local_file_header_buffer, 8),
            offset_start_of_central_directory: to_u32(&local_file_header_buffer, 12),
            comment_length,
            comment,
        })
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

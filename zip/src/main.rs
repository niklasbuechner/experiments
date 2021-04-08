pub(crate) mod central_directory_file_header;
pub(crate) mod end_of_central_directory_file_header;
pub(crate) mod local_file_header;

use central_directory_file_header::CentralDirectoryFileHeader;
use end_of_central_directory_file_header::EndOfCentralDirectoryRecord;
use local_file_header::LocalFileHeader;
use std::fs::File;
use std::io::Read;
use std::io::Result;

fn main() -> Result<()> {
    println!("Listing zip contents!");
    let mut file = File::open("./test-data.zip").unwrap();

    loop {
        let mut signature = [0; 4];
        file.read(&mut signature)?;

        let signature_value = u32::from_le_bytes(signature);
        if signature_value == 0x04034b50 {
            let file_header = LocalFileHeader::new(&mut file)?;
            println!("{:#?}", file_header);

            if file_header.uncomporessed_size == 0 {
                continue;
            }

            'readContent: loop {
                let mut file_buffer = vec![0; 1];
                file.read(&mut file_buffer)?;

                println!("{}", file_buffer[0]);

                if file_buffer[0] == 0 {
                    break 'readContent;
                }
            }
        // Data descriptor
        } else if signature_value == 0x08074b50 {
            let mut data_descriptor_buffer = [0; 12];
            file.read(&mut data_descriptor_buffer)?;
            println!(
                "crc32: {}, compressed size: {}, uncomporessed size: {}",
                to_u32(&data_descriptor_buffer, 0),
                to_u32(&data_descriptor_buffer, 4),
                to_u32(&data_descriptor_buffer, 8)
            );
        } else if signature_value == 0x02014b50 {
            let central_directory_header = CentralDirectoryFileHeader::new(&mut file)?;
            println!("{:#?}", central_directory_header);
        } else if signature_value == 0x06054b50 {
            let end_of_central_directory_file_header = EndOfCentralDirectoryRecord::new(&mut file)?;
            println!("{:#?}", end_of_central_directory_file_header);
        } else {
            println!("Signature value {:x}", signature_value);
            break;
        }
    }

    Ok(())
}

#[inline(always)]
fn to_u32(buffer: &[u8; 12], index: usize) -> u32 {
    return u32::from_le_bytes([
        buffer[index],
        buffer[index + 1],
        buffer[index + 2],
        buffer[index + 3],
    ]);
}

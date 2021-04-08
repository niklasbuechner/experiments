pub(crate) mod local_file_header;

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
        if signature_value != 0x04034b50 {
            if signature_value == 0x02014b50 {
                println!("Central Directory File Header");
            } else if signature_value == 0x06054b50 {
                println!("End of Central Directory Record");
            } else if signature_value == 0x08074b50 {
                println!("Data description record");
            }

            println!("Signature value {:x}", signature_value);
            break;
        }

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

        // Remove next signature
        let mut signature = [0; 4];
        file.read(&mut signature)?;

        let mut data_descriptor_buffer = [0; 12];
        file.read(&mut data_descriptor_buffer)?;
        println!(
            "crc32: {}, compressed size: {}, uncomporessed size: {}",
            to_u32(&data_descriptor_buffer, 0),
            to_u32(&data_descriptor_buffer, 4),
            to_u32(&data_descriptor_buffer, 8)
        );
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

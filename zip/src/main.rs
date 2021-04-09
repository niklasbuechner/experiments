pub(crate) mod central_directory_file_header;
pub(crate) mod end_of_central_directory_file_header;
pub(crate) mod local_file_header;

use central_directory_file_header::CentralDirectoryFileHeader;
use end_of_central_directory_file_header::EndOfCentralDirectoryRecord;
// use local_file_header::LocalFileHeader;
use std::fs::File;
use std::io::Read;
use std::io::Result;
use std::io::Seek;
use std::io::SeekFrom;
use std::process::exit;

fn main() -> Result<()> {
    println!("Listing zip contents!");
    let mut file = File::open("./test-data.zip").unwrap();
    let metadata = file.metadata().unwrap();

    if metadata.is_dir() {
        println!("The provided file is a directory and therefore can't be opened.");
        exit(1);
    }

    let size = metadata.len();
    file.seek(SeekFrom::Start(size - 22))?;

    let mut signature = [0; 4];
    file.read(&mut signature)?;
    let signature_value = u32::from_le_bytes(signature);
    let end_of_central_directory_file_header;
    if signature_value == 0x06054b50 {
        end_of_central_directory_file_header = EndOfCentralDirectoryRecord::new(&mut file)?;
    } else {
        println!("Zip file does not end with an end of central directory entry or the entry contains a comment. {:X}", signature_value);
        exit(1);
    }

    // Read central directory of files.
    file.seek(SeekFrom::Start(
        end_of_central_directory_file_header.offset_start_of_central_directory as u64,
    ))?;
    let mut files: Vec<CentralDirectoryFileHeader> = Vec::new();
    loop {
        file.read(&mut signature)?;
        let signature_value = u32::from_le_bytes(signature);

        if signature_value == 0x02014b50 {
            let file = CentralDirectoryFileHeader::new(&mut file)?;
            files.push(file);
        } else if signature_value == 0x06054b50 {
            // We have read all entry records, break.
            break;
        } else {
            println!(
                "Expected a central directory file header, unexpected signature received: {:X}",
                signature_value
            );
            exit(1);
        }
    }

    println!("Files:");
    files.iter().for_each(|file| {
        println!(
            " - {} (Compression mode: {} - saved {} bytes)",
            file.file_name,
            file.compression_method,
            file.uncomporessed_size as i64 - file.compressed_size as i64
        );
    });

    Ok(())
}

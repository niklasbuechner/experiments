pub(crate) mod local_file_header;

use local_file_header::LocalFileHeader;
use std::fs::File;
use std::io::Read;

fn main() {
    println!("Listing zip contents!");

    let mut signature = [0; 4];
    let mut file = File::open("./test-data.zip").unwrap();
    file.read(&mut signature);

    let signature_value = u32::from_le_bytes(signature);
    if signature_value == 0x04034b50 {
        println!("Is local file header");
    }

    let file_header = LocalFileHeader::new(&mut file);
    println!("{:#?}", file_header);
    // let local_file_header =
}

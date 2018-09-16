#![feature(test)]

extern crate gap;
extern crate test;

use test::Bencher;
use std::str::FromStr;

use gap::small_file_contents;
use gap::big_file_contents;

#[bench]
fn small_delete_file_beginning(b: &mut Bencher) {
    b.iter(|| {
        let mut buffer = gap::GapBuffer::from_str(small_file_contents()).unwrap();
        buffer.delete(0,0, 0,1);
    });
}

#[bench]
fn small_delete_file_middle(b: &mut Bencher) {
    b.iter(|| {
        let mut buffer = gap::GapBuffer::from_str(small_file_contents()).unwrap();
        buffer.delete(6,11, 6,15);
    });
}

#[bench]
fn small_delete_file_end(b: &mut Bencher) {
    b.iter(|| {
        let mut buffer = gap::GapBuffer::from_str(small_file_contents()).unwrap();
        buffer.delete(17,1, 18,0);
    });
}

#[bench]
fn small_delete_with_gap_resize(b: &mut Bencher) {
    b.iter(|| {
        let mut buffer = gap::GapBuffer::from_str(small_file_contents()).unwrap();
        buffer.delete(2,0, 18,0);
    });
}

#[bench]
fn big_delete_file_beginning(b: &mut Bencher) {
    b.iter(|| {
        let mut buffer = gap::GapBuffer::from_str(big_file_contents()).unwrap();
        buffer.delete(0,0, 0,1);
        buffer.delete(0,0, 0,1);
        buffer.delete(0,0, 0,1);
        buffer.delete(0,0, 0,1);
        buffer.delete(0,0, 0,1);
        buffer.delete(0,0, 0,1);
    });
}

#[bench]
fn big_delete_file_middle(b: &mut Bencher) {
    b.iter(|| {
        let mut buffer = gap::GapBuffer::from_str(big_file_contents()).unwrap();
        buffer.delete(4000,0, 4010,0);
    });
}

#[bench]
fn big_delete_file_end(b: &mut Bencher) {
    b.iter(|| {
        let mut buffer = gap::GapBuffer::from_str(big_file_contents()).unwrap();
        buffer.delete(8000,0, 8010,0);
    });
}

#[bench]
fn big_delete_with_gap_resize(b: &mut Bencher) {
    b.iter(|| {
        let mut buffer = gap::GapBuffer::from_str(big_file_contents()).unwrap();
        buffer.delete(2,0, 18,0);
    });
}


#[bench]
fn small_insert_file_beginning(b: &mut Bencher) {
    b.iter(|| {
        let mut buffer = gap::GapBuffer::from_str(small_file_contents()).unwrap();
        buffer.insert(0, 0, &mut "Heo".to_string());
        buffer.insert(0, 2, &mut "ll".to_string());
    });
}

#[bench]
fn small_insert_file_middle(b: &mut Bencher) {
    b.iter(|| {
        let mut buffer = gap::GapBuffer::from_str(small_file_contents()).unwrap();
        buffer.insert(15,18, &mut ", world".to_string());
    });
}

#[bench]
fn small_insert_file_end(b: &mut Bencher) {
    b.iter(|| {
        let mut buffer = gap::GapBuffer::from_str(small_file_contents()).unwrap();
        buffer.insert(18,0, &mut "Hello".to_string());
    });
}

#[bench]
fn small_insert_with_gap_resize(b: &mut Bencher) {
    b.iter(|| {
        let mut buffer = gap::GapBuffer::from_str(small_file_contents()).unwrap();
        buffer.insert(2,0, &mut "123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890".to_string());
    });
}

#[bench]
fn big_insert_file_beginning(b: &mut Bencher) {
    b.iter(|| {
        let mut buffer = gap::GapBuffer::from_str(big_file_contents()).unwrap();
        buffer.insert(0, 0, &mut "Heo".to_string());
        buffer.insert(0, 2, &mut "ll".to_string());
    });
}

#[bench]
fn big_insert_file_middle(b: &mut Bencher) {
    b.iter(|| {
        let mut buffer = gap::GapBuffer::from_str(big_file_contents()).unwrap();
        buffer.insert(4000,0, &mut ", world".to_string());
    });
}

#[bench]
fn big_insert_file_end(b: &mut Bencher) {
    b.iter(|| {
        let mut buffer = gap::GapBuffer::from_str(big_file_contents()).unwrap();
        buffer.insert(8000,0, &mut "Hello".to_string());
    });
}

#[bench]
fn big_insert_with_gap_resize(b: &mut Bencher) {
    b.iter(|| {
        let mut buffer = gap::GapBuffer::from_str(big_file_contents()).unwrap();
        buffer.insert(6000,0, &mut "123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890".to_string());
    });
}

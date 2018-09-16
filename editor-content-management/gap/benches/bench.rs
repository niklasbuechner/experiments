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

#[allow(dead_code)]
extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::{self, BufReader};
use std::time::Instant;

fn main() {
    if args().len() != 3 {
        eprintln!("Usage: `source` `target");
        return;
    }

    let mut _input: BufReader<File> = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let mut _output: File = File::create(args().nth(2).unwrap()).unwrap();

    let mut _encoder: GzEncoder<File> = GzEncoder::new(_output, Compression::default());

    let start: Instant = Instant::now();

    io::copy(&mut _input, &mut _encoder).unwrap();

    let _output: File = _encoder.finish().unwrap();

    println!(
        "Source Len: {:?}",
        _input.get_ref().metadata().unwrap().len()
    );

    println!("Target Len: {:?}", _output.metadata().unwrap().len());

    println!("Elapsed: {:?}", start.elapsed());
}

extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;


fn main() {
    if args().len() != 3 {
        eprintln!("Usage: `source` `target`");
        return;
    }

    let file = File::open(
        args().nth(1).unwrap()
    ).unwrap();

    let mut input = BufReader::new(
        file
    );

    let output = File::create(args().nth(2).unwrap());
    if let Ok(_) = output {
        println!("File created!!");
    } else {
        panic!("File creation failed!");
    }

    let mut encoder = GzEncoder::new(
        output.unwrap(), 
        Compression::default()
    );

    let start = Instant::now();

    // driver code 
    copy(&mut input, &mut encoder).unwrap();
    let out = encoder.finish().unwrap();

    println!(
        "Source len: {:?}",
        input.get_ref().metadata().unwrap().len()
    );

    println!(
        "Target len: {:?}",
        out.metadata().unwrap().len()
    );

    println!(
        "Elapsed : {:?}",
        start.elapsed()
    );
}

use std::{process, error::Error};

use csv::{self, Reader};


fn read_csv(fpath: &str) -> Result<(), Box<dyn Error>>{

    let mut reader = Reader::from_path(fpath)?;
    
    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}

fn main() {
    if let Err(err) = read_csv("./sample.csv") {
        eprintln!("Exit with error {}", err);
        process::exit(1);
    }
}

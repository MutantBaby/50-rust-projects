use csv::{self, Reader, StringRecord};
use std::{env, error::Error, ffi::OsString, fs::File, process};

fn run() -> Result<(), Box<dyn Error>> {
    let file_path: OsString = get_first_arg()?;
    let file: File = File::open(file_path)?;
    let mut rdr: Reader<_> = Reader::from_reader(file);

    for result in rdr.records() {
        let record: StringRecord = result?;
        println!("{:?}", record);
    }

    Ok(())
}

/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn main() {
    if let Err(err) = run() {
        println!("Error | ❤️‍🔥 {}", err);
        process::exit(1);
    }
}

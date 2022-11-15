use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn open_file(filename: String) -> io::Result<BufReader<File>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file))
}

fn count_logs(reader: &mut BufReader<File>) -> HashMap<String, (u32, u32)> {
    for line in reader.lines() {
        if let Ok(line) = line {
            println!("{line}");
        }
    }

    unimplemented!()
}

fn main() -> io::Result<()> {
    if env::args().len() != 2 {
        println!("Invalid args, usage: ./parser FILENAME");
        std::process::exit(1);
    }

    let filename = env::args().nth(1).unwrap();
    let mut reader = open_file(filename)?;

    count_logs(&mut reader);
    Ok(())
}

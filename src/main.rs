use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::Iterator;

type CounterMap = HashMap<String, (u32, u32)>;

fn open_file(filename: String) -> io::Result<BufReader<File>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file))
}

fn parse_line(line: String) -> Option<(String, String)> {
    let mut iterator = line.split_whitespace();
    if let Some(url) = iterator.next() {
        if let Some(ip) = iterator.next() {
            return Some((String::from(url), String::from(ip)));
        }
    }
    return None;
}

fn count_logs(reader: &mut BufReader<File>) -> CounterMap {
    let mut counts: HashMap<String, (u32, HashSet<String>)> = HashMap::new();

    for line in reader.lines() {
        if let Some((url, ip)) = parse_line(line.unwrap()) {
            let mut entry = counts.entry(url).or_insert((0, HashSet::new()));
            entry.0 += 1;
            entry.1.insert(ip);
        }
    }

    let entries = counts
        .iter()
        .map(|(url, (visits, set))| (url.clone(), (*visits, set.len() as u32)));

    HashMap::from_iter(entries)
}

fn sort_by_value_at_index(counts: &CounterMap, index: usize) -> Vec<(&String, u32)> {
    let mut entries: Vec<_> = counts
        .iter()
        .map(|(url, value)| {
            if index == 0 {
                return (url, value.0);
            }
            return (url, value.1);
        })
        .collect();

    entries.sort_by_key(|k| Reverse(k.1));

    entries
}

fn main() -> io::Result<()> {
    if env::args().len() != 2 {
        println!("Invalid args, usage: ./parser FILENAME");
        std::process::exit(1);
    }

    let filename = env::args().nth(1).unwrap();
    let mut reader = open_file(filename)?;
    let counts = count_logs(&mut reader);

    let sorted_by_visits = sort_by_value_at_index(&counts, 0);
    let sorted_by_unique = sort_by_value_at_index(&counts, 1);

    for (url, visits) in sorted_by_visits {
        println!("{url} {visits} visits");
    }

    for (url, unique) in sorted_by_unique {
        println!("{url} {unique} unique views");
    }

    Ok(())
}

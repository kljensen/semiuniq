extern crate lru;
use std::env;
use std::fs;
use std::io::{self, BufReader, BufRead};

use lru::LruCache;

fn main() -> std::io::Result<()> {
    let mut cache: LruCache<String, bool> = LruCache::new(5);

    let input = env::args().nth(1);
    let reader: Box<dyn BufRead> = match input {
        None => Box::new(BufReader::new(io::stdin())),
        Some(filename) => Box::new(BufReader::new(fs::File::open(filename)?))
    };
    for line in reader.lines() {
        let l = line?;
        let was_present = cache.put(l.clone(), true);
        match was_present {
            None => println!("{}", l),
            _ => (),
        }
    }
    Ok(())
}

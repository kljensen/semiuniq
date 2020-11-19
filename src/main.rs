extern crate lru;
use std::fs;
use std::io::{self, BufReader, BufRead};
use clap::{Arg, App};
use lru::LruCache;

fn main() -> std::io::Result<()> {

    // Parse the command line arguments
    let matches = App::new("semiuniq")
                          .version("0.1")
                          .about("Filters out repeated lines that are near each other in a file")
                          .arg(Arg::with_name("WINDOW_SIZE")
                               .help("The window size to use for filtering duplicates")
                               .required(true)
                               .index(1))
                          .arg(Arg::with_name("FILE_NAME")
                                .help("The file to read as input")
                               .required(false)
                               .index(2))
                          .get_matches();
    
    // The window size is labeled as required above, so the
    // "expect" will never be called here.
    let window_size_str = matches.value_of("WINDOW_SIZE").expect("no window size given");
    let window_size = window_size_str.parse::<usize>().expect("window size must be an integer");

    // Keep track of which lines we've seen using a Least Recently Used cache.
    // If we see "foo", we add it to the cache. And, if we've not seen it again
    // `window_size` lines later, "foo" is purged from the `LruCache` automatically.
    let mut seen_lines: LruCache<String, bool> = LruCache::new(window_size);

    // The input file is optional.
    let input_file = matches.value_of("FILE_NAME");

    // If we don't receive an input file, use stdin.
    let reader: Box<dyn BufRead> = match input_file {
        None => Box::new(BufReader::new(io::stdin())),
        Some(filename) => Box::new(BufReader::new(fs::File::open(filename)?))
    };

    // Iterate over all lines
    for line in reader.lines() {
        let l = line?;

        // The put method will return `None` if the line is new and
        // `true` if the line is a duplicate.
        let line_is_repeat = seen_lines.put(l.clone(), true);
        match line_is_repeat {
            None => println!("{}", l),
            _ => (),
        }
    }
    Ok(())
}

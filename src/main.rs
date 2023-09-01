use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;

use flate2::write::GzEncoder;
use flate2::Compression;

fn main() {
    if args().len() != 3 {
        eprintln!("Usage: `source_file` target_file`");
        return;
    }
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).expect("file should be found in destination specified and should be executable by the current user"));
    let output = File::create(args().nth(2).unwrap()).unwrap();

    let mut encoder = GzEncoder::new(output, Compression::default());
    copy(&mut input, &mut encoder).unwrap();
    encoder.finish().unwrap();
}

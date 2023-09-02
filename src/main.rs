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
    
    let output = encoder.finish().unwrap();
    let source_size: u64 = input.get_ref().metadata().unwrap().len();
    let target_size: u64 = output.metadata().unwrap().len();
    println!("source size: {:?} bytes", source_size);
    println!("target size: {:?} bytes", target_size);
    println!("compression ratio: {:.2}", source_size as f64 / target_size as f64);
}

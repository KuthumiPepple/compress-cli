# compress-cli
compress-cli is a command-line tool written in Rust for compressing files using the deflate algorithm. It utilizes the flate2 crate to perform compression.

## Getting Started
1. Clone the repository
2. Ensure that rust and cargo are installed

## Usage
To run the code, open a terminal or command prompt and navigate to the directory containing the source code.

Run the following command:
```
cargo run --release -- <source_file> <target_file>
```
Replace `<source_file>` with the path to the file you want to compress, and `<target_file>` with the desired path and name for the compressed file.

For example:
```
cargo run --release -- input.txt compressed.gz
```
This will compress the `input.txt` file and save the compressed output as `compressed.gz`.
It will also print the size of each file together with the compression ratio.

## Dependecies
This code relies on the following dependencies:
- flate2
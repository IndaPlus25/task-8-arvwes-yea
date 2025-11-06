use std::fs::File;
use std::io::{self, Read, Write};

mod assembler;
use assembler::assemble;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("failed to read file");

    let result = assemble(&input);
    for byte in &result {
        println!("{:08b}", byte);
    }

    let mut output = File::create("output.txt").expect("failed to write to create file");

    output.write_all(&result).expect("Failed to write output");
}

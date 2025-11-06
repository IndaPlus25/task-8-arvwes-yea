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

    let mut output = File::create("output.obexe").expect("failed to write to create file");

    output.write_all(&result).expect("Failed to write output");
}

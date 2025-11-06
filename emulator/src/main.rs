use std::{ env, fs};
use std::io;

type OperationCode = u8;
const CAL: OperationCode = 0b000; //call
const ADD: OperationCode = 0b100; //add 
const JMP: OperationCode = 0b010; //Jump distance in program
const BEQ: OperationCode = 0b001; //Branch if equal to 0
const SET: OperationCode = 0b110; //Set value
const SUB: OperationCode = 0b101; //subtract
const JB: OperationCode = 0b011; // jump backwards in program

const CALL_RM: OperationCode = 0b000;
const CALL_WM: OperationCode = 0b001;
const CALL_IM: OperationCode = 0b010;
const CALL_OM: OperationCode = 0b011;
const CALL_EX: OperationCode = 0b100;
const CALL_OR: OperationCode = 0b101;
const CALL_WR: OperationCode = 0b110;

type Registry = u8;
const R0: Registry = 0b00;
const R1: Registry = 0b10;
const R2: Registry = 0b01;
const R3: Registry = 0b11;

fn main() {
    
    let lines: Vec<u8> = fs::read(&env::args().collect::<Vec<_>>()[1]).expect("File read error");

    let mut pc: usize = 0;


    let mut register: [i32; 4] = [0;4];

    let mut pointer: [u32; 32] = [0; 32];

    let mut memory: [u8; 512] = [0; 512];
    
    let length: u32 = lines.len() as u32;

    while pc < lines.len() {
        match (lines[pc] >> 5) & 0b111 {
            CAL => call(lines[pc] & 0b11111, &mut register, &mut pc, &mut pointer, &mut memory, &length),
            ADD => add(lines[pc] & 0b11111, &mut register),
            JMP => {pc += (lines[pc] & 0b11111) as usize; continue;}
            BEQ => {beq(lines[pc] & 0b11111, &register, &mut pc); continue;}
            SET => set(lines[pc] & 0b11111, &mut register),
            SUB => sub(lines[pc] & 0b11111, &mut register),
            JB =>  {if pc > (lines[pc] & 0b11111) as usize {pc -= (lines[pc] & 0b11111) as usize;} continue;}
            _ => println!("Invalid operation on line {}!", pc),
        }

        pc += 1;
    }
}

fn call(line: u8, reg: &mut [i32; 4], pc: &mut usize, pointer: &mut [u32; 32], memory: &mut [u8; 512], length: &u32) {

    let regnum = (line & 0b11) as usize;
    let mut register = reg[regnum];

    // Kolla om register är negativ, ddå blir det fel för as usize
    match((line >> 2) & 0b111) {
        CALL_RM => reg[regnum] = memory[pointer[register as usize]] as i32,     // call for reading from memory, n:th position
        CALL_WM => write_memory(register),                      // call for storing to memory, n:th position
        CALL_IM => input_memory(register),                      // call for reading input to memory, n:th position
        CALL_OM => println!("{}", memory[pointer[register as usize]]),        // call for writing to console from memory, n:th position
        CALL_EX => *pc = length,                                // call for closing the application
        CALL_OR => println!("{}", register),                         // call for writing to console from register
        CALL_WR => input_register(regnum, &mut reg),  // call for reading input into register
        _ => println!("Invalid operation on line {}!", pc),
    }

}

fn write_memory(reg: &i32) {

}

fn input_memory(reg: &i32) {
    let input = io::stdin();
    let mut line = String::new();

    //input goes into line
    input
        .read_line(&mut line)
        .expect("Not a line");

//  count how long the string is (string1)
//  count how long the string at position pointer[reg as usize] is (string2)
//  if string1 < string2, purge string2 and put in string1
//  if string1 > string2, put string1 at the end of memory
       
    let mut i: u32 = 0;
    for character in line {


        i += 1;
    }
}

fn input_register(line: usize, reg: &mut [i32; 4]) {
    let input = io::stdin();
    let mut input_line = String::new();

    //input goes into line
    input
        .read_line(&mut input_line)
        .expect("Not a line");

    reg[line as usize] = input_line.trim().parse().expect("You did not input a number");
}



fn add(line: u8, reg: &mut [i32; 4]) {
    let r1 = (line >> 3) & 0b11;


    if line & 0b1 != 0b0 {
        let imm = line & 0b111;
        reg[r1 as usize] += imm as i32;
    } else {
        let r2 = (line >> 1) & 0b11;
        reg[r1 as usize] += reg[r2 as usize];
    }
}

fn beq(line: u8, reg: &[i32; 4], pc: &mut usize) {
    let r1 = (line >> 3) & 0b11;
    let r2 = (line >> 1) & 0b11;

    if reg[r1 as usize] == reg[r2 as usize] {
        let jump = (line & 0b1) as usize + 1;

        *pc += jump;        
    }
}

fn set(line: u8, reg: &mut [i32; 4]) {
    let r1 = (line >> 3) & 0b11;


    if line & 0b1 != 0b0 {
        let imm = line & 0b111;
        reg[r1 as usize] = imm as i32;
    } else {
        let r2 = (line >> 1) & 0b11;
        reg[r1 as usize] = reg[r2 as usize];
    }
}

fn sub(line: u8, reg: &mut [i32; 4]) {
    let r1 = (line >> 3) & 0b11;


    if line & 0b1 != 0b0 {
        let imm = line & 0b111;
        reg[r1 as usize] -= imm as i32;
    } else {
        let r2 = (line >> 1) & 0b11;
        reg[r1 as usize] -= reg[r2 as usize];
    }
}
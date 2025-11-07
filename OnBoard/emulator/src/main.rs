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

const CALL_RM: OperationCode = 0b000; // Read memory
const CALL_WM: OperationCode = 0b001; // write in memory from register
const CALL_IM: OperationCode = 0b010; // write in memory from input
const CALL_OM: OperationCode = 0b011; // output memory
const CALL_EX: OperationCode = 0b100; // exit
const CALL_OR: OperationCode = 0b101; // output register
const CALL_WR: OperationCode = 0b110; // write register

// The four registers, the constants never got used
type Registry = u8;
const R0: Registry = 0b00;
const R1: Registry = 0b01;
const R2: Registry = 0b10;
const R3: Registry = 0b11;

// The whole emulator is running off of this main function which loops until it has performed every action accordingly after reading a binary file
fn main() {
    // get the binary file and then make a vector with one byte/command per slot
    let args: Vec<String> = env::args().collect();
    let lines: Vec<u8> = fs::read(&args[1]).expect("File read error");

    let mut pc: usize = 0; // variable for keeping track what command number/what line we are on.

    // vectors for the registers, the pointer and memory to simulate RAM, doesn't function perfectly but hopefully well enough
    let mut register: [i32; 4] = [0;4];

    let mut pointer: [u32; 32] = [0; 32];

    let mut memory: [u8; 512] = [0; 512];
    
    // amount of commands
    let length: usize = lines.len();

    // println!("program has started");

    // the main while loop that runs the whole emulator, keeps going until we are on the last line, uses a match case on the
    // first 3 bites to check what command to perform/what method to run
    while pc < lines.len() {
        match (lines[pc] >> 5) & 0b111 {
            CAL => call(lines[pc] & 0b11111, &mut register, &mut pc, &mut pointer, &mut memory, &length),
            ADD => add(lines[pc] & 0b11111, &mut register),
            JMP => {pc += (lines[pc] & 0b11111) as usize; continue;} // skips x amount of lines, defined by the last 5 bites
            BEQ => {beq(lines[pc] & 0b11111, &register, &mut pc); continue;}
            SET => set(lines[pc] & 0b11111, &mut register),
            SUB => sub(lines[pc] & 0b11111, &mut register),
            JB =>  {if pc > (lines[pc] & 0b11111) as usize {pc -= (lines[pc] & 0b11111) as usize;} continue;} // same as jump but backwards
            _ => println!("Invalid operation on line {}!", pc), // only possible if first 3 bites are ones
        }

        // increment line number
        pc += 1;
    }
}

// effectively a second main function, except it doesn't loop. call let's you essentially use another set of commands/subcommands
// which are determined by the next 3 bites, and then does a match case
fn call(line: u8, reg: &mut [i32; 4], pc: &mut usize, pointer: &mut [u32; 32], memory: &mut [u8; 512], length: &usize) {
    // println!("doing call");

    //Call always end with a register as the last 2 bites, here it is saved for when it only needs to read it
    let regnum = (line & 0b11) as usize;
    let register = reg[regnum];

    match (line >> 2) & 0b111 {
        CALL_RM => read_memory(reg, regnum, pointer, memory),                   // call for reading from memory, n:th position
        CALL_WM => {let mut bytes = register.to_string().as_bytes().to_vec();
            write_memory(reg[0], &mut bytes, pointer, memory);},                // call for storing to memory, n:th position
        CALL_IM => input_memory(register, reg[0], pointer, memory),             // call for reading input to memory, n:th position
        CALL_OM => output_memory(register, pointer, memory),                    // call for writing to console from memory, n:th position
        CALL_EX => *pc = *length,                                               // call for closing the application
        CALL_OR => println!("{}", register),                                    // call for outputting register to console
        CALL_WR => input_register(regnum, reg),                                 // call for reading input into register
        _ => println!("Invalid operation on line {}!", pc),                     // only happens if the 3 bites are 111
    }

}

// Function to read a saved i32 integer from memory into a register
// Checks if the register is within range, and then reads the divided integer (memory is 1 byte each, each number is saved as a letter)
// and puts it in the same register used to tell what pointer/slot to use/get
fn read_memory(reg: &mut [i32; 4], regnum: usize, pointer: &mut [u32; 32], memory: &mut [u8; 512]) {
    // println!("reading memory");
    if !((0<= reg[regnum]) && (reg[regnum] <=31)) {println!("Invalid read memory call, register contains negative value."); return;}   
        let mut address = pointer[reg[regnum] as usize] as usize;

        let mut ascii_number = String::new();
        
        while memory[address] != 0 {
            ascii_number.push(memory[address] as char);
            address += 1;
        }

        reg[regnum] = ascii_number.parse().expect("Address contains non integer");
}

// Function to write to memory, is also used by input_memory()
// either writes a register or input into the memory vector
fn write_memory(reg: i32, bytes: &mut Vec<u8>, pointer: &mut [u32; 32], memory: &mut [u8; 512]) {
    // println!("writing memory");

    // Too lazy to add an edge case scenario, reg=31 should work but in the if-statements I also check reg+1, which would be out of bounds
    // Could also just add one more integer to pointer vector? Not gonna bother with it for now
    if !((0<= reg) && (reg <=30)) {
        println!("register doesn't contain an integer between 0 and 31");
        return;
    }

    let register = reg as usize;

    // adds zero character at the end of a string byte vector to signify end
    bytes.push(0);
    let length = bytes.len();
    
    // When it tries to write a "string" to memory, either the space pointed to will
    // have enough storage for the string, or it won't, so a few if-else statements is used
    // to tell if there is more than enough storage needed/exactly enough storage needed

    // First if checks if the "string" stored at the pointer location has enough storage for the current
    // string to be written, and then overwrites and deletes any excess remnants of the old string in memory
    if (pointer[register + 1] - pointer[register]) >= length as u32 {
        let mut i = pointer[register] as usize;
        //let mut next = pointer[register + 1] as usize;

        // overwrite parts/all of the old string with new string
        let mut index = 0;
        while i < length {
            memory[i] = bytes[index]; //i should not be used for bytes?
            i += 1;
            index += 1;
        }

        //Delete old string
        //while i < next {
        while memory[i] != 0 {
            memory[i] = 0;
            i += 1;
        }

    // the second if checks if there is no next word, in which case it can write freely
    // might need to change this to be like the next if statement/remove this one
    } else if pointer[register + 1] == 0{
        let mut i = pointer[register] as usize;
        let mut index = 0;

        while i < length {
            memory[i] = bytes[index]; //i should not be used for bytes?
            i += 1;
            index += 1;
        }
    // Checks if there is any place in memory to write the string to, by checking for
    // empty space equal to the length of the string, and then writes it in there
    
    } else if (pointer[register + 1] - pointer[register]) < length as u32 {
        let mut i = pointer[register] as usize;

        // I might have forgotten to still delete the old string, so added this
        // Delete old string
        while memory[i] != 0 {
            memory[i] = 0;
            i += 1;
        }

        let mut free_space = 0;

        // Loops through all of memory and then writes in the string and updates pointer to the new position
        for j in 0..512 {
            if memory[j] == 0 {
                free_space += 1;
                if free_space == length {
                    let mut index = 0;
                    for k in j..(j+length) {
                        memory[k] = bytes[index];
                        index += 1;
                    }
                    pointer[register] = (j - length + 1) as u32;
                    return;
                }
            } else {
                free_space=0;
            }
        }

        println!("No memory available.");
    }
}

// Function to print out a "string" in memory
fn output_memory(register: i32, pointer: &mut [u32; 32], memory: &mut [u8; 512]){
    // println!("output memory");
    if !((0<= register) && (register <=31)) {println!("Invalid memory output call, out of bounds."); return;}

    // Prints each byte as a letter one by one and then a newline
    let mut i = pointer[register as usize] as usize;
    while memory[i] != 0 {
        print!("{}", memory[i] as char);
        i += 1;
    }
    println!();
}

// Function to get an input and then put it in memory by calling write_memory()
// Converts the inputted string into a vector of bytes, each representing an ascii character but in byte form
fn input_memory(reg: i32, reg0: i32, pointer: &mut [u32; 32], memory: &mut [u8; 512]) {
    // println!("input memory");

    let input = io::stdin();
    let mut line = String::new();

    if !((0<= reg) && (reg <=31)) {
        println!("register doesn't contain an integer between 0 and 31");
        return;
    }

    let register = reg as usize;

    //input goes into line
    input
        .read_line(&mut line)
        .expect("Not a line");

    let mut bytes = line.trim().as_bytes().to_vec();

    write_memory(reg0, &mut bytes, pointer, memory);
}

// Function to take input and put it in a register
fn input_register(line: usize, reg: &mut [i32; 4]) {
    // println!("input register");
    let input = io::stdin();
    let mut input_line = String::new();

    //input goes into line
    input
        .read_line(&mut input_line)
        .expect("Not a line");

    reg[line as usize] = input_line.trim().parse().expect("You did not input a number");
}

//All of these functions read the first two bites given as a register, then the next two bites as
// a register or immediate/integer (0-3) and then the last bite to determine if the previous two bites
// should be interpreted as a register or immediate: (reg1: xx) (reg2/imm: xx) (flag: x): xx xx x

// Function for addition, can be used to either add a registers value to another register or add an 
// immediate/integer up to 2 bites (0-3) value to a register
fn add(line: u8, reg: &mut [i32; 4]) {
    // println!("add");
    let r1 = (line >> 3) & 0b11;
    let added = (line >> 1) & 0b11;

    // If the last bite is 1, treat the second "argument" as an immediate/integer up to 2 bites
    // Otherwise, treat it as a code for a register
    if line & 0b1 != 0b0 {
        reg[r1 as usize] += added as i32;
    } else {
        reg[r1 as usize] += reg[added as usize];
    }
}

// Function for checking if two registers are equal, or a register is equal to an immediate/integer up to 2 bites
// if they are equal then it skips a line, if not it just continues like normal
// intended use is that you put a jump back on the line under, so it keeps looping if statement is false
// Can be used in other ways
fn beq(line: u8, reg: &[i32; 4], pc: &mut usize) {
    // println!("beq");
    let r1 = (line >> 3) & 0b11;
    let comparison = (line >> 1) & 0b11;

    if line & 0b1 != 0b0 {
        // println!("beq imm");
        if reg[r1 as usize] == comparison as i32 {
            // println!("beq imm is equal");
            // println!("pc is: {}", pc);
            *pc += 2;        
        } else {
            // println!("beq imm is not equal");
            *pc += 1;
        }
    } else if reg[r1 as usize] == reg[comparison as usize]{
        // println!("beq registers");
        *pc += 2;          
    } else {
        // println!("beq but nothing");
        *pc += 1;
    }
 
}

// Function that sets the value of a register to another register value or an immediate/integer value up to 2 bites (0-3)
// Works the same way as add
fn set(line: u8, reg: &mut [i32; 4]) {
    // println!("set");
    let r1 = (line >> 3) & 0b11;
    let set_num = (line >> 1) & 0b11;


    if line & 0b1 != 0b0 {
        reg[r1 as usize] = set_num as i32;
    } else {
        reg[r1 as usize] = reg[set_num as usize];
    }
}

// Function for subtracting a registers value with another registers value or an 
// immediate/integer up to 2 bites (0-3), works same way as add
fn sub(line: u8, reg: &mut [i32; 4]) {
    // println!("sub");
    let r1 = (line >> 3) & 0b11;
    let subtracted = (line >> 1) & 0b11;


    if line & 0b1 != 0b0 {
        reg[r1 as usize] -= subtracted as i32;
    } else {
        reg[r1 as usize] -= reg[subtracted as usize];
    }
}
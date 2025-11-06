//Operations
type OperationCode = u8;
const CAL: OperationCode = 0b000; //call
const ADD: OperationCode = 0b100; //add 
const J: OperationCode = 0b010; //Jump distance in program
const BEQ: OperationCode = 0b001; //Branch if equal to 0
const SET: OperationCode = 0b110; //Set value
const SUB: OperationCode = 0b101; //subtract
const JB: OperationCode = 0b011; // jump backwards in program

//Registry
type Registry = u8;
const R0: Registry = 0b00;
const R1: Registry = 0b10;
const R2: Registry = 0b01;
const R3: Registry = 0b11;

const BOARD: [[&str; 7]; 7] = [
    ["  ", "  ", "  ", "  ", "  ", "  ", "  "],
    ["S ", "  ", "1 ", "  ", "SU", "  ", "J "],
    ["  ", "  ", "2 ", "  ", "C ", "  ", "JB"],
    ["AD", "  ", "10", "  ", "  ", "  ", "  "],
    ["  ", "  ", "0 ", "  ", "  ", "  ", "B "],
    ["  ", "  ", "  ", "  ", "R2", "  ", "  "],
    ["R0", "  ", "R1", "  ", "  ", "R3", "  "],
];

pub fn assemble(input: &str) -> Vec<u8> {
    let mut man: (usize, usize) = (0, 0);
    let paths: std::str::Split<'_, char> = input.split('*');
    let mut result: Vec<u8> = Vec::new();
    for path in paths {
        let directions: std::str::Chars<'_> = path.chars();
        let mut command: Option<u8> = None;
        let mut uses_imm: Option<bool> = None;
        for direction in directions {
            match direction {
                '>' => {
                    if man.0 == 6 {
                        man.0 = 0;
                    } else {
                        man.0 += 1;
                    }
                }
                '<' => {
                    if man.0 == 0 {
                        man.0 = 6;
                    } else {
                        man.0 -= 1;
                    }
                }
                'v' => {
                    if man.1 == 6 {
                        man.1 = 0;
                    } else {
                        man.1 += 1;
                    }
                }
                '^' => {
                    if man.1 == 0 {
                        man.1 = 6;
                    } else {
                        man.1 -= 1;
                    }
                }
                _ => {}
            }

            let mut segment: Option<u8> = None;
            let mut seg_len: u8 = 0;
            let position: &str = BOARD[man.1][man.0];

            match position {
                "J " => {
                    segment = Some(J);
                    seg_len = 3;
                    uses_imm = None;
                    println!("did jump");
                }
                "JB" => {
                    segment = Some(JB);
                    seg_len = 3;
                    uses_imm = None;
                    println!("did jump back");
                }
                "S " => {
                    segment = Some(SET);
                    seg_len = 3;
                    if command.is_none() {
                        uses_imm = Some(false);
                    }
                    println!("did set");
                }
                "C " => {
                    segment = Some(CAL);
                    seg_len = 3;
                    println!("did call");
                    uses_imm = None;
                }
                "AD" => {
                    segment = Some(ADD);
                    seg_len = 3;
                    println!("did add");
                    if command.is_none() {
                        uses_imm = Some(false);
                    }
                }
                "SU" => {
                    segment = Some(SUB);
                    seg_len = 3;
                    println!("did sub");
                    if command.is_none() {
                        uses_imm = Some(false);
                    }
                }
                "B " => {
                    segment = Some(BEQ);
                    seg_len = 3;
                    if command.is_none() {
                        uses_imm = Some(false);
                    }
                    println!("did beq");
                }
                "R0" => {
                    segment = Some(R0);
                    seg_len = 2;
                    if uses_imm.is_some() {
                        uses_imm = Some(false);
                    }

                    println!("did R0");
                }
                "R1" => {
                    segment = Some(R1);
                    seg_len = 2;
                    if uses_imm.is_some() {
                        uses_imm = Some(false);
                    }
                    println!("did R1");
                }
                "R2" => {
                    segment = Some(R2);
                    seg_len = 2;
                    println!("did R2");
                    if uses_imm.is_some() {
                        uses_imm = Some(false);
                    }
                }
                "R3" => {
                    segment = Some(R3);
                    seg_len = 2;
                    if uses_imm.is_some() {
                        uses_imm = Some(false);
                    }
                    println!("did R3");
                }
                "0 " => {
                    segment = Some(0b00);
                    seg_len = 2;
                    if uses_imm.is_some() {
                        uses_imm = Some(true);
                    }
                    println!("did 0");
                }
                "1 " => {
                    segment = Some(0b01);
                    seg_len = 2;
                    if uses_imm.is_some() {
                        uses_imm = Some(true);
                    }
                    println!("did 1");
                }
                "2 " => {
                    segment = Some(0b10);
                    seg_len = 2;
                    if uses_imm.is_some() {
                        uses_imm = Some(true);
                    }
                    println!("did 2");
                }
                "10" => {
                    segment = Some(0b11);
                    seg_len = 2;
                    if uses_imm.is_some() {
                        uses_imm = Some(true);
                    }
                    println!("did 10");
                }
                _ => {}
            }
            if command.is_none() && segment.is_some() {
                command = segment;
            } else if segment.is_some() && command.unwrap() != J &&command.unwrap() != JB{
                command = Some((command.unwrap() << seg_len) | (segment.unwrap()));
            }else if segment.is_some() {
                 command = Some((command.unwrap() << 5) | (segment.unwrap()));
            }
        }
        if command.is_some() {
            if uses_imm.is_some() {
                println!("will add flag");
                if uses_imm.unwrap() {
                    command = Some((command.unwrap() << 1) | 1);
                } else {
                    command = Some((command.unwrap() << 1) | 0);
                }
            }

            result.push(command.unwrap());
            command = Some(0);
        }
    }
    return result;
}

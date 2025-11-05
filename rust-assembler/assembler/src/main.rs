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

const BOARD: [[&str; 8]; 8] = [
    ["  ", "  ", "  ", "  ", "  ", "SU", "  ", "J "],
    ["  ", "  ", "S ", "1 ", "  ", "  ", "  ", "  "],
    ["0 ", "  ", "  ", "  ", "  ", "  ", "  ", "JB"],
    ["  ", "  ", "  ", "2 ", "  ", "  ", "C ", "  "],
    ["AD", "  ", "  ", "10", "  ", "  ", "  ", "  "],
    ["  ", "  ", "  ", "  ", "  ", "  ", "  ", "B "],
    ["  ", "  ", "  ", "  ", "  ", "R2", "  ", "  "],
    ["R0", "  ", "R1", "  ", "  ", "  ", "R3", "  "],
];

pub fn assemble(input: &str) -> Vec<u8> {
    let mut man: (usize, usize) = (0, 0);
    let paths: std::str::Split<'_, char> = input.split('*');
    let mut result: Vec<u8> = Vec::new();
    for path in paths {
        let directions: std::str::Chars<'_> = path.chars();
        let mut command: Option<u8> = None;
        for direction in directions {
            match direction {
                '>' => {
                    if man.0 == 7 {
                        man.0 = 0;
                    } else {
                        man.0 += 1;
                    }
                }
                '<' => {
                    if man.0 == 0 {
                        man.0 = 7;
                    } else {
                        man.0 -= 1;
                    }
                }
                'v' => {
                    if man.1 == 7 {
                        man.1 = 0;
                    } else {
                        man.1 += 1;
                    }
                }
                '^' => {
                    if man.1 == 0 {
                        man.1 = 7;
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
                    man.0 = 0;
                    man.1 = 0;
                }
                "JB" => {
                    segment = Some(JB);
                    seg_len = 3;
                    man.0 = 0;
                    man.1 = 0;
                }
                "S " => {
                    segment = Some(SET);
                    seg_len = 3;
                },
                "C " => {
                    segment = Some(CAL);
                    seg_len = 3;
                },
                "AD" => {
                    segment = Some(ADD);
                    seg_len = 3;
                },
                "SU" => {
                    segment = Some(SUB);
                    seg_len = 3;
                },
                "B " => {
                    segment = Some(BEQ);
                    seg_len = 3;
                },
                "R0" => {
                    segment = Some(R0);
                    seg_len = 2;
                },
                "R1" => {
                    segment = Some(R1);
                    seg_len = 2;
                },
                "R2" => {
                    segment = Some(R2);
                    seg_len = 2;
                },
                "R3" => {
                    segment = Some(R3);
                    seg_len = 2;
                },
                "0 " => {
                    segment = Some(0b00);
                    seg_len = 2;
                    },
                "1 " => {
                    segment = Some(0b01);
                    seg_len = 2;
                    },
                "2 " => {
                    segment = Some(0b10);
                    seg_len = 2;
                    },
                "10" => {
                    segment = Some(0b11);
                    seg_len = 2;
                    },
                _ => {},
            }
            if command.is_none() && segment.is_some() {
                command = segment;
            } else if segment.is_some() {
                command = Some((command.unwrap() << seg_len) | (segment.unwrap()));
            }
        }
        if command.is_some() {
            result.push(command.unwrap());
        }
    }
    return result;
}

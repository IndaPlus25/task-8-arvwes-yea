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
//board
const BOARD: [[&str; 7]; 7] = [
    ["  ", "  ", "  ", "  ", "  ", "  ", "  "],
    ["S ", "  ", "1 ", "  ", "SU", "  ", "J "],
    ["  ", "  ", "2 ", "  ", "C ", "  ", "JB"],
    ["AD", "  ", "10", "  ", "  ", "  ", "  "],
    ["  ", "  ", "0 ", "  ", "  ", "  ", "B "],
    ["  ", "  ", "  ", "  ", "R2", "  ", "  "],
    ["R0", "  ", "R1", "  ", "  ", "R3", "  "],
];
/**
 * Converts an input of directions and stars (<>^v and *) to functions located on the board
 * then converts the functions to binary
 * returns the binary code for all functions as a vector of binary strings
 */
pub fn assemble(input: &str) -> Vec<u8> {
    let mut player: (usize, usize) = (0, 0); // sets player starting position to top left

    let paths: std::str::Split<'_, char> = input.split('*');// splits the directions in to paths each path represents a command
    let mut result: Vec<u8> = Vec::new(); //result vector

    // loops trough all paths creating all commands and adding them to the result vector
    for path in paths {
        let directions: std::str::Chars<'_> = path.chars(); //creates an iterator of directions
        let mut command: Option<u8> = None; // creates an empty command
        let mut uses_imm: Option<bool> = None;
        // loops directions and determens mans position and converts position to operations
        for direction in directions {
            match direction {
                '>' => {
                    if player.0 == 6 {
                        player.0 = 0; // if player walks off player appers on the opposite side
                    } else {
                        player.0 += 1;
                    }
                }
                '<' => {
                    if player.0 == 0 {
                        player.0 = 6;// if player walks off player appers on the opposite side
                    } else {
                        player.0 -= 1;
                    }
                }
                'v' => {
                    if player.1 == 6 {
                        player.1 = 0;// if player walks off player appers on the opposite side
                    } else {
                        player.1 += 1;
                    }
                }
                '^' => {
                    if player.1 == 0 {
                        player.1 = 6;// if player walks off player appers on the opposite side
                    } else {
                        player.1 -= 1;
                    }
                }
                _ => {}
            }

            let mut segment: Option<u8> = None;
            let mut seg_len: u8 = 0;
            let position: &str = BOARD[player.1][player.0];
            // determan what operation the player is standing on and saves vaalue of operation as binary
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
            // creates a command if command does not exist and if it does exist the next operation segment is added
            if command.is_none() && segment.is_some() {
                command = segment;
            } else if segment.is_some() && command.unwrap() != J &&command.unwrap() != JB{
                // shifts the command to the left to make room for the segment and adds the segment (if it isn't a jump command)
                command = Some((command.unwrap() << seg_len) | (segment.unwrap()));  
            }else if segment.is_some() {
                // if its a jump command it is allways shifted 5blocks since a jump comand is 3bits for operation and 5 for jump distance
                // doing it like this is not recomended
                 command = Some((command.unwrap() << 5) | (segment.unwrap()));
            }
        }
        // if its an operation which wants a flag a flag is added
        if command.is_some() {
            if uses_imm.is_some() {
                println!("will add flag");
                if uses_imm.unwrap() {
                    command = Some((command.unwrap() << 1) | 1);
                } else {
                    command = Some((command.unwrap() << 1) | 0);
                }
            }
            // pushes 8 bit command to result vector
            result.push(command.unwrap());

        }
    }
    return result;
}

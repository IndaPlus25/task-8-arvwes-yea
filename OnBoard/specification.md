
Instructions:
Type	                    Encoding (bit layout)
Register / Immediate	    op<7:5>, reg/imm<4:3>, reg/imm<2:1>, immFlag<0>
Jump / Jump Back	        op<7:5>, addr<4:0>
Special (call/IO/exit)  	op<7:5>, followed by optional packed values depending on instruction

Regester instructions:
Instruction         Description
ADD rs rt	        rt = rt + rs
ADD rs imm	        rt = rt + imm
SUB rs rt	        rt = rt - rs
SUB rs imm	        rt = rt - imm
SET rt imm	        rt = imm
BEQ rs rt	        If (rs == rt) → skip next instruction

immFlag = 1 → second argument is an immediate value (2 bits).
immFlag = 0 → second argument is a register.

Jump Instructions:
Instruction         Description
J addr              Jump forward addr lines
JB addr             Jump backward addr lines

Instruction	        Behavior	                        Encoding
CAL	Calls           special function or subroutine	    Opcode only (000xxxxx)
INPUT	            Read integer from stdin → R1	    Implemented via CAL + R1?
PRINT	            Print R1 to stdout	                Implemented via CAL + R1?
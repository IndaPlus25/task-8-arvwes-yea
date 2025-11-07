### ON BOARD
## Overview
You write code by moving a character called Gob to tiles on the board
movemet is done using characters:
\>  move right
<  move left
v  move down
^  move up
\*  finalize instruction (push byte to output)
The assembler builds 8-bit instructions while you move

For the board se: on-board-board.png

comments can be writen after a '\*' IF Gob is standing on an empty tile when '\*' is entered. but not inbetwen direction characters.


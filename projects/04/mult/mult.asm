// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)

// initialize result to 0
@R2
M=0

// copy multiplier to different address to avoid mutating the input
@R0
D=M
@i
M=D

// end if R0 or R1 <= 0
@END
D;JLE
@R1
D=M
@END
D;JLE

(LOOP)
// add R1 to result
@R1
D=M
@R2
M=D+M

// decrement i
@i
MD=M-1

// loop until i <= 0
@LOOP
D;JGT

(END)
@END
0;JMP

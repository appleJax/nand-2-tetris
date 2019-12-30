// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)

// Put your code here.

@R2 // initialize result to 0
M=0

@R0 // copy multiplier to different address to avoid mutating the input
D=M
@i
M=D

@END
D;JEQ

(LOOP)
@R1
D=M
@R2
M=D+M

@i
MD=M-1
@END
D;JLE

@LOOP
0;JMP

(END)
@END
0;JMP

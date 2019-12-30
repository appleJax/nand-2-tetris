// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

// Put your code here.

(START)
  @SCREEN // initialize @position to start of screen memory map
  D=A
  @position
  M=D

  @8192 // initialize counter to number of registers in screen map (256 x (512 / 16))
  D=A
  @i
  M=D

(DRAW_LOOP)
  @READ_KBD
  0;JMP

(DRAW_REGISTER)
  @toggle_value // use -1 or 0 depending on KBD input
  D=M
  @position
  A=M
  M=D

  @position // increment position, decrement counter
  M=M+1
  @i
  MD=M-1

  @START // when counter gets to 0, we are at the end of the screen memory map, so start over
  D;JLE

  @DRAW_LOOP
  0;JMP

(READ_KBD)
  @KBD
  D=M
  @TOGGLE_OFF
  D;JEQ
  @TOGGLE_ON
  0;JMP

(TOGGLE_ON)
  @toggle_value
  M=-1
  @DRAW_REGISTER
  0;JMP

(TOGGLE_OFF)
  @toggle_value
  M=0
  @DRAW_REGISTER
  0;JMP

# CHIP-8-Assembler

Chip-8 ASM Assembler written in Rust. Build using 'make all'.

Opcode reference: https://en.wikipedia.org/wiki/CHIP-8#Opcode_table

## Call
SYS  addr     - Calls RCA 1802 program at NNN (0NNN). <br />

## Display
CLS           - Clears display (00E0). <br />
DRW  Vx, Vy, nibble - Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels (DXYN). <br />

## Flow
RET           - Returns from subroutine (00EE). <br />
JP   V0, addr - Jump to V0+NNN (BNNN). <br />
JP   addr     - Jump to NNN (1NNN). <br />
CALL addr     - Calls subroutine at NNN (2NNN). <br />

## Conditional
SE   Vx, byte - Skips next instruction if Vx==byte (3XNN). <br />
SE   Vx, Vy   - Skips next instruction if Vx==Vy (5XY0). <br />
SNE  Vx, byte - Skips next instruction if Vx!=byte (4XNN). <br />
SNE  Vx, Vy   - Skips next instruction if Vx!=Vy (9XY0). <br />

## Math
ADD  Vx, byte - Adds Vx+=byte (7XNN).
ADD  Vx, Vy   - Adds Vx+=Vy. VF is set to 1 when there's a carry, and to 0 when there isn't (8XY4). <br />
SUB  Vx, Vy   - Subtract Vx-=Vy. VF is set to 0 when there's a borrow, and 1 when there isn't (math, 8XY5). <br />
SUBN Vx, Vy   - Subtract Vx=Vy-Vx. VF is set to 0 when there's a borrow, and 1 when there isn't (math, 8XY7). <br />
RND  Vx, byte - Loads rand()&NN into Vx (CXNN). <br />

## Bitwise
OR   Vx, Vy   - Or Vx|=Vy (8XY1). <br />
AND  Vx, Vy   - And Vx&=Vy (8XY2). <br />
XOR  Vx, Vy   - Xor Vx^=Vy (8XY3). <br />
SHR  Vx       - Bitwise shift right Vx>>1. VF is set to the value of the least significant bit of VX before the shift (8XY6). <br />
SHL  Vx       - Bitwise shift left Vx<<1. VF is set to the value of the most significant bit of VX before the shift (8XYE). <br />

## Keypad 
SKP  Vx       - Skips next instruction if key==Vx (EX9E). <br />
SKNP Vx       - Skips next instruction if key!=Vx (EXA1). <br />
LD Vx, K      - Sets Vx to next key pree (FX0A). <br />

## Memory
ADD I, Vx     - Sets index+=Vx (FX1E). <br />
LD I, addr    - Sets index=NNN (ANNN). <br />
LD F, Vx      - Sets index=sprite_address(Vx) (FX29). <br />
LD I, Vx      - Stores registers V0..Vx in memory starting at index (FX55). <br />
LD Vx, I      - Loads values in memory starting at index into registers V0..Vx (FX65). <br />

## Assignment
LD Vx, byte   - Sets Vx=NN (6XNN). <br />
LD Vx, Vy     - Sets Vx=Vy (8XY0). <br />

## Timer
LD Vx, DT     - Sets Vx=delay_timer (FX07). <br />
LD DT, Vx     - Sets delay_timer=Vx (FX15). <br />
LD ST, Vx     - Sets sound_timer=Vx (FX18). <br />

## Binary-Coded Decimal
LD B, Vx      - Stores binary-coded decimal representation of Vx (BCD, FX33). <br />

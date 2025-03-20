<div align="center">
  <img src=".assets/logo.png" width="200" style="border-radius: 10%;" />
</div>

<h1 align="center">IGUANA MAC INTERPRETER</h1>


A simple 16-bit stack-based assembly interpreter for Andrew S. Tanenbaum's MAC assembly language. The interpreter is written in Rust and supports a variety of operations, including arithmetic, memory, control flow, bitwise, and debug operations.

---

# Example Assembly Code

The following example finds the largest of two values:

```python
# program to print a string
.data
STRING: .asciiz "Hello, World!" # alocates a string in memory
CHAR_POINTER: .space 2          # alocates 2 bytes in memory for a "pointer"
ONE: .word 1                    # alocates a word in memory for a auxiliar variable
.text
MAIN:
    LOCO STRING         # ac = STRING as a pointer
    STOD CHAR_POINTER   # *CHAR_POINTER = STRING
LOOP:
    LODD CHAR_POINTER   # ac = *CHAR_POINTER
    SWAP                # sp = ac ; ac = sp
    POP                 # ac = *sp ; sp = sp + 1 (decrement sp)
    JZER END            # if ac == 0 goto END
    PRINTACCHAR         # print ac as a char
    LODD CHAR_POINTER        # ac = *CHAR_POINTER
    SUBD ONE            # ac = ac - 1
    STOD CHAR_POINTER   # *CHAR_POINTER = ac
    JUMP LOOP           # goto LOOP
END:
    HALT                # finishes the program
```

output:
```
Hello, World!
```

---

# Usage Instructions
## Running a File
To run a assembly file, use the following command:
```bash	
iguana run <file>
```

## Display Interpreter Informations
To display the interpreter information, use the following command:
```bash
iguana info
```

---

# Build Instructions

1. **Ensure that you have Rust and Cargo installed**:
   - Follow the installation guide on the official [Rust website](https://www.rust-lang.org/learn/get-started) if needed.

2. **Clone the repository**:
   - Use the following command to clone the repository:
     ```bash
     git clone https://github.com/joeCavZero/iguana-mac-interpreter.git
     ```

3. **Build the source code**:
   - Run the following command to build the project in release mode:
     ```bash
     cargo build --release
     ```

4. **Locate the generated binary**:
   - After the build is complete, the binary will be located at:
     ```bash
     ./target/release/iguana.exe on Windows 
     ( or iguana.app on MacOS or iguana on Linux )
     ```

5. **Copy the binary to a convenient location**:
   - Copy the binary to a location where you can easily access it, such as a directory in your system's PATH.
   - You can also save the binary in the environment variable PATH, allowing you to run the program from any directory.

---

# Iguana's MAC Operations Guide

## Constant Loading

- **LOCO X**  
  **Behavior**: Loads the constant value `X` into the accumulator (`ac`).  
  **Pseudo-behavior**: `ac = X`

## Memory Operations

- **LODD X**  
  **Behavior**: Loads the value from the stack at address `X` into the accumulator (`ac`).  
  **Pseudo-behavior**: `ac = M[X]`

- **STOD X**  
  **Behavior**: Stores the value in the accumulator (`ac`) into the stack at address `X`.  
  **Pseudo-behavior**: `M[X] = ac`

- **LODL X**  
  **Behavior**: Loads a value from the stack relative to the stack pointer (`sp`) into the accumulator (`ac`).  
  **Pseudo-behavior**: `ac = M[sp + X]`

- **STOL X**  
  **Behavior**: Stores the value in the accumulator (`ac`) into the stack relative to the stack pointer (`sp`).  
  **Pseudo-behavior**: `M[sp + X] = ac`

- **PSHI**  
  **Behavior**: Pushes the value in the accumulator (`ac`) onto the stack.  
  **Pseudo-behavior**: `sp = sp - 1; M[sp] = M[ac]`

- **POPI**  
  **Behavior**: Pops a value from the stack into the memory address stored in the accumulator (`AC`).  
  **Pseudo-behavior**: `M[AC] = M[SP]; SP = SP + 1`

- **PUSH**  
  **Behavior**: Pushes the value in the accumulator (`ac`) onto the stack.  
  **Pseudo-behavior**: `sp = sp - 1; M[sp] = ac`

- **POP**  
  **Behavior**: Pops a value from the stack into the accumulator (`ac`).  
  **Pseudo-behavior**: `ac = M[sp]; sp = sp + 1`

- **SWAP**  
  **Behavior**: Swaps the value in the accumulator (`ac`) with the value at the top of the stack.  
  **Pseudo-behavior**: `temp = ac; ac = sp; sp = temp`

- **INSP X**  
  **Behavior**: Increments the stack pointer (`sp`) by `X`.  
  **Pseudo-behavior**: `sp = sp - X`

- **DESP X**  
  **Behavior**: Decrements the stack pointer (`sp`) by `X`.  
  **Pseudo-behavior**: `sp = sp + X`

## Arithmetic Operations

- **ADDD X**  
  **Behavior**: Adds the value at address `X` in the stack to the accumulator (`ac`).  
  **Pseudo-behavior**: `ac = ac + M[X]`

- **SUBD X**  
  **Behavior**: Subtracts the value at address `X` in the stack from the accumulator (`ac`).  
  **Pseudo-behavior**: `ac = ac - M[X]`

- **MULD X**  
  **Behavior**: Multiplies the value in the accumulator (`ac`) by the value at address `X` in the stack.  
  **Pseudo-behavior**: `ac = ac * M[X]`

- **DIVD X**  
  **Behavior**: Divides the value in the accumulator (`ac`) by the value at address `X` in the stack.  
  **Pseudo-behavior**: `ac = ac / M[X]`

- **ADDL X**  
  **Behavior**: Adds a value from the stack relative to the stack pointer (`sp`) to the accumulator (`ac`).  
  **Pseudo-behavior**: `ac = ac + M[sp + X]`

- **SUBL X**  
  **Behavior**: Subtracts a value from the stack relative to the stack pointer (`sp`) from the accumulator (`ac`).  
  **Pseudo-behavior**: `ac = ac - M[sp + X]`

- **MULL X**  
  **Behavior**: Multiplies a value from the stack relative to the stack pointer (`sp`) to the accumulator (`ac`).  
  **Pseudo-behavior**: `ac = ac * M[sp + X]`

- **DIVL X**  
  **Behavior**: Divides a value from the stack relative to the stack pointer (`sp`) from the accumulator (`ac`).  
  **Pseudo-behavior**: `ac = ac / M[sp + X]`

## Control Flow Operations

- **JPOS X**  
  **Behavior**: Jumps to the instruction at line `X` if the accumulator (`ac`) is positive.  
  **Pseudo-behavior**: `if ac >= 0: pc = pc_of_instruction_on_line( X ), else: pc = pc + 1`

- **JZER X**  
  **Behavior**: Jumps to the instruction at line `X` if the accumulator (`ac`) is zero.  
  **Pseudo-behavior**: `if ac == 0: pc = pc_of_instruction_on_line( X ), else: pc = pc + 1`

- **JUMP X**  
  **Behavior**: Unconditionally jumps to the instruction at line `X`.  
  **Pseudo-behavior**: `pc = pc_of_instruction_on_line( X )`

- **JNEG X**  
  **Behavior**: Jumps to the instruction at line `X` if the accumulator (`ac`) is negative.  
  **Pseudo-behavior**: `if ac < 0: pc = pc_of_instruction_on_line( X ), else: pc = pc + 1`

- **JNZE X**  
  **Behavior**: Jumps to the instruction at line `X` if the accumulator (`ac`) is not zero.  
  **Pseudo-behavior**: `if ac != 0: pc = pc_of_instruction_on_line( X ), else: pc = pc + 1`

- **CALL X**  
  **Behavior**: Calls a subroutine at line `X`, saving the return address on the stack.  
  **Pseudo-behavior**: `sp = sp - 1; M[sp] = current_line + 1; pc = pc_of_instruction_on_line( X )`

- **RETN**  
  **Behavior**: Returns from a subroutine by popping the return address from the stack.  
  **Pseudo-behavior**: `pc = pc_of_instruction_on_line( M[sp] ); sp = sp + 1`

## Bitwise Operations

- **ANDI X**  
  **Behavior**: Performs a bitwise AND between the accumulator (`ac`) and the constant `X`.  
  **Pseudo-behavior**: `ac = ac & X`

- **ORI X**  
  **Behavior**: Performs a bitwise OR between the accumulator (`ac`) and the constant `X`.  
  **Pseudo-behavior**: `ac = ac | X`

- **XORI X**  
  **Behavior**: Performs a bitwise XOR between the accumulator (`ac`) and the constant `X`.  
  **Pseudo-behavior**: `ac = ac ^ X`

- **NOT**  
  **Behavior**: Performs a bitwise NOT on the accumulator (`ac`).  
  **Pseudo-behavior**: `ac = ~ac`

- **SHFLI X**  
  **Behavior**: Shifts the value in the accumulator (`ac`) left by `X` bits.  
  **Pseudo-behavior**: `ac = ac << X`

- **SHFRI X**  
  **Behavior**: Shifts the value in the accumulator (`ac`) right by `X` bits.  
  **Pseudo-behavior**: `ac = ac >> X`

- **ANDD X**  
  **Behavior**: Performs a bitwise AND between the accumulator (`ac`) and the value at address `X` in the stack.  
  **Pseudo-behavior**: `ac = ac & M[X]`

- **ORD X**  
  **Behavior**: Performs a bitwise OR between the accumulator (`ac`) and the value at address `X` in the stack.  
  **Pseudo-behavior**: `ac = ac | M[X]`

- **XORD X**  
  **Behavior**: Performs a bitwise XOR between the accumulator (`ac`) and the value at address `X` in the stack.  
  **Pseudo-behavior**: `ac = ac ^ M[X]`

- **NOTD X**  
  **Behavior**: Performs a bitwise NOT on the value at address `X` in the stack and stores the result in the accumulator (`ac`).  
  **Pseudo-behavior**: `ac = ~M[X]`

- **SHFRD X**  
  **Behavior**: Shifts the value in the accumulator (`ac`) right by the value at address `X` in the stack.  
  **Pseudo-behavior**: `ac = ac >> M[X]`

- **SHFLD X**  
  **Behavior**: Shifts the value in the accumulator (`ac`) left by the value at address `X` in the stack.  
  **Pseudo-behavior**: `ac = ac << M[X]`

## Debug Operations

- **PRINTLNAC**  
  **Behavior**: Prints the value in the accumulator (`ac`) with a line break.  
  **Pseudo-behavior**: `print( ac + '\n' )`

- **PRINTAC**  
  **Behavior**: Prints the value in the accumulator (`ac`).  
  **Pseudo-behavior**: `print( ac )`

- **PRINTLNSP**  
  **Behavior**: Prints the value in the stack pointer (`sp`) with a line break.  
  **Pseudo-behavior**: `print( sp + '\n' )`

- **PRINTSP**  
  **Behavior**: Prints the value in the stack pointer (`sp`).  
  **Pseudo-behavior**: `print( sp )`

- **PRINTLNTOPI X**  
  **Behavior**: Prints the value at offset `X` from the stack pointer (`sp`) with a line break.  
  **Pseudo-behavior**: `print( M[ sp + X ] + '\n' )`

- **PRINTLNTOPD X**  
  **Behavior**: Prints the value at offset `X` from the stack pointer (`sp`) with a line break.  
  **Pseudo-behavior**: `print( M[ sp + M[X] ] + '\n' )`

- **PRINTTOPI X**  
  **Behavior**: Prints the value at offset `X` from the stack pointer (`sp`).  
  **Pseudo-behavior**: `print( M[ sp + X ] )`

- **PRINTTOPD X**  
  **Behavior**: Prints the value at offset `X` from the stack pointer (`sp`).  
  **Pseudo-behavior**: `print( M[ sp + M[X] ] )`

- **PRINTLNACCHAR**  
  **Behavior**: Prints the value in the accumulator (`ac`) as a character with a line break.  
  **Pseudo-behavior**: `print( char( ac ) + '\n' )`

- **PRINTACCHAR**  
  **Behavior**: Prints the value in the accumulator (`ac`) as a character.  
  **Pseudo-behavior**: `print( char( ac ) )`

- **PRINTLNTOPCHARI X**  
  **Behavior**: Prints the value at offset `X` from the stack pointer (`sp`) as a character with a line break.  
  **Pseudo-behavior**: `print( char( M[ sp + X ] ) + '\n' )`

- **PRINTLNTOPCHARD X**  
  **Behavior**: Prints the value at offset `X` from the stack pointer (`sp`) as a character with a line break.  
  **Pseudo-behavior**: `print( char( M[ sp + M[X] ] ) + '\n' )`

- **PRINTTOPCHARI X**  
  **Behavior**: Prints the value at offset `X` from the stack pointer (`sp`) as a character.  
  **Pseudo-behavior**: `print( char( M[ sp + X ] ) )`

- **PRINTTOPCHARD X**  
  **Behavior**: Prints the value at offset `X` from the stack pointer (`sp`) as a character.  
  **Pseudo-behavior**: `print( char( M[ sp + M[X] ] ) )`

- **PRINTLNINSTRUCTION X**  
  **Behavior**: Prints the instruction hash on line `X` with a line break.
  **Pseudo-behavior**: `print( hash( pc_of_instruction_on_line( X ) ) + '\n' )`

- **PRINTINSTRUCTION X**  
  **Behavior**: Prints the instruction hash on line `X`.
  **Pseudo-behavior**: `print( hash( pc_of_instruction_on_line( X ) ) )`



## Custom Operations

- **HALT**  
  **Behavior**: Stops the execution of the entire program.  
  **Pseudo-behavior**: `end_program`

- **SLEEPD X**  
  **Behavior**: Pauses execution for the number of milliseconds specified by the value at address `X` in the stack.  
  **Pseudo-behavior**: `sleep_in_milliseconds( M[ X ] )`

- **SLEEPI X**  
  **Behavior**: Pauses execution for `X` milliseconds.  
  **Pseudo-behavior**: `sleep_in_milliseconds( X )`

## Important Notes

Here are some key details about the Iguana MAC Interpreter that you should know:

### **Stack Size**
- The interpreter uses a fixed-size stack with a capacity of **32,768 items**. This means that the stack can hold up 32,768 `i16` values at any given time.
- Exceeding this limit will result in a **stack overflow** or **stack pointer out-of-bounds error**.

### **Stack Growth Direction**
- The stack pointer (`sp`) and the stack grow **downward** in memory. This means that as values are pushed onto the stack, the stack pointer decreases, and as values are popped, the stack pointer increases.

### **16-bit Architecture**
- The interpreter operates on a **16-bit architecture**, meaning:
  - The accumulator (`ac`) and all memory values are 16-bit signed integers (`i16`).
  - The valid range for values is **-32,768 to 32,767**.
  - Arithmetic operations that exceed this range will cause an **overflow error**.
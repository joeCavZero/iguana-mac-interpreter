<div align="center">
  <img src="assets/logo.png" width="200" />
</div>

<h1 align="center">IGUANA MAC INTERPRETER</h1>


A simple 16-bit stack-based assembly interpreter for Andrew S. Tanenbaum's MAC assembly language. The interpreter is written in Rust and supports a variety of operations, including arithmetic, memory, control flow, bitwise, and debug operations.

---

# Example Program

The following example demonstrates a simple program that reads a string from the user and prints it to the console.

```python
# program to print a string
.data
    BUFFER: .space 1000 # alocates 1000 bytes in memory
.text
MAIN:
    INPUTSTRING BUFFER  # read a string from the input
    LOCO BUFFER         # ac = BUFFER as a pointer
    SWAP                # ac <-> sp
LOOP:
    LODL 0              # ac = *sp
    JZER END            # if ac == 0 goto END
    PRINTACCHAR         # print ac as a char
    DESP 1              # sp = sp - 1
    JUMP LOOP           # goto LOOP
END:
    HALT                # finishes the program
```

output:
```
Hello, World!
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

1. **If you're using Windows, ensure that you have the Visual C++ Build Tools installed**:
    - You'll need to install the [Visual C++ Build Tools](https://visualstudio.microsoft.com/pt-br/visual-cpp-build-tools/).
    - Select the "Desktop development with C++" workload during installation.
    - Press the "Install" button to install the required components.
    - This is a rust requirement for building the project on Windows.
    - If you're using Linux or MacOS, you can skip this step.
2. **Ensure that you have Rust and Cargo installed**:
    - Follow the installation guide on the official [Rust website](https://www.rust-lang.org/learn/get-started) if needed.
   

3. **Clone the repository**:
    - Use the following command to clone the repository:
      ```bash
      git clone https://github.com/joeCavZero/iguana-mac-interpreter.git
      ```
    - Alternatively, you can download the repository as a ZIP file and extract it.
  
4. **Build the source code**:
   - Run the following command to build the project in release mode:
     ```bash
     cargo build --release
     ```

5. **Locate the generated binary**:
   - After the build is complete, the binary will be located at:
     ```bash
     ./target/release/iguana.exe on Windows 
     ( or iguana.app on MacOS or iguana on Linux )
     ```

6. **Copy the binary to a convenient location**:
   - Copy the binary to a location where you can easily access it, such as a directory in your system's PATH.
   - You can also save the binary in the environment variable PATH, allowing you to run the program from any directory.

---

# **16-bit Architecture**
- The interpreter operates on a **16-bit architecture**, meaning:
  - The accumulator (`ac`) and all memory values are 16-bit signed integers (`i16`).
  - The valid range for values is **-32,768 to 32,767**.
  - Arithmetic operations that exceed this range will cause an **overflow error**.

# **Symbol Table**
- The interpreter uses a symbol table to store the memory addresses and line of label declarations.
<div align="center">
<img src="assets/symbol-table.png" width="500" />
</div>

# **Stack Size**
- The interpreter uses a fixed-size stack with a capacity of **32,768 items**. This means that the stack can hold up 32,768 `i16` values at any given time.
- Exceeding this limit will result in a **stack overflow** or **stack pointer out-of-bounds error**.

# **Stack Growth Direction**
- The stack pointer (`sp`) and the stack grow **downward** in memory. This means that when values are pushed onto the stack, the stack pointer decreases, and when values are popped, the stack pointer increases.
<div align="center">
<img src="assets/data-memory.png" width="500" />
</div>

# **Instruction Memory**
- The interpreter uses a linked list to store the instructions of the program.
- Each instruction is stored in a node with the following fields:
  - `operation`: The operation code of the instruction.
  - `argument`: The argument of the operation.

<div align="center">
<img src="assets/hash.png" width="500" />
</div>

- Each instruction can be viewed as its format with the instructions `PRINTLNINSTRUCTION` and `PRINTINSTRUCTION`. 

<div align="center">
<img src="assets/hash-example.png" width="500" />
</div>

---

# Iguana's MAC Operations Guide
  - There are some new operations that were added to the original MAC assembly language to make the interpreter more debugable and user-friendly.
  - The operations can be divided into the following categories:
    - **Constant Loading**
    - **Memory Operations**
    - **Arithmetic Operations**
    - **Control Flow Operations**
    - **Bitwise Operations**
    - **Debug Operations**
    - **Input Operations**
    - **Custom Operations**

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
  **Pseudo-behavior**: `sp = sp + X`

- **DESP X**  
  **Behavior**: Decrements the stack pointer (`sp`) by `X`.  
  **Pseudo-behavior**: `sp = sp - X`

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
  **Pseudo-behavior**: `sp = sp - 1; M[sp] = pc + 1; pc = pc_of_instruction_on_line( X )`

- **RETN**  
  **Behavior**: Returns from a subroutine by popping the return address from the stack.  
  **Pseudo-behavior**: `pc = M[sp]; sp = sp + 1`

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

- **PRINTLNACCHAR**  
  **Behavior**: Prints the value in the accumulator (`ac`) as a character with a line break.  
  **Pseudo-behavior**: `print( char( ac ) + '\n' )`

- **PRINTACCHAR**  
  **Behavior**: Prints the value in the accumulator (`ac`) as a character.  
  **Pseudo-behavior**: `print( char( ac ) )`

- **PRINTLNINSTRUCTION X**  
  **Behavior**: Prints the instruction format on line `X` with a line break.
  **Pseudo-behavior**: `print( format( pc_of_instruction_on_line( X ) ) + '\n' )`

- **PRINTINSTRUCTION X**  
  **Behavior**: Prints the instruction format on line `X`.
  **Pseudo-behavior**: `print( format( pc_of_instruction_on_line( X ) ) )`


## Input Operations
- **INPUTAC**  
  **Behavior**: Reads an number from the user and stores it in the accumulator (`ac`).  
  **Pseudo-behavior**: `ac = input_number()`

- **INPUTACCHAR**  
  **Behavior**: Reads a character from the user and stores it in the accumulator (`ac`).  
  **Pseudo-behavior**: `ac = number( input_character() )`

- **INPUTSTRING X**  
  **Behavior**: Reads a string from the user and stores it in the memory at address `X` with a null terminator.  
  **Pseudo-behavior**: `M[X] = input_string() + '\0'`

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
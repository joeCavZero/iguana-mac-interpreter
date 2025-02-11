# program to sum two numbers using only bitwise operations
.data
    NUMBER1: .word 25
    NUMBER2: .word 25
    RESULT: .word 0
    CARRY: .word 0
    TEMP: .word 0
.text
MAIN:
    LODD NUMBER1            # ac = *NUMBER1
    STOD RESULT             # *RESULT = ac

SUM_LOOP:
    LODD NUMBER2            # ac = *NUMBER2
    ANDD RESULT             # ac = ac & *RESULT
    STOD CARRY              # *CARRY = ac

    LODD NUMBER2            # ac = *NUMBER2
    XORD RESULT             # ac = ac ^ *RESULT
    STOD RESULT             # *RESULT = ac

    LODD CARRY              # ac = *CARRY
    SHFLI 1                 # ac = ac << 1
    STOD NUMBER2            # *NUMBER2 = ac

    LODD NUMBER2            # ac = *NUMBER2
    JZER END                # if ac == 0 goto END

    JUMP SUM_LOOP           # goto SUM_LOOP

END:
    LODD RESULT             # ac = *RESULT
    PRINTLNAC               # print ac as a number with a newline
    HALT                    # finishes the program
# program to calculate the average of a vector of numbers
.data
    VECTOR: .word 10, 20, 30, 40, 50
    VECTOR_SIZE: .word 5

    VECTOR_POINTER: .word 0
    INDEX: .word 0
    SUM: .word 0

    ONE: .word 1
.text
    LOCO VECTOR             # ac = VECTOR
    STOD VECTOR_POINTER     # *VECTOR_POINTER = ac
LOOP:
    LODD VECTOR_POINTER     # ac = *VECTOR_POINTER
    SWAP                    # ac = sp; sp = ac
    POP                     # ac = VECTOR[i]

    ADDD SUM                # ac = ac + *SUM
    STOD SUM                # *SUM = ac

    LODD INDEX              # ac = *INDEX
    ADDD ONE                # ac = ac + 1
    STOD INDEX              # *INDEX = ac

    LODD VECTOR_POINTER     # ac = *VECTOR_POINTER
    SUBD ONE                # ac = ac - 1
    STOD VECTOR_POINTER     # *VECTOR_POINTER = ac

    LODD VECTOR_SIZE        # ac = *VECTOR_SIZE
    SUBD INDEX              # ac = ac - *INDEX
    JZER END JNEG END       # if ac <= 0 goto END
    JUMP LOOP
END:
    LODD SUM                # ac = *SUM
    DIVD VECTOR_SIZE        # ac = ac / *VECTOR_SIZE
    PRINTLNAC               # print ac as a number with a newline
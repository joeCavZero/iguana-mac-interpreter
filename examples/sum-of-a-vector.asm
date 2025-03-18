# program to sum the elements of a vector
.data
    VECTOR: .word 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
    VECTOR_SIZE: .word 10
    SUM: .word 0
    AUX: .space 2

    ONE: .word 1
    SPACE: .ascii " "
    NEWLINE: .ascii "\n"
.text
    LOCO VECTOR         # ac = VECTOR as a pointer value
    STOD AUX            # *AUX = VECTOR
LOOP:
    LODD AUX            # ac = *AUX
    SWAP                # sp = ac ; ac = sp
    POP                 # ac = VECTOR[i]
    PRINTAC             # print ac as a number
    ADDD SUM            # ac = ac + SUM
    STOD SUM            # *SUM = ac

    LODD SPACE          # ac = *SPACE
    PRINTACCHAR         # print ac as a char

    LODD AUX            # ac = *AUX
    SUBD ONE            # ac = ac - 1
    STOD AUX            # *AUX = ac
    
    LODD VECTOR_SIZE    # ac = *VECTOR_SIZE
    SUBD ONE            # ac = ac - 1
    STOD VECTOR_SIZE    # *VECTOR_SIZE = ac
    JZER END            # if ac == 0 goto END

    JUMP LOOP           # goto LOOP
END:
    LODD NEWLINE        # ac = *NEWLINE
    PRINTACCHAR         # print ac as a char
    LODD SUM            # ac = *SUM
    PRINTLNAC           # print ac as a number with a newline
    HALT                # finishes the program
    

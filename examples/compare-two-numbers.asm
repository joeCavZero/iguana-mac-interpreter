# program to compare 2 numbers (equals), if true print 1, else print 0
.data
    NUMBER1: .word 2
    NUMBER2: .word 5
    
    RESULT: .word 0
.text
    LODD NUMBER1        # ac = *NUMBER1
    SUBD NUMBER2        # ac = ac - *NUMBER2
    JZER EQUAL          # if ac == 0 goto EQUAL
    JNZE NOT_EQUAL      # if ac != 0 goto NOT_EQUAL
EQUAL:
    LOCO 1              # ac = 1
    STOD RESULT         # *RESULT = ac
    JUMP PRINT_AND_END  # goto PRINT_AND_END
NOT_EQUAL:
    LOCO 0              # ac = 0
    STOD RESULT         # *RESULT = ac
    JUMP PRINT_AND_END  # goto PRINT_AND_END
PRINT_AND_END:
    LODD RESULT         # ac = *RESULT
    PRINTLNAC           # print ac as a number with a newline
    HALT                # finishes the program
# program to compare 2 numbers (equals)
# case true print "numbers are equal"
# case false print "numbers are not equal"
.data
    NUMBER1: .word 5        # change this value to test
    NUMBER2: .word 4        # change this value to test

    TRUE_STRING: .asciiz "numbers are equal!"
    FALSE_STRING: .asciiz "numbers are not equal!"

    STRING_POINTER: .word 0
    ONE: .word 1
.text
    LODD NUMBER1            # ac = *NUMBER1
    SUBD NUMBER2            # ac = ac - *NUMBER2
    JZER EQUAL              # if ac == 0 goto EQUAL
    JNZE NOT_EQUAL          # if ac != 0 goto NOT_EQUAL

EQUAL:
    LOCO TRUE_STRING        # ac = TRUE_STRING as a pointer
    STOD STRING_POINTER     # *STRING_POINTER = ac
    JUMP PRINT_AND_END      # goto PRINT_AND_END
NOT_EQUAL:
    LOCO FALSE_STRING       # ac = FALSE_STRING as a pointer
    STOD STRING_POINTER     # *STRING_POINTER = ac
    JUMP PRINT_AND_END      # goto PRINT_AND_END

PRINT_AND_END:
  LOOP:
    LODD STRING_POINTER     # ac = *STRING_POINTER
    SWAP                    # sp = ac ; ac = sp
    POP                     # ac = STRING[i]
    JZER END                # if ac == \0 goto END
    PRINTACCHAR
    LODD STRING_POINTER     # ac = *STRING_POINTER
    SUBD ONE                # ac = ac - 1
    STOD STRING_POINTER     # *STRING_POINTER = ac
    JUMP LOOP               # goto LOOP

END:
    HALT                    # finishes the program
# program to print the reverse of a string
.data
    STRING: .asciiz "Hi John Doe!"
    LAST_CHAR_POINTER: .word 0
    ONE: .word 1

    STRING_PTR: .word 0
.text
    LOCO STRING                     # ac = STRING as a pointer
    STOD LAST_CHAR_POINTER          # *LAST_CHAR_POINTER = STRING
    STOD STRING_PTR                 # *STRING_PTR = STRING
REACH_TO_LAST_CHAR_LOOP:
    LODD LAST_CHAR_POINTER          # ac = *LAST_CHAR_POINTER
    SWAP                            # sp = ac ; ac = sp
    POP                             # ac = string[i]
    JZER PRINT                      # if ac == \0 goto PRINT

    LODD LAST_CHAR_POINTER          # ac = *LAST_CHAR_POINTER
    SUBD ONE                        # ac = ac - 1
    STOD LAST_CHAR_POINTER          # *LAST_CHAR_POINTER = ac
    JUMP REACH_TO_LAST_CHAR_LOOP    # goto REACH_TO_LAST_CHAR_LOOP
PRINT:
    LODD LAST_CHAR_POINTER          # ac = *LAST_CHAR_POINTER
    SUBD STRING_PTR                 # ac = ac - *STRING_PTR (STRING)
    SUBD ONE                        # ac = ac + 1
    JPOS END                        # if ac == 0 goto END

    LODD LAST_CHAR_POINTER          # ac = *LAST_CHAR_POINTER
    SWAP                            # sp = ac ; ac = sp
    POP                             # ac = string[i]

    PRINTACCHAR                     # print ac as a char

    LODD LAST_CHAR_POINTER          # ac = *LAST_CHAR_POINTER
    ADDD ONE                        # ac = ac + 1
    STOD LAST_CHAR_POINTER          # *LAST_CHAR_POINTER = ac

    JUMP PRINT                      # goto PRINT
END:
    HALT                            # finishes the program
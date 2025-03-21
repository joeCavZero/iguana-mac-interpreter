# program to print the reverse of a string
.data
    STRING: .asciiz "Hi John Doe!"
    LAST_CHAR_POINTER: .word 0
    ONE: .word 1

    STRING_PTR: .space 2
.text
    LOCO STRING                     # ac = STRING as a pointer
    STOD LAST_CHAR_POINTER          # *LAST_CHAR_POINTER = STRING
    STOD STRING_PTR                 # *STRING_PTR = STRING
REACH_TO_LAST_CHAR_LOOP:
    LODD LAST_CHAR_POINTER          # ac = *LAST_CHAR_POINTER
    SWAP                            # sp = ac ; ac = sp
    LODL 0                          # ac = string[i]
    JZER PRINT                      # if ac == \0 goto PRINT

    LODD LAST_CHAR_POINTER          # ac = *LAST_CHAR_POINTER
    SUBD ONE                        # ac = ac - 1
    STOD LAST_CHAR_POINTER          # *LAST_CHAR_POINTER = ac
    JUMP REACH_TO_LAST_CHAR_LOOP    # goto REACH_TO_LAST_CHAR_LOOP
PRINT:
    LODD LAST_CHAR_POINTER          # ac = *LAST_CHAR_POINTER
    SUBD STRING_PTR                 # ac = ac - *STRING_PTR (STRING)
    JPOS PRINT_FIRST_CHAR           # if ac == 0 goto PRINT_FIRST_CHAR  

    LODD LAST_CHAR_POINTER          # ac = *LAST_CHAR_POINTER
    SWAP                            # sp = ac ; ac = sp
    LODL 0                          # ac = string[i]

    PRINTACCHAR                     # print ac as a char

    LODD LAST_CHAR_POINTER          # ac = *LAST_CHAR_POINTER
    ADDD ONE                        # ac = ac + 1
    STOD LAST_CHAR_POINTER          # *LAST_CHAR_POINTER = ac

    JUMP PRINT                      # goto PRINT
PRINT_FIRST_CHAR:
    LODD STRING_PTR                 # ac = *STRING_PTR
    SWAP                            # sp = ac ; ac = sp
    LODL 0                          # ac = string[0]
    PRINTACCHAR                     # print ac as a char
    JUMP END                        # goto END
END:
    HALT                            # finishes the program
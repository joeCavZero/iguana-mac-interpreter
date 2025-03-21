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
    LODL 0              # ac = *sp
    JZER END            # if ac == 0 goto END
    PRINTACCHAR         # print ac as a char
    LODD CHAR_POINTER   # ac = *CHAR_POINTER
    SUBD ONE            # ac = ac - 1
    STOD CHAR_POINTER   # *CHAR_POINTER = ac
    JUMP LOOP           # goto LOOP
END:
    HALT                # finishes the program
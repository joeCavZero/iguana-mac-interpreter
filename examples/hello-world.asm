# program to print a string
.data
    STRING: .asciiz "Hello, World!" # alocates a string in memory
.text
MAIN:
    LOCO STRING         # ac = STRING as a pointer
    SWAP                # ac <-> sp
LOOP:
    LODL 0              # ac = *sp
    JZER END            # if ac == 0 goto END
    PRINTACCHAR         # print ac as a char
    DESP 1              # sp = sp - 1
    JUMP LOOP           # goto LOOP
END:
    HALT                # finishes the program
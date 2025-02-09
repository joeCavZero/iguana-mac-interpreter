.data
STRING: .asciiz "Hello, World!"
AUX: .word 0
ONE: .word 1
.text
MAIN:
    LOCO STRING     # ac = STRING as a pointer
    STOD AUX        # *AUX = STRING
LOOP:
    LODD AUX        # ac = *AUX
    SWAP            # sp = ac ; ac = sp
    POP             # ac = *sp ; sp = sp + 1 (decrement sp)
    JZER END        # if ac == 0 goto END
    PRINTACCHAR     # print ac as a char
    LODD AUX        # ac = *AUX
    SUBD ONE        # ac = ac - 1
    STOD AUX        # *AUX = ac
    JUMP LOOP       # goto LOOP
END:
    HALT            # finishes the program
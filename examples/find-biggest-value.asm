# program to find the biggest value between two numbers
.data
VAL1: .word 10
VAL2: .word 257
BIGGER: .word 0

.text
    LODD VAL1
    SUBD VAL2
    JPOS LABEL1

    LODD VAL2 
    STOD BIGGER
    JUMP PRINT_AND_END

LABEL1:
    LODD VAL1  
    STOD BIGGER   

PRINT_AND_END:
    LODD BIGGER
    PRINTLNAC
    HALT

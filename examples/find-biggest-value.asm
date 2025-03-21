# program to find the biggest value between two numbers
.data
VAL1: .word 100
VAL2: .word 1000
BIGGER: .space 2

.text
    LODD VAL1
    SUBD VAL2
    JPOS VAL1_IS_BIGGER

    LODD VAL2 
    STOD BIGGER
    JUMP PRINT_AND_END

VAL1_IS_BIGGER:
    LODD VAL1  
    STOD BIGGER   

PRINT_AND_END:
    LODD BIGGER
    PRINTLNAC
    HALT

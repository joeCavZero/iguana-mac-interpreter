#programa de achar o BIGGER valor entre VAL1 e VAL2
.data
VAL1: .word 256
VAL2: .word 10
BIGGER: .word 0

.text
    LODD VAL1
    SUBD VAL2
    JPOS LABEL1

    LODD VAL2 
    STOD BIGGER
    JUMP IMPRIMIR

LABEL1:
    LODD VAL1  
    STOD BIGGER   

PRINT_AND_END:
    LODD BIGGER
    PRINTLNAC
    HALT

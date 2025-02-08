.data
    UM: .word 1
.text
    LOCO 10
LOOP:
    SUBD UM
    PRINTLNAC
    JUMP LOOP
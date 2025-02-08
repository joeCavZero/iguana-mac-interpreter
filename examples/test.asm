.data
    TWO: .word 2
    AUX: .word 0
.text
    LOCO 2
LOOP:
    MULD TWO
    STOD AUX
    PRINTLNAC
    SLEEPD TWO
    JUMP LOOP

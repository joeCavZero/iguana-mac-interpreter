.data
    COMMA: .ascii ","
    ONE: .word 0
    COUNTER: .word 0
.text
LOOP:
    PRINTAC
    LODD COUNTER
    ADDD ONE
    PRINTAC
    STOD COUNTER
    LODD COMMA
    PRINTACCHAR
    SLEEPI 1
    JUMP LOOP

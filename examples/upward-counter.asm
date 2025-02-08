# program to print an upward counter
.data
    COMMA: .ascii ","
    ONE: .word 1
    COUNTER: .word 0
    MAX: .word 5
.text
LOOP:
    LODD COUNTER
    PRINTAC
    ADDD ONE
    STOD COUNTER
    LODD COMMA
    PRINTACCHAR
    SLEEPD ONE

    LODD MAX
    SUBD COUNTER

    JZER END
    JNEG END

    JUMP LOOP
END:
    HALT

# program of fibonacci series
.data
    MAX: .word 32000
    A: .word 0
    B: .word 1
    TEMP: .word 0

    COMMA: .ascii ","
.text

FIBONACCI_LOOP:
    LODD MAX            # ac = *MAX
    SUBD A              # ac = ac - *A
    SUBD B              # ac = ac - *B
    JZER END JNEG END   # if ac <= 0 goto END

    LODD A              # ac = *A
    STOD TEMP           # *TEMP = ac
    LODD A              # ac = *A
    ADDD B              # ac = ac + *B
    STOD A              # *A = ac
    LODD TEMP           # ac = *TEMP
    STOD B              # *B = ac
     LODD A             # ac = *A
     PRINTAC            # print ac as a number
     LODD COMMA         # ac = *COMMA
     PRINTACCHAR        # print ac as a char
    JUMP FIBONACCI_LOOP # goto FIBONACCI_LOOP

END:
    HALT                # finishes the program
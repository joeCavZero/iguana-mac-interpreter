# program to print a factorial of a initial value
.data
    INITIAL_VALUE: .word 7 # only works for values less than 8 because of the 16-bit limit
    AUX: .word 1

    ONE: .word 1
.text
LOOP:
    LODD INITIAL_VALUE      # ac = *INITIAL_VALUE
    JZER END                # if ac == 0 goto END

    LODD AUX                # ac = *AUX
    MULD INITIAL_VALUE      # ac = ac * *INITIAL_VALUE
    STOD AUX                # *AUX = ac

    LODD INITIAL_VALUE      # ac = *INITIAL_VALUE
    SUBD ONE                # ac = ac - 1
    STOD INITIAL_VALUE      # *INITIAL_VALUE = ac

    JUMP LOOP               # goto LOOP
END:
    LODD AUX                # ac = *AUX
    PRINTLNAC               # print ac as a number with a newline
    HALT                    # finishes the program

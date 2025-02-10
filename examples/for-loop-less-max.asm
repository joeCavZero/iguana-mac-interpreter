# program that simulates a for loop 
# [ for(int i = INITIAL; i < MAX; i += INCREMENT ) ]
.data
    INITIAL: .word 1
    MAX: .word 10
    INCREMENT: .word 1

    I: .word 0

    COMMA: .ascii ","
.text

FOR_LOOP:
    LODD INITIAL            # ac = *INITIAL
    STOD I                  # *I = ac
  LOOP:
    LODD MAX                # ac = *MAX
    SUBD I                  # ac = ac - *I
    JNEG END JZER END       # if ac < 0 goto END
      LODD I                # ac = *I
      PRINTAC               # print ac as a number
      LODD COMMA            # ac = *COMMA
      PRINTACCHAR           # print ac as a char
    LODD I                  # ac = *I
    ADDD INCREMENT          # ac = ac + *INCREMENT
    STOD I                  # *I = ac

    JUMP LOOP               # goto LOOP

END:
    HALT                    # finishes the program
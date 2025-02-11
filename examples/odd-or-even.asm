# program to check if a number is odd or even
.data
VALUE: .word 12   
RESULT: .word 0 

.text
    LODD VALUE    # load the value to be checked
    ANDI 1        # realize the bitwise AND operation with 1 (0b0001)
    JZER EVEN     # if the result is zero, jump to EVEN
    JNZE ODD      # else, jump to ODD
EVEN:
    LOCO 0        # load 0 to the accumulator (ac)
    STOD RESULT   # store the ac in the result
    JUMP END      # jump to END      
ODD:
    LOCO 1        # load 1 to the ac
    STOD RESULT   # store the ac in the result
    JUMP END      # jump to END
END:
    LODD RESULT   # load the result
    PRINTLNAC     # print the result
    HALT          # halt the program
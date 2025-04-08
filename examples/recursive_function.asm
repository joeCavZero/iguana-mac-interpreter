# THIS PROGRAM IMPLEMENTS A RECURSIVE FUNCTION
# THAT USE THE STACK TO STORE THE FUNCTION ARG
# AND THE RETURNED VALUE
.data
    ONE: .word 0b01
    MAX: .word 200
    INCR: .word 5
    NEW_LINE: .ascii "\n"
.text
    DESP 1 # increases one space to store the function argument
    LOCO 0 # this number is the function argument
    STOL 0 # store the function argument
    CALL RECURSIVE_FUNCTION
    LODD NEW_LINE PRINTACCHAR
    LODL -2
    PRINTLNAC
    HALT
RECURSIVE_FUNCTION:
    DESP 1
    # [ARG, RTN, ~]
    LODL 2
    STOL 0
    ADDD INCR
    STOL 0
    # [ARG, RTN, ARG+INCR]
    LODL 0
    PRINTLNAC SLEEPI 100
    SUBD MAX
    JPOS RECURSIVE_FUNCTION_REGRESS
    CALL RECURSIVE_FUNCTION
  RECURSIVE_FUNCTION_REGRESS:
    # [ARG, RTN, RESULT]
    LODL 0
    STOL 2
    INSP 1
    # [RESULT, RTN]
    RETN
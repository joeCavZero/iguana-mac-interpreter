# THIS PROGRAM IMPLEMENTS:
#   RECURSIVE FUNCTION
#   FINABOCCI
#   PRINT_STRING
# EACH ARE STACK BASED FUNCTIONS
.data
    RECURSIVE_FUNCTION_STR_1: .asciiz "=== RECURSIVE FUNCTION ===\n"
    RECURSIVE_FUNCTION_STR_2: .asciiz "Iteration value: "
    RECURSIVE_FUNCTION_STR_3: .asciiz "Final values: "

    FIBONACCI_STR_1: .asciiz "=== FIBONACCI ===\n"
    FIBONACCI_STR_2: .asciiz "Fibonacci: "
    FIBONACCI_STR_3: .asciiz "The last fibonacci is: "

    NEW_LINE: .ascii "\n"
.text
    #=== RECURSIVE FUNCTION ===
    # Print recursive function header
    LOCO RECURSIVE_FUNCTION_STR_1 PUSH
    CALL PRINT_STRING
    DESP 1
    #

    # Push the param and call the recursive function
    LOCO 0 PUSH # Push the param that can be any initial number
    CALL RECURSIVE_FUNCTION
    # Print returned value of recursive function
    LOCO RECURSIVE_FUNCTION_STR_3 PUSH
    CALL PRINT_STRING
    DESP 1
    LODL 0 PRINTLNAC
    LODD NEW_LINE PRINTACCHAR
    #

    #=== FIBONACCI ===
    PUSH # push trash that can be used as a return value
    # ( FIBONACCI DOES NOT USES ARGUMENTS )
    CALL FIBONACCI
    # Print
    LOCO FIBONACCI_STR_3 PUSH
    CALL PRINT_STRING
    DESP 1
    LODL 0
    PRINTLNAC
    #
    
    HALT

FIBONACCI:
    # Print
    LOCO FIBONACCI_STR_1 PUSH
    CALL PRINT_STRING
    DESP 1
    #
    # Variables
    LOCO 0 PUSH
    LOCO 1 PUSH
    PUSH # Push trash
    LOCO 3000 PUSH
    #
  FIBONACCI_LOOP:
    LODL 3
    ADDL 2
    STOL 1

    LODL 2 STOL 3
    LODL 1 STOL 2
    # Print
    LOCO FIBONACCI_STR_2 PUSH
    CALL PRINT_STRING
    DESP 1
    #
    LODL 2
    PRINTLNAC

    LODL 0
    SUBL 2
    JNEG FIBONACCI_END

    JUMP FIBONACCI_LOOP
  FIBONACCI_END:
    LODL 2
    DESP 4
    STOL 1
    RETN

RECURSIVE_FUNCTION:
    # Variables
    LODL 1 PUSH
        #[ ARG, RTN, ARG]
    LOCO 1000 PUSH # this is the max
    LOCO 5 PUSH # this is the increment after each call
        #[ ARG, RTN, ARG, 1000, 5 ]
    #
    LODL 2
    ADDL 0
    STOL 2
    # Print
    LOCO RECURSIVE_FUNCTION_STR_2 PUSH
    CALL PRINT_STRING
    DESP 1
    #
        # [ARG, RTN, ARG+5, 1000, 5]
    LODL 2
    PRINTLNAC
    SUBL 1
    DESP 2
        # [ARG, RTN, ARG+5]
    JPOS RECURSIVE_FUNCTION_REGRESS
    CALL RECURSIVE_FUNCTION
  RECURSIVE_FUNCTION_REGRESS:
        # [ARG, RTN, LAST_RESULT]
    LODL 0
    STOL 2
    DESP 1
        # [RESULT, RTN]
    RETN



PRINT_STRING:
    LODL 1
    INSP 1
    STOL 0
    #[ ARG, RETN, AUX_ARG ]
  PRINT_STRING_LOOP:
    LODL 0
    PSHI
    #[ ARG, RETN, AUX_ARG, CHAR ]
    LODL 0
    JZER PRINT_STRING_END
    PRINTACCHAR
    LOCO 1
    PUSH
    #[ ARG, RETN, AUX_ARG, CHAR, 1 ]
    LODL 2
    SUBL 0
    STOL 2
    DESP 2
    JUMP PRINT_STRING_LOOP
  PRINT_STRING_END:
    DESP 2
    RETN
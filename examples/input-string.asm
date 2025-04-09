# program to read a string and print it
.data
    STR1: .asciiz "--> Enter your name: "
    STR2: .asciiz "Hello, "
    BUFF: .space 1000
.text
    LOCO STR1 PUSH
    CALL PRINT_STRING

    INPUTSTRING BUFF

    LOCO STR2 PUSH
    CALL PRINT_STRING

    LOCO BUFF PUSH
    CALL PRINT_STRING

    HALT
PRINT_STRING:
    LODL 1
    DESP 1
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
    INSP 2
    JUMP PRINT_STRING_LOOP
  PRINT_STRING_END:
    INSP 2
    RETN
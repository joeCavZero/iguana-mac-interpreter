.data
    STR1: .asciiz "Hi Mom!\n"
    STR2: .asciiz "Ola Mae!\n"
    STR3: .asciiz "Konnichiwa Okaasan!\n"
.text
    CALL MAIN
    HALT
MAIN:
    LOCO STR1 PUSH
    CALL PRINT_STRING
    INSP 1
    LOCO STR2 PUSH
    CALL PRINT_STRING
    INSP 1
    LOCO STR3 PUSH
    CALL PRINT_STRING
    INSP 1
    RETN
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
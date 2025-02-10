# program that simulates a while loop:
#   while( CONDITIONAL == true ) { 
#       print I + ","
#   }
.data
    CONDITIONAL: .word 1    # 0 for false, else true
    I: .word 'A'            # you can use .word for chars
    COMMA: .ascii ","       # you can use .ascii for strings
.text
WHILE_LOOP:
  LODD CONDITIONAL      # ac = *CONDITIONAL
  JZER BREAK            # if ac == 0 goto BREAK
    LODD I              # ac = *I
    PRINTACCHAR         # print ac as a char
    LODD COMMA          # ac = *COMMA
    PRINTACCHAR         # print ac as a char
  JUMP WHILE_LOOP       # goto WHILE_LOOP

BREAK:
    HALT                # finishes the program
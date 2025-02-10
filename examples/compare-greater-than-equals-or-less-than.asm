# program to print a string if number1 is greater than, equal to or less than number2
.data
    NUMBER1: .word 6
    NUMBER2: .word 5

    STRING1: .asciiz "Number1 is greater than Number2"
    STRING2: .asciiz "Number1 is equal to Number2"
    STRING3: .asciiz "Number1 is less than Number2"

    STRING_POINTER: .word 0
    ONE: .word 1
.text
    LODD NUMBER1            # ac = *NUMBER1
    SUBD NUMBER2            # ac = ac - *NUMBER2
    JZER EQUAL              # if ac == 0 goto EQUAL
    JPOS GREATER            # if ac > 0 goto GREATER
    JNEG LESS               # if ac < 0 goto LESS
GREATER:
    LOCO STRING1            # ac = STRING1 as a pointer
    STOD STRING_POINTER     # *STRING_POINTER = ac
    JUMP PRINT_AND_END      # goto PRINT_AND_END
EQUAL:
    LOCO STRING2            # ac = STRING2 as a pointer
    STOD STRING_POINTER     # *STRING_POINTER = ac
    JUMP PRINT_AND_END      # goto PRINT_AND_END
LESS:
    LOCO STRING3            # ac = STRING3 as a pointer
    STOD STRING_POINTER     # *STRING_POINTER = ac
    JUMP PRINT_AND_END      # goto PRINT_AND_END

PRINT_AND_END:
  LOOP:
    LODD STRING_POINTER     # ac = *STRING_POINTER
    SWAP                    # sp = ac ; ac = sp
    POP                     # ac = STRING[i]
    JZER END                # if ac == \0 goto END
    PRINTACCHAR
    LODD STRING_POINTER     # ac = *STRING_POINTER
    SUBD ONE                # ac = ac - 1
    STOD STRING_POINTER     # *STRING_POINTER = ac
    JUMP LOOP               # goto LOOP
    
END:
    HALT                    # finishes the program
# program implementing a function that print a string
# the function receives a pointer to the string to be printed (PRINT_STRING_POINTER)
# the function uses the stack to store the return address (FUNCTION_RETURN_ADDRESS)

.data
    PRINT_STRING_POINTER: .word 0
    FUNCTION_RETURN_ADDRESS: .word 0

    STR1: .asciiz "Hi everyone!\n"
    STR2: .asciiz "My name is Joe!\n"
    STR3: .asciiz "And I'm a programmer!\n"

    ONE: .word 1

.text
    LOCO STR1                       # ac = STR1 as a pointer
    STOD PRINT_STRING_POINTER       # *PRINT_STRING_POINTER = ac (STR1)
    CALL PRINT_STRING               # call the function PRINT_STRING

    LOCO STR2                       # ac = STR2 as a pointer
    STOD PRINT_STRING_POINTER       # *PRINT_STRING_POINTER = ac (STR2)
    CALL PRINT_STRING               # call the function PRINT_STRING

    LOCO STR3                       # ac = STR3 as a pointer
    STOD PRINT_STRING_POINTER       # *PRINT_STRING_POINTER = ac (STR3)
    CALL PRINT_STRING               # call the function PRINT_STRING

    JUMP END_PROGRAM                # jump to END_PROGRAM

END_PROGRAM:
    HALT                            # finishes the program

PRINT_STRING:
    LODL 0                          # ac = *PRINT_STRING_POINTER
    STOD FUNCTION_RETURN_ADDRESS    # *FUNCTION_RETURN_ADDRESS = ac
  PRINT_STRING_LOOP:        
    LODD PRINT_STRING_POINTER       # ac = *PRINT_STRING_POINTER
    SWAP                            # sp = ac ; ac = sp
    LODL 0                          # ac = STRING[i]

    JZER PRINT_STRING_END           # if ac == 0 goto PRINT_STRING_END
    PRINTACCHAR                     # print ac as a char
    LODD PRINT_STRING_POINTER       # ac = *PRINT_STRING_POINTER
    SUBD ONE                        # ac = ac - 1
    STOD PRINT_STRING_POINTER       # *PRINT_STRING_POINTER = ac
    JUMP PRINT_STRING_LOOP          # goto PRINT_STRING_LOOP

  PRINT_STRING_END:
    LOCO FUNCTION_RETURN_ADDRESS    # ac = FUNCTION_RETURN_ADDRESS
    SWAP                            # sp = FUNCTION_RETURN_ADDRESS ; ac = sp

    RETN                            # return to the address in the top of the stack

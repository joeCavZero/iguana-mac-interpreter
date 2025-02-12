# program tha sum two numbers (one in hex and other in binary) using call, retn and stack operations
# note:
#   you cannot write negative numbers in binary or hexadecimal
#   in this ISA, but you can write negative numbers in decimal
.data
    NUMBER1: .word 0x21     # 33 in decimal
    NUMBER2: .word 0b1011   # 11 in decimal
.text
    LODD NUMBER1            # load NUMBER1 to the accumulator
    PUSH                    # push the accumulator to the stack
    LODD NUMBER2            # load NUMBER2 to the accumulator
    PUSH                    # push the accumulator to the stack
    # in this moment the stack is something like this: [ NUMBER1, NUMBER2 ]
    CALL ADD_TWO_NUMBERS    # call the function ADD_TWO_NUMBERS (the return address is pushed to the stack)
    # in this moment the stack is something like this: [ RESULT, NUMBER2 ]
    DESP 1                  # decrement the stack pointer 1 position
    # in this moment the stack is something like this: [ RESULT ]
    CALL PRINT              # call the function PRINT (the return address is pushed to the stack)
    JUMP END                # jump to END

ADD_TWO_NUMBERS:
    # in this moment the stack is something like this: [ NUMBER1, NUMBER2, RETURN_ADDRESS ]
    LODL 1                  # load NUMBER1 to the accumulator
    ADDL 2                  # ac = ac + M[sp + 2] (NUMBER2)
    STOL 2                  # store the result in the position M[sp + 2]
    RETN                    # return to the address in the top of the stack

PRINT:
    # in this moment the stack is something like this: [ RESULT, RETURN_ADDRESS ]
    LODL 1                  # load the result to the accumulator
    PRINTLNAC               # print the accumulator as a number
    RETN                    # return to the address in the top of the stack

END:
    HALT                    # finishes the program
# program to print numbers from 0 to 100 that are not equal to a specific number
.data
NUMBERS: .word 0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,64,65,66,67,68,69,70,71,72,73,74,75,76,77,78,79,80,81,82,83,84,85,86,87,88,89,90,91,92,93,94,95,96,97,98,99,100
REACH: .word 55
AUX: .word 0

COMMA: .ascii ","
ONE: .word 1

.text
MAIN:
    LOCO NUMBERS    # ac = NUMBERS as a pointer value
    STOD AUX        # *AUX = NUMBERS
LOOP:
    LODD AUX        # ac = *AUX
    SWAP            # sp = ac ; ac = sp
    POP             # ac = numbers[i]

    SUBD REACH      # ac = ac - *REACH
    JZER END        # if ac == 0 goto END

    LODD AUX        # ac = *AUX
    SWAP            # sp = ac ; ac = sp
    POP             # ac = numbers[i]

    PRINTAC         # print ac as a number
    LODD COMMA      # ac = *COMMA
    PRINTACCHAR     # print ac as a char
    
    LODD AUX        # ac = *AUX
    SUBD ONE        # ac = ac - 1
    STOD AUX        # *AUX = ac
    JUMP LOOP       # goto LOOP
END:
    HALT            # finishes the program
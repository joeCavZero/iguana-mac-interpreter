# program to show how to use branch instructions with literals values
# JUMPs, JZERs, etc... are used to jump based on operation indexes
# and not text editor line numbers
.data
STRING: .asciiz "JOAO"
AUX: .word 0

ONE: .word 1
NEWLINE: .ascii "\n"

.text
    LOCO STRING     # operation with index 0
    SWAP            # operation with index 1
    POP             # operation with index 2
    PRINTACCHAR     # operation with index 3
    
    LOCO STRING     # operation with index 4
    SUBD ONE        # operation with index 5
    SWAP            # operation with index 6
    POP             # ...
    PRINTACCHAR

    LOCO STRING
    SUBD ONE SUBD ONE
    SWAP
    POP
    PRINTACCHAR

    LOCO STRING
    SUBD ONE SUBD ONE SUBD ONE
    SWAP
    POP
    PRINTACCHAR

    LOCO STRING
    SUBD ONE SUBD ONE SUBD ONE SUBD ONE
    SWAP
    POP
    PRINTACCHAR
    
    LODD NEWLINE
    PRINTACCHAR

    SLEEPI 1        # sleep thread for 1 second

    JUMP 0
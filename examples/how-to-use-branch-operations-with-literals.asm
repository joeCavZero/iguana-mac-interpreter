# program to show how to use branch instructions with literals values
# JUMPs, JZERs, etc... are used to jump based on operation indexes
# and not text editor line numbers
.data
STRING: .asciiz "John"
AUX: .word 0

ONE: .word 1
NEWLINE: .ascii "\n"

.text
    LOCO STRING     # operation at line 12
    SWAP            # operation at line 13
    POP             # operation at line 14
    PRINTACCHAR     # operation at line 15
    
    LOCO STRING     # operation at line 17
    SUBD ONE        # [...]
    SWAP            
    POP             
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

    SLEEPI 1000        # sleep thread for 1 second

    JUMP 12          # jump to operation with index 0
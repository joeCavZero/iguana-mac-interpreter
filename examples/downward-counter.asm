# programa que imprime os números de MAX a 1, separados por vírgula
.data
COMMA: .ascii ","
MAX: .word 10
ONE: .word 1

.text
    LODD MAX
LOOP:
    PRINTAC
    SUBD ONE
    STOD MAX
    LODD COMMA
    PRINTACCHAR
    LODD MAX
    JNEG END
    JUMP LOOP
END: HALT
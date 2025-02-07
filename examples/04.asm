#programa de achar o maior valor entre VALOR1 e VALOR2
.data
VALOR1: .word 256
VALOR2: .word 10
MAIOR: .word 0

.text
    LODD VALOR1 #comentario
    SUBD VALOR2
    JPOS LABEL1

    LODD VALOR2 
    STOD MAIOR
    JUMP IMPRIMIR

LABEL1:
    LODD VALOR1  
    STOD MAIOR   

IMPRIMIR:
    LODD MAIOR
    PRINTAC
    HALT

.data
VALOR1: .word 255     # Pode testar com outros valores, como 255
RESULTADO: .word 0
UM: .word 1

.text
    LODD VALOR1       # Carrega VALOR1 na acumuladora (ac)
    ANDD 1            # Realiza um AND com 1, para verificar o bit menos significativo (LSB)
    PRINTAC
    JNZE IMPAR           # Se o LSB for 0, é par (o resultado de ANDD 1 é 0)
    JZER PAR
PAR:
    LOCO 0
    STOD RESULTADO
    JUMP FIM
IMPAR:
    LOCO 1
    STOD RESULTADO
    JUMP FIM
FIM:
    LODD RESULTADO
    PRINTAC
    HALT

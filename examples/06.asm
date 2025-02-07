.data
VALOR1: .word 255     # Pode testar com outros valores, como 255
RESULTADO: .word 0

.text
    LODD VALOR1       # Carrega VALOR1 na acumuladora (ac)
    PRINTAC           # Imprime o valor de VALOR1 para depuração
    ANDD 1            # Realiza um AND com 1, para verificar o bit menos significativo (LSB)
    PRINTAC
    JZER PAR           # Se o LSB for 0, é par (o resultado de ANDD 1 é 0)
    
    LODD 1            # Se for ímpar, carrega 1 na acumuladora
    STOD RESULTADO    # Armazena 1 em RESULTADO
    JUMP FIM          # Pula a parte de paridade

PAR:
    LODD 0            # Se for par, carrega 0 na acumuladora
    STOD RESULTADO    # Armazena 0 em RESULTADO

FIM:
    LODD RESULTADO    # Carrega o valor de RESULTADO
    PRINTAC           # Imprime o valor de RESULTADO (0 ou 1)
    HALT              # Fim do programa

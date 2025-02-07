# programa de verificar se VALOR1 é par ou ímpar
.data
VALOR1: .word 12   # Número a ser verificado
RESULTADO: .word 0 # Resultado (0 para par, 1 para ímpar)

.text
    LODD VALOR1      # Carrega VALOR1 no acumulador
    ANDI 1           # Faz a operação AND com 1
    JZER PAR         # Se o resultado for 0 (par), pula para PAR
    JNZE IMPAR       # Se o resultado for 1 (ímpar), pula para IMPAR
PAR:
    LOCO 0           # Se o número for par, armazena 0 em RESULTADO
    STOD RESULTADO
    JUMP FIM         # Pula para o fim
IMPAR:
    LOCO 1           # Se o número for ímpar, armazena 1 em RESULTADO
    STOD RESULTADO
    JUMP FIM
FIM:
    LODD RESULTADO   # Carrega o valor de RESULTADO em ac
    PRINTLNAC          # Imprime o valor no acumulador
    HALT             # Encerra o programa
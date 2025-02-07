# programa de verificar se VALOR1 é par ou ímpar
.data
VALOR1: .word 12   # Número a ser verificado
RESULTADO: .word 0 # Resultado (0 para par, 1 para ímpar)

.text
    LODD VALOR1      # Carrega VALOR1 no acumulador
    ADDD 0           # Adiciona 0 (não faz alteração no valor, apenas mantém)
    SUBD 1           # Subtrai 1 (isso simula uma operação de AND, apenas para verificação)
    JZER PAR         # Se o resultado for 0 (par), pula para PAR

    LODD 1           # Se o número for ímpar, armazena 1 em RESULTADO
    STOD RESULTADO
    JUMP IMPRIMIR    # Pula para a impressão

PAR:
    LODD 0           # Se o número for par, armazena 0 em RESULTADO
    STOD RESULTADO

IMPRIMIR:
    LODD RESULTADO   # Carrega o valor de RESULTADO (0 ou 1)
    PRINTAC          # Imprime o valor no acumulador
    HALT             # Encerra o programa
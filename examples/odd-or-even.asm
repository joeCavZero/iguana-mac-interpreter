# program to check if a number is odd or even
.data
VALOR1: .word 12   
RESULTADO: .word 0 

.text
    LODD VALOR1    
    ANDI 1         
    JZER PAR       
    JNZE IMPAR     
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
    PRINTLNAC       
    HALT            
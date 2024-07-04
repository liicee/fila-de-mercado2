fila = ['alice', 'luan', 'maju','ana']


while True :    
    n = str(input(" digite um nome: "))
    b = str(input(" Deseja finalizar sua lista? (sim ou não): "))
    fila.append(n)
    if b == "sim":
        break
    if n in fila:
        fila.pop(0)
print(fila)

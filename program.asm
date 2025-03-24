mov 1 x1
loop:
    add 1
    teq acc 10
    + jmp end
    jmp double
double:
    mov acc x2
    mov x1 acc
    mul 2
    mov acc x1
    mov x2 acc
    jmp loop
end:
    nop
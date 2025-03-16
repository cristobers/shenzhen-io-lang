# Works out the nth Fibbonacci number.

mov 1 x1
mov 1 x2
loop:
    add 1
    teq acc 2 
    + jmp end
    mov acc x3

    # fibb
    mov 0 acc
    add x1
    add x2
    mov x2 x1
    mov acc x2

    # reset acc
    mov x3 acc
    jmp loop
end:
    nop
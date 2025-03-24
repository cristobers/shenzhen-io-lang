mov 1 x1
mov 1 x2
loop:
    add 1
    # 0 - 9 gives us 10 fibbonacci numbers.
    teq acc 9
    - jmp fibb
    + jmp end
fibb:
    # save our acc state to x3
    mov acc x3

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
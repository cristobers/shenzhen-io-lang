loop:
    add 1
    teq acc 10
    + jmp end
    jmp loop
end:
    nop
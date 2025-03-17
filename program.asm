mov 5 x1
loop:
    add 1
    tlt acc x1
    - jmp end
    jmp loop
end:
    nop
# shenzhen-io-lang

This is a WIP interpreter for the pseudo-assembly language featured in the game
Shenzhen I/O.

```asm
mov 1 x1
mov 1 x2
loop:
    add 1
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
```

| Implemented |
| ----------- |
| mov         |
| add         |
| mul         |
| div         |
| sub         |
| not         |
| jmp         |
| teq         |
| tgt         |
| tlt         |

For executing, the instructions can take in one or many of these arguments.

```
R - Register
L - Label
I - Integer
```
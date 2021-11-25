extern printf

section .data
    fmt: db "%s", 10, 0
    msg: db "Hello, world!", 0

section .text
    global main

main:
    push rbp
    mov rdi, fmt
    mov rsi, msg
    mov rax, 0
    call printf
    pop rbp
    mov rax, 0
    ret

bits 64
global _start
section .text

    _start:
    push rbp
    mov rbp, rsp
    sub rsp, 64


    mov rax, [rbp - 16]
    mov [rbp - 8], rax

    mov rax, [rbp - 8]
    mov [rbp - 24], rax

    L0_loop:

    mov rax, [rbp - 24]
    test rax, rax
    je L0

    mov rax, 1
    mov [rbp - 32], rax

    jmp L0_loop

    L0:

    ; exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall


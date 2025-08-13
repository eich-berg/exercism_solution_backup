section .text
global leap_year
leap_year:

    mov rcx, 4
    call mod
    jnz .not_leap

    mov rcx, 400
    call mod
    jz .leap

    mov rcx, 100
    call mod
    jz .not_leap

    .leap:
    mov rax, 1
    ret

    .not_leap:
    mov rax, 0
    ret

    mod:
    xor rdx, rdx
    mov rax, rdi
    div rcx
    cmp rdx, 0
    ret


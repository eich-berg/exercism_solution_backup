section .text
global leap_year
leap_year:
    mov rax, rdi
    xor rdx, rdx
    mov rcx, 4
    div rcx
    test rdx, rdx
    jnz .false

    mov rax, rdi
    xor rdx, rdx
    mov rcx, 100
    div rcx
    test rdx, rdx
    jnz .true

    mov rax, rdi
    xor rdx, rdx
    mov rcx, 400
    div rcx
    test rdx, rdx
    jnz .false

    mov eax, 1
    ret

    .false:
    xor eax, eax
    ret
    
    .true:
    mov eax, 1
    ret



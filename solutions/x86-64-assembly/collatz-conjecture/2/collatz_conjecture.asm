section .text
global steps
steps:    

    xor rax, rax
    cmp edi, 0
    jle .error

    .collatz_loop:

    cmp rdi, 1
    je .done
    inc rax

    test rdi, 1
    jz .even
    lea rdi, [rdi + 2*rdi + 1]
    jmp .collatz_loop
    
    .even:
    shr rdi, 1
    jmp .collatz_loop

    .error:
    mov rax, -1

    .done:
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
    

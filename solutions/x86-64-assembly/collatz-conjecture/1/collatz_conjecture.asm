section .text
global steps
steps:
    ; Provide your implementation here
    mov eax, 0

    test edi, edi
    jle .error  

    cmp rdi, 1
    je .done
    
    .collatz_loop:
    inc eax
    
    test rdi, 1
    jz .even
    lea rdi, [rdi + rdi*2 + 1]
    jmp .check_done

    .even:
    shr rdi, 1
    
    .check_done:
    cmp rdi, 1
    jne .collatz_loop

    .done:
    ret

    .error:
    mov eax, -1
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
    

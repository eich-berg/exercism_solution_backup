section .text
global square_of_sum
square_of_sum:
    mov rax, rdi
    inc rdi
    imul rax, rdi
    shr rax, 1
    imul rax, rax
    ret
        
global sum_of_squares
sum_of_squares:
    mov rax, rdi
    mov rcx, rdi
    inc rcx
    
    imul rax, rcx

    lea rdx, [rdi*2+1]

    imul rax, rdx
    
    xor rdx, rdx
    mov rcx, 6
    div rcx
    ret

global difference_of_squares
difference_of_squares:
    push rdi
    call square_of_sum
    mov rbx, rax

    pop rdi
    push rbx
    call sum_of_squares

    pop rbx
    sub rbx, rax
    mov rax, rbx
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif

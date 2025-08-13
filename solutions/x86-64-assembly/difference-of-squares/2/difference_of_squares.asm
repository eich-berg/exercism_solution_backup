section .text

global square_of_sum
square_of_sum:
    mov rax, rdi
    inc rax
    imul rax, rdi
    shr rax, 1
    imul rax, rax
    ret
    
global sum_of_squares
sum_of_squares:
    ; rax = n(n + 1)(2n + 1)
    mov rax, rdi
    inc rax
    imul rax, rdi
    mov rcx, rdi
    lea rcx, [rdi*2+1]
    imul rax, rcx
    xor rdx, rdx   
    mov rcx, 6
    div rcx   
    ret
    
global difference_of_squares
difference_of_squares:
    call sum_of_squares
    mov rbx, rax
    call square_of_sum
    sub rax, rbx
    ret

section .text
global to_rna
to_rna:
    .loop_seq:
        mov al, byte [rdi]
        test al, al
        jz .done
        cmp al, 'G'
        je .case_G
        cmp al, 'C'
        je .case_C
        cmp al, 'T'
        je .case_T
        cmp al, 'A'
        je .case_A
 
    .case_G:
        mov byte [rsi], 'C'
        jmp .next
    .case_C:
        mov byte [rsi], 'G'
        jmp .next
    .case_T:
        mov byte [rsi], 'A'
        jmp .next
    .case_A:
        mov byte [rsi], 'U'
        jmp .next

    .next:
        inc rdi
        inc rsi
        jmp .loop_seq
        
    .done:
        mov byte [rsi], 0     ; Null-terminate destination
        ret
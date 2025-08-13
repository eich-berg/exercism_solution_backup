section .text
global reverse
reverse:
    mov rsi, rdi         ; source pointer
    xor ecx, ecx         ; clear counter
    
    .push_loop:
        mov al, [rsi]    ; load first char
        test al, al      ; check for null terminator
        jz .pop_loop     ; done, start rev
        push rax
        inc rsi          ; point to next char
        inc ecx
        jmp .push_loop

    .pop_loop:
        test ecx, ecx
        jz .done
        pop rax
        mov [rdi], al
        inc rdi
        dec ecx
        jmp .pop_loop
    
    .done:
        ret
        
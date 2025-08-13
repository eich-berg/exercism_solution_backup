section .text
global reverse
reverse:
    mov rsi, rdi           ; source pointer
    mov rdi, rdi           ; rdi will become end pointer
    .find_end:
        cmp byte [rdi], 0  ; Check for null terminator
        je .found_end
        inc rdi
        jmp .find_end
    .found_end:
        dec rdi            ; Move back to last non-null character
    .swap_loop:
        cmp rsi, rdi       ; Compare pointers
        jge .done          ; If they meet/cross, we're done
        ; Swap [rsi] and [rdi]
        mov  al, [rsi]      ; Load left char
        mov  bl, [rdi]      ; Load right char
        mov  [rsi], bl      ; Swap them
        mov  [rdi], al
        inc  rsi
        dec  rdi
        jmp  .swap_loop
            
    .done:
        ret
        
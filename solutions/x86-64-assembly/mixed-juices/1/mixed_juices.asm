section .data

    time_arr dd 1, 3, 3, 4, 5, 4, 7, 10 ; dd for 32-bit values

section .text

global time_to_make_juice
time_to_make_juice:
    ; This function has one argument, the ID for a juice as a 32-bit number
    ; It returns the time to prepare this juice, as a 32-bit number
    lea r8, [rel time_arr]
    mov eax, dword [r8 + (rdi - 1)*4]
    ret
    
global time_to_prepare
time_to_prepare:
    ; This function has two arguments:
    ; - An array with the IDs for ordered juices, each ID a 32-bit number
    ; - The number of ordered juices, also a 32-bit number.
    ; It returns the total time to prepare all ordered juices, as a 32-bit number
    mov rdx, rdi           ; array pointer
    mov ecx, esi
    xor r9, r9
    .prep_time_loop:                  
        mov edi, dword [rdx + (rcx-1)*4] ; base address + (index * element size)
        call time_to_make_juice
        add r9d, eax
        loop .prep_time_loop
    mov eax, r9d
    ret

global limes_to_cut
limes_to_cut:
    ; This function takes three arguments:
    ; - The number of wedges needed, as a 32-bit number.
    ; - An array with the current supply of limes, each represented by a 8-bit number.
    ; - The number of limes in the supply, as a 32-bit number.
    ; It returns the number of limes that need to be cut, as a 32-bit number
    mov rcx, rdx
    mov r9, 0        ; for array indexing
    .lime_loop:                  
        movzx r8, byte [rsi + r9*1] ; use counter to access byte elements in rsi
        cmp r8d, 'S'
        je .small_lime
        cmp r8d, 'M'
        je .medium_lime
        mov r11, 10 ; otherwise L'
        jmp .cont
        .small_lime:
        mov r11, 6
        jmp .cont
        .medium_lime:
        mov r11, 8
    .cont:
        inc r9
        sub rdi, r11
        cmp edi, 0        
        jle .done 
        loop .lime_loop
    .done:
        mov eax, r9d
        ret

global remaining_orders
remaining_orders:
    ; This function takes two arguments:
    ; - The time left in the shift, as a 32-bit number.
    ; - An array  with the IDs for ordered juices still not prepared, each ID a 32-bit number.
    ; It returns the number of juices made before the shift ends, as a 32-bit number.
    ; You may consider that:
    ; - The array is never empty.
    ; - The time left in the shift at the beginning is always greater than 0.
    ; - There are more orders in the array than that which can be prepared before the shift ends.
    mov rcx, rdi
    xor r9d, r9d
    .order_loop:                  
        mov edi, dword [rsi + r9*4] ; base address + (index * element size)
        call time_to_make_juice
        inc r9d
        sub ecx, eax
        cmp ecx, 0
        jg .order_loop    ; If still positive time left, continue
        mov eax, r9d
        ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
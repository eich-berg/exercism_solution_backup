section .data
    prev_week db 0, 2, 5, 3, 7, 8, 4, 0

section .bss
    curr_week resb 8
    curr_len resq 1
    
section .text

; the global directive makes a function visible to the test files
global last_week_counts
last_week_counts:
    ; This function takes no parameter
    ; It returns a copy of last week's counts as a 8-byte number
    ; At the start of the program, last week's counts are 0, 2, 5, 3, 7, 8 and 4
    ; The last byte of the return value is always zero
    mov rax, qword [rel prev_week]
    ret

global current_week_counts
current_week_counts:
    ; This function takes no parameter
    ; It returns two values:
    ; - A copy of current week's counts as a 8-byte number.
    ; - The number of days already filled in the current week, as a 8-byte number.
    ; All days after the most recent one should have its corresponding byte zeroed-out in the output
    ; At the start of the program, there is no count for the current week
    mov rax, qword [rel curr_week]
    mov rdx, qword [rel curr_len]
    ret

global save_count
save_count:
    ; This function takes as parameter the most recent count, as a 1-byte number
    ; It must save this value in a new entry for the current week
    ; If there is already 7 entries in the current week before the function is called, then:
    ; - The current week becomes the last week.
    ; - A new entry is added with the passed value in a new current week.
    ; The function has no return value
    mov rcx, [rel curr_len]
    cmp rcx, 7
    jl .inc                ; otherwise, overwrite prev_week, zero out curr_week + len
    mov rdx, qword [rel curr_week]
    mov qword [rel prev_week], rdx
    mov qword [rel curr_week], 0
    mov qword [rel curr_len], 0
    xor rcx, rcx
    .inc:
    lea r8, [rel curr_week]
    mov byte [r8 + rcx], dil
    inc qword [rel curr_len]
    ret

global today_count
today_count:
    ; This function has no parameter
    ; It returns the most recent entry for the current week, as a 1-byte number
    lea r8, [rel curr_week]
    mov rcx, [rel curr_len]
    movzx rax, byte [r8 + rcx - 1]
    ret

global update_today_count
update_today_count:
    ; This function takes as parameter a 1-byte number
    ; It adds this number to the most recent entry for the current week
    ; This function has no return value
    lea r8, [rel curr_week]
    mov rcx, [rel curr_len]
    add byte [r8 + rcx - 1], dil
    ret

global update_week_counts
update_week_counts:
    ; This function takes as parameter a 8-byte number
    ; Each byte in the input parameter, but the last, represents a day's count in the current week
    ; The last byte in the input parameter has no meaning and must be zeroed-out
    ; This function makes the following changes:
    ; - The current week becomes the last week.
    ; - The counts in the input parameter are fully inserted in the current week.
    mov rdx, qword [rel curr_week]
    mov qword [rel prev_week], rdx
    mov [rel curr_week], rdi
    ret


%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif

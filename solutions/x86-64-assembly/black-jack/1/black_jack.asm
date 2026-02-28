; Everything that comes after a semicolon (;) is a comment

C2 equ 2
C3 equ 3
C4 equ 4
C5 equ 5
C6 equ 6
C7 equ 7
C8 equ 8
C9 equ 9
C10 equ 10
CJ equ 11
CQ equ 12
CK equ 13
CA equ 14

TRUE equ 1
FALSE equ 0

section .text

; You should implement functions in the .text section

; the global directive makes a function visible to the test files
global value_of_card
value_of_card:
    ; This function takes as parameter a number representing a card
    ; The function should return the numerical value of the passed-in card
    mov rax, rdi
    cmp rdi, C10
    jle .fin
    xor rax, rax
    mov rax, 10
    cmp rdi, CK
    jle .fin
    xor rax, rax
    mov rax, 1
    .fin:
    ret

global higher_card
higher_card:
    ; This function takes as parameters two numbers each representing a card
    ; The function should return which card has the higher value
    ; If both have the same value, both should be returned
    ; If one is higher, the second one should be 0
    mov r8, rdi
    mov r9, rsi
    call value_of_card
    mov rbx, rax
    mov rdi, rsi
    call value_of_card
    cmp rbx, rax
    je .fin           ; jump if equal
    jg .fin_g         ; jump if first > second
    ; otherwise, second > first
    mov rax, r9
    xor rdx, rdx
    ret
    .fin_g:
    mov rax, r8
    xor rdx, rdx
    ret
    .fin:
    mov rax, r8
    mov rdx, r9
    ret
        
global value_of_ace
value_of_ace:
    ; This function takes as parameters two numbers each representing a card
    ; The function should return the value of an upcoming ace
    cmp rdi, CA
    je .fin
    call value_of_card
    mov rbx, rax
    mov rdi, rsi
    call value_of_card
    add rax, rbx
    add rax, 11
    cmp rax, 21
    jg .fin
    mov rax, 11
    ret
    .fin:
    mov rax, 1
    ret

global is_blackjack
is_blackjack:
    ; This function takes as parameters two numbers each representing a card
    ; The function should return TRUE if the two cards form a blackjack, and FALSE otherwise
    mov r8, rdi
    mov r9, rsi
    call value_of_card
    mov rbx, rax
    mov rdi, rsi
    call value_of_card
    
    cmp r8, CA
    je .test_b
    cmp r9, CA
    je .test_a
    mov rax, 0
    ret

    .test_b:
    cmp rax, C10
    jge .fin
    mov rax, 0
    ret

    .test_a:
    cmp rbx, C10
    jge .fin
    mov rax, 0
    ret

    .fin:
    mov rax, 1
    ret

global can_split_pairs
can_split_pairs:
    ; This function takes as parameters two numbers each representing a card
    ; The function should return TRUE if the two cards can be split into two pairs, and FALSE otherwise
    call value_of_card
    mov rbx, rax
    mov rdi, rsi
    call value_of_card
    cmp rbx, rax
    je .true
    mov rax, 0
    ret
    .true:
    mov rax, 1
    ret

global can_double_down
can_double_down:
    ; This function takes as parameters two numbers each representing a card
    ; The function should return TRUE if the two cards form a hand that can be doubled down, and FALSE otherwise
    call value_of_card
    mov rbx, rax
    mov rdi, rsi
    call value_of_card
    add rax, rbx
    cmp rax, 9
    jl .false
    cmp rax, 11
    jg .false
    mov rax, 1
    ret
    .false:
    mov rax, 0
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif

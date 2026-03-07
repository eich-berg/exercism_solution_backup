
PRIVATE_KEY equ 0b1011_0011_0011_1100

section .text

global extract_higher_bits
extract_higher_bits:
    ; This function has a 16-bit integer as argument.
    ; it returns the higher 8-bit value of the argument.
    shr di, 8       ; Shift right 8 bits - high byte moves to low byte
    movzx eax, di   ; Zero-extend to 32-bit for return
    ret
    
global extract_lower_bits
extract_lower_bits:
    ; This function takes one 16-bit integer as argument and must return the lower 8-bit value     of it.
    mov al, dil      ; Load the 16-bit value into rax lower bits
    ret

global extract_redundant_bits
extract_redundant_bits: 
    ; This function takes one 16-bit integer as argument.
    ; It returns a 8-bit integer with all bits set in both the lower and the higher 8 bits of     the argument.
    mov r9w, di
    call extract_higher_bits
    mov rdx, rax
    mov di, r9w
    call extract_lower_bits
    and rax, rdx
    ret
    
global set_message_bits
set_message_bits: 
    ; This function takes one 16-bit integer as argument.
    ; It returns a 8-bit integer with all bits set if they are set in the higher 8 bits of the     argument, the others unchanged.
    mov r9w, di
    call extract_higher_bits
    mov rdx, rax
    mov di, r9w
    call extract_lower_bits
    or rax, rdx
    ret

global rotate_private_key
rotate_private_key: 
    ; This function takes one 16-bit integer as argument.
    ; It returns a 16-bit integer with bits of the private key rotated to the left a number of positions equal to the redundant bits.
    ; The private key is 0b1011_0011_0011_1100.
    ; A bit is redundant when it is set in both the lowest 8-bit portion of the argument and the highest 8-bit portion of the argument.
    call extract_redundant_bits ; call func using di
    popcnt rcx, rax             ; count set bits in call result (rax) and save count in rcx
    mov dx, PRIVATE_KEY         ; mov equ into reg
    rol dx, cl                  ; left rot
    movzx eax, dx               ; zero extend
    ret
    
global format_private_key
format_private_key: 
    ; This function takes one 16-bit integer as argument.
    ; It returns a 8-bit integer with the private key fully formatted.
    ; To format a private key, you must:
    ; - Rotate it.
    ; - Isolate the lowest 8-bit portion of the rotated private key, which is the base value.
    ; - Isolate the highest 8-bit portion of the rotated private key, which is a mask to be applied to the base value.
    ; - Flip set bits in the base value that are also set in the mask.
    ; - Flip all bits in the result.
    call rotate_private_key      ; rotate
    mov edi, eax                 ; rot result is now new input
    call extract_lower_bits      ; isolate the lowest 8-bit portion of the rotated private key
    mov edx, eax                 ; save isolation result
    call extract_higher_bits     ; isolate the highest 8-bit portion of the rotated private key
    xor eax, edx                 ; find bits in the base value that are also set in the mask
    not eax                      ; flip all bits in the result
    ret
    
global decrypt_message
decrypt_message: 
    ; This function takes one 16-bit integer as argument
    ; It returns a 16-bit integer, of which:
    ; - The higher 8 bits are the formatted private key, according to 'format_private_key'
    ; - The lower 8 bits are the message with all bits set, according to 'set_message_bits'
    mov r9w, di
    call format_private_key
    mov bl, al          ; formatted key
    mov di, r9w
    call set_message_bits
    mov ah, bl
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
; Everything that comes after a semicolon (;) is a comment.

section .text

; You should implement functions in the .text section.
; A skeleton is provided for the first function.

; The global directive makes a function visible to the test files.
global daily_rate
daily_rate:
    ; This function takes an hourly_rate, as a 64-bit floating-point number.
    ; It returns the daily rate, also as a 64-bit floating-point number.
    ; A day has 8 billable hours.
    mov rbx, 8
    cvtsi2sd xmm8, rbx
    mulsd xmm0, xmm8
    ret
    
global apply_discount
apply_discount:
    ; It takes as parameters a price and a discount in percent, both as 64-bit floating-point number.
    ; It returns the price with discount applied, as a 64-bit floating-point number.
    mov rbx, 100
    cvtsi2sd xmm8, rbx
    divsd xmm1, xmm8
    mulsd xmm1, xmm0
    subsd xmm0, xmm1
    ret

global monthly_rate
monthly_rate:
    ; It takes as parameters an hourly_rate and a discount in percent, both as a 64-bit floating-point number.
    ; It returns the discounted monthly rate, as a 64-bit integer, rounded up.
    ; A month has 22 billable days.
    mov rbx, 8
    imul rbx, 22
    cvtsi2sd xmm8, rbx
    mulsd xmm0, xmm8
    call apply_discount
    roundsd xmm0, xmm0, 2
    cvtsd2si rax, xmm0
    ret

global days_in_budget
days_in_budget:
    ; It takes as parameters:
    ; 1. A budget as a 64-bit unsigned integer.
    ; 2. An hourly_rate, as a 64-bit floating-point number.
    ; 3. A discount in percent, as a 64-bit floating-point number.
    ; It returns the number of complete days of work the budget covers, as a 32-bit unsigned integer, rounded down. 
    call daily_rate
    call apply_discount
    cvtsi2sd xmm8, rdi
    divsd xmm8, xmm0
    roundsd xmm8, xmm8, 1
    cvtsd2si eax, xmm8
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif                  
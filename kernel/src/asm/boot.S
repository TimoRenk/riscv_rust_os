.section .text.init

.global _start
_start:
	la sp, _stack_end
    call kernel_setup
loop:
    j loop
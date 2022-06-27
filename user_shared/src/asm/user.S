	.section .text.init
	.global _start
_start:
	la sp, _stack_end
	call main
goaway:
	li	a7, 42
	li	a0, 0
	ecall
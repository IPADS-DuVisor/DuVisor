#include "asm.h"

BEGIN_FUNC_FILE_NAME()
	li t0, 0
	li t1, 100
	li t2, 0x3000
	li t4, 0
	
continue:
	ld t3, (t2)
	add t4, t4, t3
	addi t2, t2, 0xf8
	ld t3, (t2)
	add t4, t4, t3
	addi t0, t0, 1
	li t5, 0x1008
	add t2, t2, t5
	bne	t0, t1, continue

	li a7, ECALL_VM_TEST_END
	ecall
END_FUNC_FILE_NAME()
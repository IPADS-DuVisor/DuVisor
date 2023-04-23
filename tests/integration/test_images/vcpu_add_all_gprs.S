#include "asm.h"

BEGIN_FUNC_FILE_NAME()
	// x0 is hardwired to 0
	li x1, 10
	add x1, x1, x2
	add x1, x1, x3
	add x1, x1, x4
	add x1, x1, x5
	add x1, x1, x6
	add x1, x1, x7
	add x1, x1, x8
	add x1, x1, x9
	add x1, x1, x10
	add x1, x1, x11
	add x1, x1, x12
	add x1, x1, x13
	add x1, x1, x14
	add x1, x1, x15
	add x1, x1, x16
	add x1, x1, x17
	add x1, x1, x18
	add x1, x1, x19
	add x1, x1, x20
	add x1, x1, x21
	add x1, x1, x22
	add x1, x1, x23
	add x1, x1, x24
	add x1, x1, x25
	add x1, x1, x26
	add x1, x1, x27
	add x1, x1, x28
	add x1, x1, x29
	add x1, x1, x30
	add x1, x1, x31
	mv a0, x1
	li a7, ECALL_VM_TEST_END
	ecall
END_FUNC_FILE_NAME()

#include "asm.h"

BEGIN_FUNC_FILE_NAME()
	li t1, 0x3000
	ld t1, (t1)
	li t2, 0x3000
	li t3, 0x4321
	sd t3, (t2)
	li a7, ECALL_VM_TEST_END
	ecall
END_FUNC_FILE_NAME()
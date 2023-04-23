#include "asm.h"

BEGIN_FUNC_FILE_NAME()
	li t0, 0x3000
	ld t1, (t0)
	add t2, t1, 1
	sd t2, (t0)
	jalr t0
	li a7, ECALL_VM_TEST_END
	ecall
END_FUNC_FILE_NAME()

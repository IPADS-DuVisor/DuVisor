#include "asm.h"

BEGIN_FUNC_FILE_NAME()
	li t0, 0
	li t1, 0x3000

	ld t0, (t1)

	// test end
	li a7, ECALL_VM_TEST_END
	ecall
END_FUNC_FILE_NAME()
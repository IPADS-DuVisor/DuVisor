#include "asm.h"

BEGIN_FUNC_FILE_NAME()
	li t1, 0x3000
	ld t1, (t1)
	li a7, ECALL_VM_TEST_END
	ecall
END_FUNC_FILE_NAME()
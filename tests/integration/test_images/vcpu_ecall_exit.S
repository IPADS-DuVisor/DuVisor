#include "asm.h"

BEGIN_FUNC_FILE_NAME()
	li a7, ECALL_VM_TEST_END
	ecall
END_FUNC_FILE_NAME()

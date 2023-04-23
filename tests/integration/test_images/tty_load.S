#include "asm.h"

BEGIN_FUNC_FILE_NAME()
	li t0, 0x3000
	li t1, 0x3f8

	lb a0, 0x0(t1)
	sb a0, 0x0(t0)

	lb a0, 0x1(t1)
	sb a0, 0x1(t0)

	lb a0, 0x2(t1)
	sb a0, 0x2(t0)

	lb a0, 0x3(t1)
	sb a0, 0x3(t0)

	lb a0, 0x4(t1)
	sb a0, 0x4(t0)

	lb a0, 0x5(t1)
	sb a0, 0x5(t0)

	lb a0, 0x6(t1)
	sb a0, 0x6(t0)

	lb a0, 0x7(t1)
	sb a0, 0x7(t0)

	// test end
	li a7, ECALL_VM_TEST_END
	ecall
END_FUNC_FILE_NAME()
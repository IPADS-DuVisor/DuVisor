#include "asm.h"

BEGIN_FUNC_FILE_NAME()
    /* Set up irq_handler */
    la   t1, irq_handler
    csrw stvec, t1
    csrs sstatus, 0x2

    /* Enable VS_SOFT via SSIE bit */
    li   t1, 0x2
    csrs sie, t1

    /* Notice local vcpu thread to insert virtual ipi */
	li a7, SBI_TEST_HU_VIRTUAL_IPI
	ecall

loop:
    wfi
    j loop


END_FUNC_FILE_NAME()

.align 4
irq_handler:
    li a7, ECALL_VM_TEST_END
    ecall


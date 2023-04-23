#include "asm.h"

BEGIN_FUNC_FILE_NAME()
    /* Set up irq_handler */
    la   t1, irq_handler
    csrw stvec, t1
    csrs sstatus, 0x2

    /* Enable VS_SOFT via SSIE bit */
    li   t1, 0x2
    csrs sie, t1

    /* Address for sync */
    li t0, 0x3000

    li t1, 0
    li t2, 1

    beq	a0, t1, vcpu_0
    beq	a0, t2, vcpu_1
    j test_failed

/* vcpu 0 */
vcpu_0:
    /* Wait for vcpu 1 util sync = 1 */
    ld t5, 0x0(t0)
    li t6, 0x1
    bne t5, t6, vcpu_0

    /* Set up hart mask */
    li a0, 0x3100
    li a2, 2
    sd a2, 0x0(a0)

    /* Send virtual ipi */
    li a7, SBI_EXT_0_1_SEND_IPI
    ecall

    /* Update sync to 2 */
    li t6, 2
    sd t6, 0x0(t0)

    j test_success


/* vcpu 1 */
vcpu_1:
    /* 
     * Ecall into HU-mode
     * a1 = hva of 0x3000(gpa)
     * a2 = start signal = 1
     * a3 = end signal = 2
     */
    li a7, SBI_TEST_HU_LOOP
    li a2, 1
    li a3, 2
    ecall

    /* Vcpu 1 should exit from irq_handler */
    j test_failed

test_success:
    li a7, ECALL_VM_TEST_END
    ecall

test_failed:
    li a7, SBI_TEST_FAILED
    ecall

END_FUNC_FILE_NAME()

.align 4
irq_handler:
    li a7, SBI_TEST_SUCCESS
    ecall
    li a7, ECALL_VM_TEST_END
    ecall

#include "asm.h"

BEGIN_FUNC_FILE_NAME()
    /* Set up irq_handler */
    la   a6, irq_handler
    csrw stvec, a6

    /* Set up sstatus.SIE */
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

    /* 
     * Set up hart mask to aim at vcpu [1,2,3,4,5,6,7]
     * And vcpu [2,3,4,5,6,7] is invalid.
     */
    li a0, 0x3100
    li a2, 0xfe
    sd a2, 0x0(a0)

    /* Send virtual ipi */
    li a7, SBI_EXT_0_1_SEND_IPI
    ecall

    /* Vcpu 0 should finish the SEND_IPI ecall */
    j test_success

/* vcpu 1 */
vcpu_1:
    /* Set sync data = 1 */
    sd t2, 0x0(t0)

wait_for_ipi_1:
    wfi
    /* Wait for vipi of vcpu 0 */
    j wait_for_ipi_1

    /* Vcpu 1 should never reach here */
    j test_failed

test_success:
    li a7, SBI_TEST_SUCCESS
    ecall
    li a7, ECALL_VM_TEST_END
    ecall

test_failed:
    li a7, SBI_TEST_FAILED
    ecall
    li a7, ECALL_VM_TEST_END
    ecall
END_FUNC_FILE_NAME()

/* 
 * Irq handler for vcpu 0, it will end this
 * test case.
 */
.align 4
irq_handler:
    li a7, ECALL_VM_TEST_END
    ecall

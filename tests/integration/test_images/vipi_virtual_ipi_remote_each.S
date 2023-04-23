#include "asm.h"

BEGIN_FUNC_FILE_NAME()
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
    /* Set up irq_handler */
    la   a6, irq_handler_0
    csrw stvec, a6

    /* Wait for vcpu 1 util sync = 1 */
    ld t5, 0x0(t0)
    li t6, 0x1
    bne t5, t6, vcpu_0

    /* Set up hart mask to aim at vcpu 1 */
    li a0, 0x3100
    li a2, 2
    sd a2, 0x0(a0)

    /* Send virtual ipi */
    li a7, SBI_EXT_0_1_SEND_IPI
    ecall

wait_for_ipi_0:
    wfi
    /* Wait for vipi of vcpu 1 */
    j wait_for_ipi_0

    /* Vcpu 0 should never reach here */
    j test_failed

/* vcpu 1 */
vcpu_1:
    /* Set up irq_handler */
    la   a6, irq_handler_1
    csrw stvec, a6

    /* Set sync data = 1 */
    sd t2, 0x0(t0)

wait_for_ipi_1:
    wfi
    /* Wait for vipi of vcpu 0 */
    j wait_for_ipi_1

    /* Vcpu 1 should never reach here */
    j test_failed

test_success:
    li a7, ECALL_VM_TEST_END
    ecall

test_failed:
    li a7, SBI_TEST_FAILED
    ecall

END_FUNC_FILE_NAME()

/* 
 * Irq handler for vcpu 0, it will end this
 * test case.
 */
.align 4
irq_handler_0:
    li a7, SBI_TEST_SUCCESS
    ecall
    li a7, ECALL_VM_TEST_END
    ecall

/* 
 * Irq handler for vcpu 1, it will send vipi
 * to vcpu 0.
 */
.align 4
irq_handler_1:
    /* Set up hart mask to aim at vcpu 0 */
    li a0, 0x3200
    li a2, 1
    sd a2, 0x0(a0)

    /* Send virtual ipi */
    li a7, SBI_EXT_0_1_SEND_IPI
    ecall

    /* Vcpu 1 exit */
    li a7, ECALL_VM_TEST_END
    ecall

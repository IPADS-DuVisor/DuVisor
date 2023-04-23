#include "asm.h"

BEGIN_FUNC_FILE_NAME()
    /* Address for sync */
    li t0, 0x3000

    li t1, 0
    li t2, 1

    beq	a0, t1, vcpu_0
    beq	a0, t2, vcpu_1
    j test_failed

vcpu_0:
    ld t5, 0x0(t0)
    li t6, 0x2
    /* 
     * If *0x3000 == 2, the sending thread must have finished
     * the setting of VIPI CSR via the vipi struct of vcpu 0.
     * Then vcpu 0 can exit.
     */
    bne t5, t6, vcpu_0
    j test_success

vcpu_1:
    /* Set sync data */
    sd t2, 0x0(t0)

wait_for_uipi:
    ld t3, 0x0(t0)
    li t4, 0x2
    wfi
    /* 
     * If *0x3000 == 2, the sending thread must have finished
     * the setting of VIPI CSR. And vcpu will get user ipi before
     * it reach test_success.
     */
    bne t4, t3, wait_for_uipi
    
test_success:
    li a7, ECALL_VM_TEST_END
    ecall

test_failed:
    li a7, SBI_TEST_FAILED
    ecall

END_FUNC_FILE_NAME()

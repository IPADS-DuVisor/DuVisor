#include "asm.h"

BEGIN_FUNC_FILE_NAME()
    /* all the results will be stored on 0x3000(GPA) */
    li t0, 0x3000

    /* save return values of CLEAR_IPI */
    li a7, SBI_EXT_0_1_CLEAR_IPI
    ecall
    sd a0, (t0)
    add t0, t0, 8
    sd a1, (t0)

    li a7, ECALL_VM_TEST_END
    ecall
END_FUNC_FILE_NAME()

#include "csr.h"
#include "../vcpu/asm_offset.h"
#include "save_restore.S"

.align 2

.global enter_guest

enter_guest:
    /* a0 point to vcpu_ctx */

    /* save host gp with a0=ctx, except t0-t6 and zero-x0 */
    SAVE_HOST_CTX a0

    /* save a0-vcpu-ctx in CSR_USCRATCH & save USCRATCH */ 
    csrrw t3, CSR_USCRATCH, a0
    sd t3, HOST_HYP_USCRATCH(a0)

    ld t0, HOST_HYP_HUSTATUS(a0)
    csrw CSR_HUSTATUS, t0

    ld t0, HOST_HYP_HUCOUNTEREN(a0)
    csrw CSR_HUCOUNTEREN, t0

    ld t0, HOST_HYP_HUTVAL(a0)
    csrw CSR_HUTVAL, t0

    ld t0, HOST_HYP_HUCOUNTEREN(a0)
    csrw CSR_HUCOUNTEREN, t0

    /* Restore UEPC & UCAUSE & UTVAL for trap handler */
    ld t0, HOST_HYP_UEPC(a0)
    csrw CSR_UEPC, t0

    ld t0, HOST_HYP_UCAUSE(a0)
    csrw CSR_UCAUSE, t0

    ld t0, HOST_HYP_UTVAL(a0)
    csrw CSR_UTVAL, t0

    /* restore guest GP except A0 & X0 */
    RESTORE_GUEST_CTX a0
    
    /* restore guest A0 */
    ld x10, GUEST_GP_X10(a0)

    /* huret */
    uret

    .align 2
    .global exit_guest
exit_guest:
    /* save guest-a0 in sscratch & get host-a0 */
    csrrw a0, CSR_USCRATCH, a0

    /* save guest gp except A0 & X0 */
    SAVE_GUEST_CTX a0

    /* save guest A0 with USCRATCH */
    csrr t1, CSR_USCRATCH
    sd t1, GUEST_GP_X10(a0)

    csrr t0, CSR_HUSTATUS
    sd t0, HOST_HYP_HUSTATUS(a0)

    csrr t0, CSR_HUCOUNTEREN
    sd t0, HOST_HYP_HUCOUNTEREN(a0)

    csrr t0, CSR_HUTVAL
    sd t0, HOST_HYP_HUTVAL(a0)

    csrr t0, CSR_HUCOUNTEREN
    sd t0, HOST_HYP_HUCOUNTEREN(a0)

    /* Save UEPC & UCAUSE & UTVAL for trap handler */
    csrr t0, CSR_UEPC
    sd t0, HOST_HYP_UEPC(a0)

    csrr t0, CSR_UCAUSE
    sd t0, HOST_HYP_UCAUSE(a0)

    csrr t0, CSR_UTVAL
    sd t0, HOST_HYP_UTVAL(a0)

    /* restore host gp with a0=ctx, except t0-t6 and zero-x0 */
    RESTORE_HOST_CTX a0

    ret
    
/*
    * Instruction encoding of hufence.gvma is:
    * HUFENCE.GVMA rs1, rs2
    * HUFENCE.GVMA zero, rs2
    * HUFENCE.GVMA rs1
    * HUFENCE.GVMA
    *
    * rs1!=zero and rs2!=zero ==> HUFENCE.GVMA rs1, rs2
    * rs1==zero and rs2!=zero ==> HUFENCE.GVMA zero, rs2
    * rs1!=zero and rs2==zero ==> HUFENCE.GVMA rs1
    * rs1==zero and rs2==zero ==> HUFENCE.GVMA
    *
    * Instruction encoding of HUFENCE.GVMA is:
    * 1110001 rs2(5) rs1(5) 000 00000 1110011
    */
.global hufence_gvma_vmid_gpa
hufence_gvma_vmid_gpa:
	/*
	 * rs1 = a0 (GPA)
	 * rs2 = a1 (VMID)
	 * HUFENCE.GVMA a0, a1
	 * 1110001 01011 01010 000 00000 1110011
	 */
	.word 0xe2b50073
	ret

.global hufence_gvma_vmid
hufence_gvma_vmid:
	/*
	 * rs1 = zero
	 * rs2 = a0 (VMID)
	 * HUFENCE.GVMA zero, a0
	 * 1110001 01010 00000 000 00000 1110011
	 */
	.word 0xe2a00073
	ret

.global hufence_gvma_gpa
hufence_gvma_gpa:
	/*
	 * rs1 = a0 (GPA)
	 * rs2 = zero
	 * HUFENCE.GVMA a0
	 * 1110001 00000 01010 000 00000 1110011
	 */
	.word 0xe2050073
	ret

.global hufence_gvma_all
hufence_gvma_all:
	/*
	 * rs1 = zero
	 * rs2 = zero
	 * HUFENCE.GVMA
	 * 1110001 00000 00000 000 00000 1110011
	 */
	.word 0xe2000073
	ret
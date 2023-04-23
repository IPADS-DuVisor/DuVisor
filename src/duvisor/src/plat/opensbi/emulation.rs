use crate::csrw;
use crate::dbgprintln;
#[cfg(feature = "cve")]
use crate::init::cmdline::cve_mode::CVE_PARA_VIRTUALIZATION;
#[cfg(feature = "cve")]
use crate::init::cmdline::CVE_MODE;
use crate::init::cmdline::MAX_VCPU;
use crate::irq::delegation::delegation_constants::*;
use crate::irq::vipi::VirtualIpi;
#[allow(unused)]
use crate::plat::uhe::csr::csr_constants::*;
use crate::plat::uhe::ioctl::ioctl_constants::*;
use crate::print_flush;
#[allow(unused)]
use crate::vcpu::utils::*;
use crate::vcpu::virtualcpu::VirtualCpu;
use core::arch::asm;
use error_code::*;
use sbi_number::*;
use sbi_test::*;
use std::io::{self, Write};
use std::sync::atomic::Ordering;
use std::{thread, time};

#[cfg(test)]
use crate::irq::vipi::tests::TEST_SUCCESS_CNT;

#[cfg(test)]
use crate::irq::vipi::tests::INVALID_TARGET_VCPU;

/* Flag for SBI SHUTDOWN */
pub static mut SHUTDOWN_FLAG: i32 = 0;

pub mod sbi_number {
    pub const SBI_EXT_0_1_SET_TIMER: u64 = 0x0;
    pub const SBI_EXT_0_1_CONSOLE_PUTCHAR: u64 = 0x1;
    pub const SBI_EXT_0_1_CONSOLE_GETCHAR: u64 = 0x2;
    pub const SBI_EXT_0_1_CLEAR_IPI: u64 = 0x3;
    pub const SBI_EXT_0_1_SEND_IPI: u64 = 0x4;
    pub const SBI_EXT_0_1_REMOTE_FENCE_I: u64 = 0x5;
    pub const SBI_EXT_0_1_REMOTE_SFENCE_VMA: u64 = 0x6;
    pub const SBI_EXT_0_1_REMOTE_SFENCE_VMA_ASID: u64 = 0x7;
    pub const SBI_EXT_0_1_SHUTDOWN: u64 = 0x8;
}

/*
 * SBI introduced for evaluation, test cases of this project.
 * Extension name: ULH Extension
 * The SBI extension space is 0xC000000-0xCFFFFFF
 */
pub mod sbi_test {
    pub const SBI_TEST_SPACE_START: u64 = 0xC000000;
    pub const SBI_TEST_SPACE_END: u64 = 0xCFFFFFF;

    pub const SBI_TEST_HU_VIRTUAL_IPI: u64 = 0xC000001;

    /* Test result */
    pub const SBI_TEST_SUCCESS: u64 = 0xC000007;
    pub const SBI_TEST_FAILED: u64 = 0xC000008;

    /* Loop in HU-mode */
    pub const SBI_TEST_HU_LOOP: u64 = 0xC100000;
}

#[allow(unused)]
pub mod error_code {
    pub const SBI_SUCCESS: i64 = 0;
    pub const SBI_ERR_FAILURE: i64 = -1;
    pub const SBI_ERR_NOT_SUPPORTED: i64 = -2;
    pub const SBI_ERR_INVALID_PARAM: i64 = -3;
    pub const SBI_ERR_DENIED: i64 = -4;
    pub const SBI_ERR_INVALID_ADDRESS: i64 = -5;
}

#[allow(unused)]
extern "C" {
    fn getchar_emulation() -> i32;
    fn wrvtimectl(val: u64);
    fn wrvtimecmp(val: u64);
}

pub struct Ecall {
    /* EID - a7 */
    pub ext_id: u64,

    /* FID - a6 */
    pub func_id: u64,

    /* Args - a0~a5 */
    pub arg: [u64; 6],

    /* Return - a0, a1 */
    pub ret: [u64; 2],
}

impl Ecall {
    pub fn new() -> Self {
        let ext_id: u64 = 0;
        let func_id: u64 = 0;
        let arg: [u64; 6] = [0; 6];
        let ret: [u64; 2] = [0; 2];

        Self {
            ext_id,
            func_id,
            arg,
            ret,
        }
    }

    /*
     * Emulation for the ecall from VS-mode, however part of the ecall cannot
     * be finished in U-mode for now. So pass the ioctl_fd to call kernel
     * module.
     */
    pub fn ecall_handler(&mut self, ioctl_fd: i32, vcpu: &VirtualCpu) -> i32 {
        let ext_id = self.ext_id;
        let ret: i32;

        match ext_id {
            SBI_EXT_0_1_SET_TIMER => {
                /*
                 * TODO: add rust feature to tell between rv64 and rv32
                 * TODO: next_cycle = ((u64)cp->a1 << 32) | (u64)cp->a0; if
                 * rv32
                 */
                let next_cycle = self.arg[0];

                /*
                 * Linux thinks that the IRQ_S_TIMER will be cleared when ecall
                 * SBI_EXT_0_1_SET_TIMER
                 * For record, opensbi thinks that IRQ_M_TIMER should be
                 * cleared by software.
                 * Qemu and xv6 think that IRQ_M_TIMER should be clear when
                 * writing timecmp.
                 * I think that IRQ_U_TIMER should be cleared by software.
                 * That's a drawback of riscv, unlike GIC which can provide the
                 * same interface for eoi.
                 */
                vcpu.unset_pending_irq(IRQ_VS_TIMER);
                unsafe {
                    #[cfg(feature = "xilinx")]
                    {
                        wrvtimectl(1);
                        wrvtimecmp(next_cycle);
                    }

                    #[cfg(feature = "qemu")]
                    {
                        csrw!(VTIMECTL, (IRQ_U_TIMER << 1) | (1 << VTIMECTL_ENABLE));
                        csrw!(VTIMECMP, next_cycle);
                    }
                }
                dbgprintln!("set vtimer for ulh");
                ret = 0;

                #[cfg(feature = "cve")]
                unsafe {
                    if CVE_MODE == CVE_PARA_VIRTUALIZATION {
                        static mut CNT: u64 = 0;
                        CNT += 1;

                        /* Boot a 1-core VM has about 300 vtimer-settings */

                        if CNT > 300 {
                            /* Wait about 2 seconds */
                            panic!("Emulating CVE-2016-5412 (infinite loop) in para-virtualization!");
                        }
                    }
                }
            }
            SBI_EXT_0_1_CONSOLE_PUTCHAR => {
                ret = self.console_putchar();
            }
            SBI_EXT_0_1_CONSOLE_GETCHAR => {
                ret = self.console_getchar();
            }
            SBI_EXT_0_1_CLEAR_IPI => {
                dbgprintln!("EXT ID {} has not been implemented yet.", ext_id);
                ret = self.unsupported_sbi();
            }
            SBI_EXT_0_1_SEND_IPI => {
                dbgprintln!("ready to hart mask");
                let hart_mask = self.get_hart_mask(self.arg[0]);
                dbgprintln!("finish hart mask");

                let mut vipi_id: u64;
                for i in 0..MAX_VCPU {
                    if ((1 << i) & hart_mask) != 0 {
                        /* Check whether the target vcpu is valid */
                        if i >= vcpu.vcpu_num() {
                            /* Invalid target */
                            #[cfg(test)]
                            unsafe {
                                *INVALID_TARGET_VCPU.lock().unwrap() += 1;
                            }

                            continue;
                        }
                        vipi_id = vcpu.get_vcpu_id_map(i).load(Ordering::SeqCst);
                        if vcpu.get_irqchip().unwrap().trigger_virtual_irq(i) {
                            VirtualIpi::set_vipi(vipi_id);
                        }
                    }
                }
                dbgprintln!("hart mask 0x{:x}", hart_mask);
                dbgprintln!("{} send ipi ...", vcpu.vcpu_id());

                ret = 0;
            }
            SBI_EXT_0_1_SHUTDOWN => {
                println!("Poweroff the virtual machine by vcpu {}", vcpu.vcpu_id());
                ret = -100;
                unsafe {
                    SHUTDOWN_FLAG = 1;
                }
            }
            SBI_EXT_0_1_REMOTE_FENCE_I
            | SBI_EXT_0_1_REMOTE_SFENCE_VMA
            | SBI_EXT_0_1_REMOTE_SFENCE_VMA_ASID => {
                /*
                 * All of these three SBIs will be directly emulated as
                 * SBI_EXT_0_1_REMOTE_FENCE_I for now.
                 */
                unsafe {
                    let ecall_ret: [u64; 2] = [0, 0];
                    let ret_ptr = (&ecall_ret) as *const u64;

                    /* Call ioctl IOCTL_REMOTE_FENCE to kernel module */
                    let _res = libc::ioctl(ioctl_fd, IOCTL_REMOTE_FENCE, ret_ptr);

                    self.ret[0] = ecall_ret[0];
                    self.ret[1] = ecall_ret[1];
                }
                ret = 0;
            }
            SBI_TEST_SPACE_START..=SBI_TEST_SPACE_END => {
                /* ULH Extension */
                ret = self.ulh_extension_emulation(vcpu);
            }
            _ => {
                dbgprintln!("EXT ID {} has not been implemented yet.", ext_id);
                ret = self.unsupported_sbi();
            }
        }

        ret
    }

    fn ulh_extension_emulation(&mut self, vcpu: &VirtualCpu) -> i32 {
        let ext_id = self.ext_id;

        match ext_id {
            SBI_TEST_HU_VIRTUAL_IPI => {
                /* Set vipi for the vcpu itself */
                vcpu.get_irqchip()
                    .unwrap()
                    .trigger_virtual_irq(vcpu.vcpu_id());
            }
            SBI_TEST_HU_LOOP => {
                /* Keep the vcpu thread in HU-mode */

                /* Get hva of the sync data and the end signal */
                let target_hva: u64 = self.arg[1];
                let start_signal = self.arg[2];
                let end_signal = self.arg[3];
                println!("target a1: 0x{:x}", target_hva);
                println!("start signal a2: {}", start_signal);
                println!("end signal a3: {}", end_signal);

                unsafe {
                    /* Set up the start signal */
                    *(target_hva as *mut u64) = start_signal;

                    /* Wait for the end signal */
                    while *(target_hva as *mut u64) != end_signal {
                        let ten_millis = time::Duration::from_millis(10);

                        thread::sleep(ten_millis);
                    }
                }

                println!("SBI_TEST_HU_LOOP end!");
            }
            SBI_TEST_SUCCESS => {
                #[cfg(test)]
                unsafe {
                    *TEST_SUCCESS_CNT.lock().unwrap() += 1;
                }
            }
            SBI_TEST_FAILED => {
                dbgprintln!("SBI_TEST_FAILED {}", vcpu.vcpu_id());
            }
            _ => {
                dbgprintln!("EXT ID {} has not been implemented yet.", ext_id);
                self.unsupported_sbi();
            }
        }

        0
    }

    /* Get hart_mask from guest memory by the address in a0 */
    fn get_hart_mask(&self, target_address: u64) -> u64 {
        let a0 = target_address;
        let hart_mask: u64;
        dbgprintln!("get_hart_mask a0 = 0x{:x}", a0);
        unsafe {
            asm!(
                ".option push",
                ".option norvc",

                /* HULVX.HU t0, (t2) */
                ".word 0x6c03c2f3",

                /* HULVX.HU t1, (t2) */
                out("t0") hart_mask,
                in("t2") a0,
            );
        }

        return hart_mask;
    }

    fn console_putchar(&mut self) -> i32 {
        let ch = self.arg[0] as u8;
        let ch = ch as char;
        print_flush!("{}", ch);

        /* Success and return with a0 = 0 */
        self.ret[0] = 0;

        0
    }

    fn unsupported_sbi(&mut self) -> i32 {
        /* SBI error and return with a0 = SBI_ERR_NOT_SUPPORTED */
        self.ret[0] = SBI_ERR_NOT_SUPPORTED as u64;

        0
    }

    fn console_getchar(&mut self) -> i32 {
        let ret: i32;

        /* Cannot switch the backend process to the front. */
        /* So test_ecall_getchar() have to get chars from here.  */
        #[cfg(test)]
        {
            let virtual_input: [i32; 16];

            /* Input "getchar succeed\n" */
            virtual_input = [
                103, 101, 116, 99, 104, 97, 114, 32, 115, 117, 99, 99, 101, 101, 100, 10,
            ];

            static mut INDEX: usize = 0;

            unsafe {
                ret = virtual_input[INDEX];
                INDEX += 1;
            }

            /* Success and return with a0 = 0 */
            self.ret[0] = ret as u64;

            return 0;
        }

        #[allow(unreachable_code)]
        {
            unsafe {
                ret = getchar_emulation();
            }

            /* Success and return with a0 = 0 */
            self.ret[0] = ret as u64;

            0
        }
    }
}

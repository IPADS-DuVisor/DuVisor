use libc;
use std::ffi::CString;
use core::arch::asm;

pub const VPLIC_LENGTH: usize = 0x1000;
pub const VMODE_VPLIC_OFFSET: u64 = 0x1f00000;
pub const VIRT_IRQ_OFFSET: u32 = 5;
pub const NULLPTR: *mut libc::c_void = 0 as *mut libc::c_void;

pub struct VPlic {
    pending_vector: u64,
}

impl VPlic {
    pub fn new() -> Self {
        let pending_vector = VPlic::acquire_vplic();
        let ptr = pending_vector as *mut u32;
        unsafe {
            *ptr = 0;
        }
        Self {
            pending_vector: pending_vector as u64,
        }
    }

    pub fn acquire_vplic() -> *mut u32 {
        let file_path = CString::new("/dev/vplic_dev").unwrap();
        let vplic_fd;
        unsafe {
            vplic_fd = (libc::open(file_path.as_ptr(), libc::O_RDWR)) as i32;
            let vplic_base_addr = libc::mmap(
                NULLPTR,
                VPLIC_LENGTH,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED,
                vplic_fd,
                0,
            );
            assert_ne!(vplic_base_addr, libc::MAP_FAILED);
            let vmode_vplic_addr =
                vplic_base_addr as *mut libc::c_void;
            let vplic_ptr = vmode_vplic_addr as *mut u32;
            return vplic_ptr;
        }
    }

    /* check irq is sent to virtual device. */
    pub fn check_virt_irq(irq: u32) -> bool {
        irq >= VIRT_IRQ_OFFSET
    }

    pub fn send_posted_interrupt(&self, irq: u32) {
        if VPlic::check_virt_irq(irq) == false {
            println!("send_posted_interrupt ERROR. irq: 0x{:x}", irq);
            return;
        }
        /* offset will be minused in QEMU as long as DuVisor access plic with VMODE_VPLIC_OFFSET */
        /* more detail explaination can be found in code of QEMU:hw/intc/sifive_plic.c:sifive_plic_write */
        /* I know such kind of design can be ugly and confusing, but it's not convenient to
         * change it since previous developer has already designed it */
        let real_irq = irq - VIRT_IRQ_OFFSET;
        let ptr = self.pending_vector as *mut u32;
        unsafe {
            asm!("fence w,o");
            *ptr = *ptr | (1 << real_irq);
            asm!("fence w,o");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusty_fork::rusty_fork_test;

    rusty_fork_test! {
        #[test]
        fn test_access_vplic() {
            unsafe {
                let vplic_ptr = VPlic::acquire_vplic();
                let val = *vplic_ptr;
                println!("[debug-DuVisor] vplic value:0x{:x}", val);
            }
        }
    }
}

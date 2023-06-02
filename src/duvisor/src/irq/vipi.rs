/*
Copyright (c) 2023 The institute of parallel and distributed systems (IPADS)
DuVisor is licensed under Mulan PSL v2.
You can use this software according to the terms and conditions of the Mulan PSL v2.
You may obtain a copy of Mulan PSL v2 at:
     http://license.coscl.org.cn/MulanPSL2

THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND,
EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT,
MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
See the Mulan PSL v2 for more details.
*/

/*
 * TODO: Virutal ipi should be implemented by adding new instructions, not
 * additional CSRs.
 */
/*
 * TODO: Expose too much info of vm to the hardware now. Vcpu should be
 * pinned and use software to find the target pcpu.
 */
use crate::csrc;
use crate::csrs;
use crate::csrw;
use crate::init::cmdline::MAX_VCPU;
#[allow(unused)]
use crate::vcpu::utils::*;
use core::arch::asm;
use std::sync::atomic::{AtomicU64, Ordering};

#[allow(unused)]
pub struct VirtualIpi {
    vcpu_id_map: Vec<AtomicU64>,
    vcpu_num: u32,
}

impl VirtualIpi {
    pub fn new(vcpu_num: u32) -> Self {
        let mut vcpu_id_map: Vec<AtomicU64> = Vec::with_capacity(vcpu_num as usize);

        for _ in 0..vcpu_num {
            vcpu_id_map.push(AtomicU64::new(0));
        }

        Self {
            vcpu_id_map,
            vcpu_num,
        }
    }

    pub fn vcpu_regist(&self, vcpu_id: u32, vipi_id: u64) {
        self.vcpu_id_map[vcpu_id as usize].store(vipi_id, Ordering::SeqCst);

        #[cfg(feature = "xilinx")]
        wrvcpuid(vipi_id);

        #[cfg(feature = "qemu")]
        unsafe {
            csrw!(VCPUID, vipi_id);
        }
    }

    /* TODO: Get cpu mask for the target vcpus */
    pub fn send_vipi(&self, hart_mask: u64) {
        let mut vipi_id: u64;
        for i in 0..MAX_VCPU {
            if ((1 << i) & hart_mask) != 0 {
                vipi_id = self.vcpu_id_map[i as usize].load(Ordering::SeqCst);
                VirtualIpi::set_vipi(vipi_id);
            }
        }
    }

    fn set_vipi_bit(csr_id: i32, vipi_id: u64) {
        match csr_id {
            0 => {
                set_vipi0(1 << vipi_id);
            }
            1 => {
                set_vipi1(1 << (vipi_id - 64));
            }
            2 => {
                set_vipi2(1 << (vipi_id - 128));
            }
            3 => {
                set_vipi3(1 << (vipi_id - 192));
            }
            _ => {
                panic!("Invalid vipi csr id ! {}", csr_id);
            }
        }
    }

    pub fn set_vipi(vipi_id: u64) {
        let csr_id: i32;

        match vipi_id {
            1..=63 => {
                /* Set VIPI0 */
                csr_id = 0;
            }
            64..=127 => {
                /* Set VIPI1 */
                csr_id = 1;
            }
            128..=191 => {
                /* Set VIPI2 */
                csr_id = 2;
            }
            192..=255 => {
                /* Set VIPI3 */
                csr_id = 3;
            }
            _ => {
                panic!("Invalid vipi id ! {}", vipi_id);
            }
        }

        VirtualIpi::set_vipi_bit(csr_id, vipi_id);
    }

    fn clear_vipi_bit(csr_id: i32, vipi_id: u64) {
        match csr_id {
            0 => {
                clear_vipi0(1 << vipi_id);
            }
            1 => {
                clear_vipi1(1 << (vipi_id - 64));
            }
            2 => {
                clear_vipi2(1 << (vipi_id - 128));
            }
            3 => {
                clear_vipi3(1 << (vipi_id - 192));
            }
            _ => {
                panic!("Invalid vipi csr id ! {}", csr_id);
            }
        }
    }

    pub fn clear_vipi(vipi_id: u64) {
        let csr_id: i32;

        match vipi_id {
            1..=63 => {
                /* Clear VIPI0 */
                csr_id = 0;
            }
            64..=127 => {
                /* Clear VIPI1 */
                csr_id = 1;
            }
            128..=191 => {
                /* Clear VIPI2 */
                csr_id = 2;
            }
            192..=255 => {
                /* Clear VIPI3 */
                csr_id = 3;
            }
            _ => {
                panic!("Invalid vipi id ! {}", vipi_id);
            }
        }

        VirtualIpi::clear_vipi_bit(csr_id, vipi_id);
    }
    pub fn get_vcpu_id_map(&self, i: u32) -> &AtomicU64 {
        &self.vcpu_id_map[i as usize]
    }

    pub fn vcpu_num(&self) -> u32 {
        self.vcpu_num
    }
}

fn set_vipi0(new_val: u64) {
    #[cfg(feature = "xilinx")]
    stvipi0(new_val);

    #[cfg(feature = "qemu")]
    unsafe {
        csrs!(VIPI0, new_val);
    }
}

fn set_vipi1(new_val: u64) {
    #[cfg(feature = "xilinx")]
    stvipi1(new_val);

    #[cfg(feature = "qemu")]
    unsafe {
        csrs!(VIPI1, new_val);
    }
}

fn set_vipi2(new_val: u64) {
    #[cfg(feature = "xilinx")]
    stvipi2(new_val);

    #[cfg(feature = "qemu")]
    unsafe {
        csrs!(VIPI2, new_val);
    }
}

fn set_vipi3(new_val: u64) {
    #[cfg(feature = "xilinx")]
    stvipi3(new_val);

    #[cfg(feature = "qemu")]
    unsafe {
        csrs!(VIPI3, new_val);
    }
}

fn clear_vipi0(new_val: u64) {
    #[cfg(feature = "xilinx")]
    clvipi0(!new_val);

    #[cfg(feature = "qemu")]
    unsafe {
        csrc!(VIPI0, new_val);
    }
}

fn clear_vipi1(new_val: u64) {
    #[cfg(feature = "xilinx")]
    clvipi1(!new_val);

    #[cfg(feature = "qemu")]
    unsafe {
        csrc!(VIPI1, new_val);
    }
}

fn clear_vipi2(new_val: u64) {
    #[cfg(feature = "xilinx")]
    clvipi2(!new_val);

    #[cfg(feature = "qemu")]
    unsafe {
        csrc!(VIPI2, new_val);
    }
}

fn clear_vipi3(new_val: u64) {
    #[cfg(feature = "xilinx")]
    clvipi3(!new_val);

    #[cfg(feature = "qemu")]
    unsafe {
        csrc!(VIPI3, new_val);
    }
}

/* Write vcpuid by a0 */
#[allow(unused)]
fn wrvcpuid(vcpuid: u64) {
    unsafe {
        asm!(
            ".option push",
            ".option norvc",

            /* wrvcpuid */
            ".word 0xf8a01077",

            ".option pop",
            in("a0") vcpuid,
        );
    }
}

/* Read a0 from vcpuid */
#[allow(unused)]
pub fn rdvcpuid() -> u64 {
    let a0: u64;
    unsafe {
        asm!(
            ".option push",
            ".option norvc",

            /* rdvcpuid */
            ".word 0xf8102577",

            ".option pop",
            out("a0") a0,
        );
    }

    return a0;
}

/* VIPI0 */
#[allow(unused)]
pub fn rdvipi0() -> u64 {
    let a0: u64;
    unsafe {
        asm!(
            ".option push",
            ".option norvc",

            /* rdvipi0 */
            ".word 0xc8101577",

            ".option pop",
            out("a0") a0,
        );
    }

    return a0;
}

/* A0 should be formated as 0b11111101111 */
pub fn clvipi0(a0: u64) {
    unsafe {
        asm!(
            ".option push",
            ".option norvc",

            /* clvipi0 */
            ".word 0xc8a02077",

            ".option pop",
            in("a0") a0,
        );
    }
}

/* A0 should be formated as 0b0000001000 */
#[allow(unused)]
pub fn stvipi0(a0: u64) {
    unsafe {
        asm!(
            ".option push",
            ".option norvc",

            /* stvipi0 */
            ".word 0xc8a03077",

            ".option pop",
            in("a0") a0,
        );
    }
}

/* VIPI1 */
#[allow(unused)]
fn rdvipi1() -> u64 {
    let a0: u64;
    unsafe {
        asm!(
            ".option push",
            ".option norvc",

            /* rdvipi1 */
            ".word 0xd0101577",

            ".option pop",
            out("a0") a0,
        );
    }

    return a0;
}

/* A0 should be formated as 0b11111101111 */
#[allow(unused)]
fn clvipi1(a0: u64) {
    unsafe {
        asm!(
            ".option push",
            ".option norvc",

            /* clvipi1 */
            ".word 0xd0a02077",

            ".option pop",
            in("a0") a0,
        );
    }
}

/* A0 should be formated as 0b0000001000 */
#[allow(unused)]
fn stvipi1(a0: u64) {
    unsafe {
        asm!(
            ".option push",
            ".option norvc",

            /* stvipi1 */
            ".word 0xd0a03077",

            ".option pop",
            in("a0") a0,
        );
    }
}

/* VIPI2 */
#[allow(unused)]
fn rdvipi2() -> u64 {
    let a0: u64;
    unsafe {
        asm!(
            ".option push",
            ".option norvc",

            /* rdvipi2 */
            ".word 0xd8101577",

            ".option pop",
            out("a0") a0,
        );
    }

    return a0;
}

/* A0 should be formated as 0b11111101111 */
#[allow(unused)]
fn clvipi2(a0: u64) {
    unsafe {
        asm!(
            ".option push",
            ".option norvc",

            /* clvipi2 */
            ".word 0xd8a02077",

            ".option pop",
            in("a0") a0,
        );
    }
}

/* A0 should be formated as 0b0000001000 */
#[allow(unused)]
fn stvipi2(a0: u64) {
    unsafe {
        asm!(
            ".option push",
            ".option norvc",

            /* stvipi2 */
            ".word 0xd8a03077",

            ".option pop",
            in("a0") a0,
        );
    }
}

/* VIPI3 */
#[allow(unused)]
fn rdvipi3() -> u64 {
    let a0: u64;
    unsafe {
        asm!(
            ".option push",
            ".option norvc",

            /* rdvipi3 */
            ".word 0xe8101577",

            ".option pop",
            out("a0") a0,
        );
    }

    return a0;
}

/* A0 should be formated as 0b11111101111 */
#[allow(unused)]
fn clvipi3(a0: u64) {
    unsafe {
        asm!(
            ".option push",
            ".option norvc",

            /* clvipi3 */
            ".word 0xe8a02077",

            ".option pop",
            in("a0") a0,
        );
    }
}

/* A0 should be formated as 0b0000001000 */
#[allow(unused)]
fn stvipi3(a0: u64) {
    unsafe {
        asm!(
            ".option push",
            ".option norvc",

            /* stvipi3 */
            ".word 0xe8a03077",

            ".option pop",
            in("a0") a0,
        );
    }
}

#[cfg(test)]
pub mod tests {
    use crate::init::cmdline::MAX_VCPU;
    use crate::irq::vipi::VirtualIpi;
    use crate::mm::gstagemmu::*;
    use crate::mm::utils::*;
    use crate::test::utils::configtest::test_vm_config_create;
    use crate::vcpu::utils::*;
    use crate::vm::virtualmachine;
    use crate::vm::virtualmachine::VIPI_OFFSET;
    use once_cell::sync::Lazy;
    use rusty_fork::rusty_fork_test;
    use std::sync::Mutex;
    use std::{thread, time};

    /*
     * Used by:
     * test_vipi_virtual_ipi_remote_running
     * test_vipi_virtual_ipi_remote_not_running
     * test_vipi_virtual_ipi_remote_each
     * test_vipi_send_to_null_vcpu
     * test_vipi_virtual_ipi_accurate
     */
    pub static mut TEST_SUCCESS_CNT: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(0));

    /*
     * Used by:
     * test_vipi_send_to_null_vcpu
     */
    pub static mut INVALID_TARGET_VCPU: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(0));

    /*
     * Used by:
     * test_vipi_user_ipi_remote
     * test_vipi_user_ipi_remote_multi
     */
    pub static mut GET_UIPI_CNT: Lazy<Mutex<i64>> = Lazy::new(|| Mutex::new(0));

    rusty_fork_test! {
        /*
         * Use an additional thread to send one user ipi to vcpu 0.
         * Vcpu 0 should get one irq caused by the user ipi.
         */
        #[test]
        fn test_vipi_user_ipi_remote() {
            unsafe {
                VIPI_OFFSET = 0;
            }
            let mut vm_config = test_vm_config_create();
            let elf_path: &str
                = "./tests/integration/vipi_user_ipi_remote.img";
            vm_config.set_kernel_img_path(String::from(elf_path));
            let mut vm = virtualmachine::VirtualMachine::new(vm_config);

            vm.vm_init();

            /* Set entry point */
            let entry_point: u64 = vm.get_elf_file().ehdr.entry;

            vm.vcpu(0).set_host_csr(UEPC, entry_point);

            /* Set a0 = vcpu_id */
            vm.vcpu(0).set_guest_gpreg(10, vm.vcpu(0).vcpu_id() as u64);

            /* Set target address for sync */
            /* Target address will be set with 0x1 if the vcpu is ready */
            let target_address = 0x3000;

            /* Add gpa_block for target_address in advance */
            let res = vm.gpa_block_add(target_address, PAGE_SIZE);
            if !res.is_ok() {
                panic!("gpa region add failed!");
            }

            /* Get the hva of 0x3000(gpa) */
            let (hva, hpa) = res.unwrap();
            println!("hva {:x}, hpa {:x}", hva, hpa);

            /* Map the page on g-stage */
            let flag: u64 = PTE_USER | PTE_VALID | PTE_READ | PTE_WRITE
                    | PTE_EXECUTE;
            vm.map_page(target_address, hpa, flag);

            /* Clear target address before the threads run */
            unsafe {
                *(hva as *mut u64) = 0;
            }

            let vmid = vm.get_vmid();
            println!("******Test 0 vmid {}", vmid);

            /* Start a thread to wait for vcpu 1 ready and send user ipi */
            thread::spawn(move || {
                println!("Wait for vcpu 1");

                unsafe {
                    while *(hva as *mut u64) == 0 {
                        let ten_millis = time::Duration::from_millis(10);

                        thread::sleep(ten_millis);
                    }
                }

                unsafe {
                    println!("Vcpu ready! {:x}", *(hva as *mut u64));
                    let target_vipi_id = vmid * (MAX_VCPU as u64) + 1;
                    println!("target_vipi_id: {}", target_vipi_id);

                    /* Send user ipi via VIPI0_CSR */
                    VirtualIpi::set_vipi(target_vipi_id);

                    /*
                     * Set *0x3000 = 2 to drive the vcpu continue to end.
                     * Otherwise the vcpu will loop forever and there will
                     * be no output even eith --nocapture
                     */
                    *(hva as *mut u64) = 2;
                }
            });

            /* Start the test vm */
            vm.vm_run();

            let u_ipi_cnt: i64;

            unsafe {
                u_ipi_cnt = *GET_UIPI_CNT.lock().unwrap();
            }

            println!("Get {} user ipi", u_ipi_cnt);

            vm.vm_destroy();

            /* This test case should only get 1 user ipi and end immediately */
            assert_eq!(1, u_ipi_cnt);
        }

        /*
         * Use an additional thread to send one user ipi to vcpu 1 and no user
         * ipis for vcpu 0. Vcpu 1 and vcpu 0 should get one irq caused by the
         * user ipi in sum.
         */
        #[test]
        fn test_vipi_user_ipi_remote_multi() {
            unsafe {
                VIPI_OFFSET = 1;
            }
            let mut vm_config = test_vm_config_create();
            /* Multi vcpu test */
            vm_config.set_vcpu_count(2);
            let elf_path: &str
                = "./tests/integration/vipi_user_ipi_remote_multi.img";
            vm_config.set_kernel_img_path(String::from(elf_path));
            let mut vm = virtualmachine::VirtualMachine::new(vm_config);

            vm.vm_init();

            /* Set entry point */
            let entry_point: u64 = vm.get_elf_file().ehdr.entry;

            vm.vcpu(0).set_host_csr(UEPC, entry_point);
            vm.vcpu(1).set_host_csr(UEPC, entry_point);

            /* Set a0 = vcpu_id */
            vm.vcpu(0).set_guest_gpreg(10, vm.vcpu(0).vcpu_id() as u64);
            vm.vcpu(1).set_guest_gpreg(10, vm.vcpu(1).vcpu_id() as u64);

            /* Set target address for sync */
            /* Target address will be set with 0x1 if the vcpu is ready */
            let target_address = 0x3000;

            /* Add gpa_block for target_address in advance */
            let res = vm.gpa_block_add(target_address, PAGE_SIZE);
            if !res.is_ok() {
                panic!("gpa region add failed!");
            }

            /* Get the hva of 0x3000(gpa) */
            let (hva, hpa) = res.unwrap();
            println!("hva {:x}, hpa {:x}", hva, hpa);

            /* Map the page on g-stage */
            let flag: u64 = PTE_USER | PTE_VALID | PTE_READ | PTE_WRITE
                    | PTE_EXECUTE;
            vm.map_page(target_address, hpa, flag);

            /* Clear target address before the threads run */
            unsafe {
                *(hva as *mut u64) = 0;
            }

            let vmid = vm.get_vmid();
            println!("******Test 1 vmid {}", vmid);

            /* Start a thread to wait for vcpu 1 ready and send user ipi */
            thread::spawn(move || {
                println!("Wait for vcpu 1");

                unsafe {
                    while *(hva as *mut u64) == 0 {
                        let ten_millis = time::Duration::from_millis(10);

                        thread::sleep(ten_millis);
                    }
                }

                unsafe {
                    println!("Vcpu ready! {:x}", *(hva as *mut u64));
                    let target_vipi_id = vmid * (MAX_VCPU as u64) + 2;
                    println!("target_vipi_id: {}", target_vipi_id);

                    /*
                     * Send user ipi via VIPI0_CSR before change the
                     * sync data.
                     */
                    VirtualIpi::set_vipi(target_vipi_id);

                    /*
                     * Set *0x3000 = 2 to drive the vcpu continue to end.
                     * Otherwise the vcpu will loop forever and there will
                     * be no output even eith --nocapture
                     */
                    *(hva as *mut u64) = 2;
                }
            });

            /* Start the test vm */
            vm.vm_run();

            let u_ipi_cnt: i64;

            unsafe {
                u_ipi_cnt = *GET_UIPI_CNT.lock().unwrap();
            }

            println!("Get {} user ipi", u_ipi_cnt);

            vm.vm_destroy();

            /* This test case should only get 1 user ipi and end immediately */
            assert_eq!(1, u_ipi_cnt);
        }

        /*
         * Vcpu 0 will send vipi to itself to test local vipi.
         * After the vipi is sent, vcpu 0 will wait for it.
         * The vipi will cause vcpu 0 jump into irq_handler and
         * end the test.
         */
        #[test]
        fn test_vipi_virtual_ipi_local() {
            unsafe {
                VIPI_OFFSET = 2;
            }
            let mut vm_config = test_vm_config_create();
            let elf_path: &str
                = "./tests/integration/vipi_virtual_ipi_local.img";
            vm_config.set_kernel_img_path(String::from(elf_path));
            let mut vm = virtualmachine::VirtualMachine::new(vm_config);

            vm.vm_init();

            let vmid = vm.get_vmid();
            println!("******Test 2 vmid {}", vmid);

            /* Set entry point */
            let entry_point: u64 = vm.get_elf_file().ehdr.entry;

            vm.vcpu(0).set_host_csr(UEPC, entry_point);

            /* Start the test vm */
            vm.vm_run();

            vm.vm_destroy();

            /* This test case is passed if the vm_run can bypass the loop */
        }

        /*
         * Vcpu 0 will send a virtual ipi to the running vcpu 1.
         * This test case must ensure that the vcpu 1 is not in HU-mode.
         * So vcpu 1 will set up a signal via sync data before it loops and vpu 0
         * will send the vipi after it get this signal.
         */
        #[test]
        fn test_vipi_virtual_ipi_remote_running() {
            unsafe {
                VIPI_OFFSET = 3;
            }
            let mut vm_config = test_vm_config_create();
            /* Multi vcpu test */
            vm_config.set_vcpu_count(2);
            let elf_path: &str
                = "./tests/integration/vipi_virtual_ipi_remote_running.img";
            vm_config.set_kernel_img_path(String::from(elf_path));
            let mut vm = virtualmachine::VirtualMachine::new(vm_config);

            vm.vm_init();

            let vmid = vm.get_vmid();
            println!("******Test 3 vmid {}", vmid);

            /* Set entry point */
            let entry_point: u64 = vm.get_elf_file().ehdr.entry;

            vm.vcpu(0).set_host_csr(UEPC, entry_point);
            vm.vcpu(1).set_host_csr(UEPC, entry_point);

            /* Set a0 = vcpu_id */
            vm.vcpu(0).set_guest_gpreg(10, vm.vcpu(0).vcpu_id() as u64);
            vm.vcpu(1).set_guest_gpreg(10, vm.vcpu(1).vcpu_id() as u64);

            /* Set target address for sync */
            /* Target address will be set with 0x1 if the vcpu is ready */
            let target_address = 0x3000;

            /* Add gpa_block for target_address in advance */
            let res = vm.gpa_block_add(target_address, PAGE_SIZE);
            if !res.is_ok() {
                panic!("gpa region add failed!");
            }

            /* Get the hva of 0x3000(gpa) */
            let (hva, hpa) = res.unwrap();
            println!("hva {:x}, hpa {:x}", hva, hpa);

            /* Map the page on g-stage */
            let flag: u64 = PTE_USER | PTE_VALID | PTE_READ | PTE_WRITE
                    | PTE_EXECUTE;
            vm.map_page(target_address, hpa, flag);

            /* Clear target address before the threads run */
            unsafe {
                *(hva as *mut u64) = 0;
            }

            /* Start the test vm */
            vm.vm_run();

            let success_cnt: i32;

            unsafe {
                success_cnt = *TEST_SUCCESS_CNT.lock().unwrap();
            }

            println!("Get {} success cnt", success_cnt);

            vm.vm_destroy();

            /* Vcpu 1 should exit from irq_handler */
            assert_eq!(1, success_cnt);
        }

        /*
         * Vcpu 0 will send a virtual ipi to the non-running vcpu 1.
         * This test case must ensure that the vcpu 1 is not in V-mode.
         * So vcpu 1 will ecall SBI_TEST_HU_LOOP to loop in HU-mode and
         * send a signal to vpu 0 the latter will send the vipi after
         * it get the signal.
         */
        #[test]
        fn test_vipi_virtual_ipi_remote_not_running() {
            unsafe {
                VIPI_OFFSET = 4;
            }
            let mut vm_config = test_vm_config_create();
            /* Multi vcpu test */
            vm_config.set_vcpu_count(2);
            let elf_path: &str =
                "./tests/integration/vipi_virtual_ipi_remote_not_running.img";
            vm_config.set_kernel_img_path(String::from(elf_path));
            let mut vm = virtualmachine::VirtualMachine::new(vm_config);

            vm.vm_init();

            let vmid = vm.get_vmid();
            println!("******Test 4 vmid {}", vmid);

            /* Set entry point */
            let entry_point: u64 = vm.get_elf_file().ehdr.entry;

            vm.vcpu(0).set_host_csr(UEPC, entry_point);
            vm.vcpu(1).set_host_csr(UEPC, entry_point);

            /* Set a0 = vcpu_id */
            vm.vcpu(0).set_guest_gpreg(10, vm.vcpu(0).vcpu_id() as u64);
            vm.vcpu(1).set_guest_gpreg(10, vm.vcpu(1).vcpu_id() as u64);

            /* Set target address for sync */
            /* Target address will be set with 0x1 if the vcpu is ready */
            let target_address = 0x3000;

            /* Add gpa_block for target_address in advance */
            let res = vm.gpa_block_add(target_address, PAGE_SIZE);
            if !res.is_ok() {
                panic!("gpa region add failed!");
            }

            /* Get the hva of 0x3000(gpa) */
            let (hva, hpa) = res.unwrap();
            println!("hva {:x}, hpa {:x}", hva, hpa);

            /* Map the page on g-stage */
            let flag: u64 = PTE_USER | PTE_VALID | PTE_READ | PTE_WRITE
                    | PTE_EXECUTE;
            vm.map_page(target_address, hpa, flag);

            /* Clear target address before the threads run */
            unsafe {
                *(hva as *mut u64) = 0;
            }

            /* Set a1 = hva */
            vm.vcpu(0).set_guest_gpreg(11, hva);
            vm.vcpu(1).set_guest_gpreg(11, hva);

            /* Start the test vm */
            vm.vm_run();

            let success_cnt: i32;

            unsafe {
                success_cnt = *TEST_SUCCESS_CNT.lock().unwrap();
            }

            println!("Get {} success cnt", success_cnt);

            vm.vm_destroy();

            /* Vcpu 1 should exit from irq_handler */
            assert_eq!(1, success_cnt);
        }

        /*
         * Vcpu 0 will send a vipi to vcpu 1 and vcpu1 will response via a
         * vipi.
         * Only when vcpu 0 get the vipi sent by vcpu 1 and trigger
         * irq_hadnler_0, the test case can pass.
         */
        #[test]
        fn test_vipi_virtual_ipi_remote_each() {
            unsafe {
                VIPI_OFFSET = 5;
            }
            let mut vm_config = test_vm_config_create();
            /* Multi vcpu test */
            vm_config.set_vcpu_count(2);
            let elf_path: &str
                = "./tests/integration/vipi_virtual_ipi_remote_each.img";
            vm_config.set_kernel_img_path(String::from(elf_path));
            let mut vm = virtualmachine::VirtualMachine::new(vm_config);

            vm.vm_init();

            let vmid = vm.get_vmid();
            println!("******Test 5 vmid {}", vmid);

            /* Set entry point */
            let entry_point: u64 = vm.get_elf_file().ehdr.entry;

            vm.vcpu(0).set_host_csr(UEPC, entry_point);
            vm.vcpu(1).set_host_csr(UEPC, entry_point);

            /* Set a0 = vcpu_id */
            vm.vcpu(0).set_guest_gpreg(10, vm.vcpu(0).vcpu_id() as u64);
            vm.vcpu(1).set_guest_gpreg(10, vm.vcpu(1).vcpu_id() as u64);

            /* Set target address for sync */
            /* Target address will be set with 0x1 if the vcpu is ready */
            let target_address = 0x3000;

            /* Add gpa_block for target_address in advance */
            let res = vm.gpa_block_add(target_address, PAGE_SIZE);
            if !res.is_ok() {
                panic!("gpa region add failed!");
            }

            /* Get the hva of 0x3000(gpa) */
            let (hva, hpa) = res.unwrap();
            println!("hva {:x}, hpa {:x}", hva, hpa);

            /* Map the page on g-stage */
            let flag: u64 = PTE_USER | PTE_VALID | PTE_READ | PTE_WRITE
                    | PTE_EXECUTE;
            vm.map_page(target_address, hpa, flag);

            /* Clear target address before the threads run */
            unsafe {
                *(hva as *mut u64) = 0;
            }

            /* Set a1 = hva */
            vm.vcpu(0).set_guest_gpreg(11, hva);
            vm.vcpu(1).set_guest_gpreg(11, hva);

            /* Start the test vm */
            vm.vm_run();

            let success_cnt: i32;

            unsafe {
                success_cnt = *TEST_SUCCESS_CNT.lock().unwrap();
            }

            println!("Get {} success cnt", success_cnt);

            vm.vm_destroy();

            /* Vcpu 1 should exit from irq_handler */
            assert_eq!(1, success_cnt);
        }

        /*
         * Vcpu 0 will send vipi to vcpu [1,2,3,4,5,6,7] and the vm has only
         * 2 vcpus. Vcpu 1 should get the vipi, but the vipis for vcpu
         * [2,3,4,5,6,7] should be detected and throw out. INVALID_TARGET_VCPU
         * is the count value of vipis with invalid target vcpus.
         */
        #[test]
        fn test_vipi_send_to_null_vcpu() {
            unsafe {
                VIPI_OFFSET = 6;
            }
            let mut vm_config = test_vm_config_create();
            /* Multi vcpu test */
            vm_config.set_vcpu_count(2);
            let elf_path: &str
                = "./tests/integration/vipi_send_to_null_vcpu.img";
            vm_config.set_kernel_img_path(String::from(elf_path));
            let mut vm = virtualmachine::VirtualMachine::new(vm_config);

            vm.vm_init();

            let vmid = vm.get_vmid();
            println!("******Test 6 vmid {}", vmid);

            /* Set entry point */
            let entry_point: u64 = vm.get_elf_file().ehdr.entry;

            vm.vcpu(0).set_host_csr(UEPC, entry_point);
            vm.vcpu(1).set_host_csr(UEPC, entry_point);

            /* Set a0 = vcpu_id */
            vm.vcpu(0).set_guest_gpreg(10, vm.vcpu(0).vcpu_id() as u64);
            vm.vcpu(1).set_guest_gpreg(10, vm.vcpu(1).vcpu_id() as u64);

            /* Set target address for sync */
            /* Target address will be set with 0x1 if the vcpu is ready */
            let target_address = 0x3000;

            /* Add gpa_block for target_address in advance */
            let res = vm.gpa_block_add(target_address, PAGE_SIZE);
            if !res.is_ok() {
                panic!("gpa region add failed!");
            }

            /* Get the hva of 0x3000(gpa) */
            let (hva, hpa) = res.unwrap();
            println!("hva {:x}, hpa {:x}", hva, hpa);

            /* Map the page on g-stage */
            let flag: u64 = PTE_USER | PTE_VALID | PTE_READ | PTE_WRITE
                    | PTE_EXECUTE;
            vm.map_page(target_address, hpa, flag);

            /* Clear target address before the threads run */
            unsafe {
                *(hva as *mut u64) = 0;
            }

            /* Set a1 = hva */
            vm.vcpu(0).set_guest_gpreg(11, hva);
            vm.vcpu(1).set_guest_gpreg(11, hva);

            /* Start the test vm */
            vm.vm_run();

            vm.vm_destroy();

            let invalid_cnt: i32;

            unsafe {
                invalid_cnt = *INVALID_TARGET_VCPU.lock().unwrap();
            }

            println!("Get {} invalid cnt", invalid_cnt);

            /* Target vcpu [2,3,4,5,6,7] is invalid */
            assert_eq!(6, invalid_cnt);

            /* Vcpu 0 should exit from test_success */
            let success_cnt: i32;

            unsafe {
                success_cnt = *TEST_SUCCESS_CNT.lock().unwrap();
            }

            println!("Get {} success cnt", success_cnt);

            assert_eq!(1, success_cnt);
        }

        /*
         * Vcpu 1 and 2 will wait for the vipi send by vcpu 0.
         * But vcpu 0 will only send one vipi to vcpu 1. So vcpu
         * 1 should exit from irq_handler_1 but vcpu 2 should not.
         */
        #[test]
        fn test_vipi_virtual_ipi_accurate() {
            unsafe {
                VIPI_OFFSET = 7;
            }
            let mut vm_config = test_vm_config_create();
            /* Multi vcpu test */
            vm_config.set_vcpu_count(3);
            let elf_path: &str
                = "./tests/integration/vipi_virtual_ipi_accurate.img";
            vm_config.set_kernel_img_path(String::from(elf_path));
            let mut vm = virtualmachine::VirtualMachine::new(vm_config);

            vm.vm_init();

            let vmid = vm.get_vmid();
            println!("******Test 7 vmid {}", vmid);

            /* Set entry point */
            let entry_point: u64 = vm.get_elf_file().ehdr.entry;

            vm.vcpu(0).set_host_csr(UEPC, entry_point);
            vm.vcpu(1).set_host_csr(UEPC, entry_point);
            vm.vcpu(2).set_host_csr(UEPC, entry_point);

            /* Set a0 = vcpu_id */
            vm.vcpu(0).set_guest_gpreg(10, vm.vcpu(0).vcpu_id() as u64);
            vm.vcpu(1).set_guest_gpreg(10, vm.vcpu(1).vcpu_id() as u64);
            vm.vcpu(2).set_guest_gpreg(10, vm.vcpu(2).vcpu_id() as u64);

            /* Set target address for sync */
            /* Target address will be set with 0x1 if the vcpu is ready */
            let target_address = 0x3000;

            /* Add gpa_block for target_address in advance */
            let res = vm.gpa_block_add(target_address, PAGE_SIZE);
            if !res.is_ok() {
                panic!("gpa region add failed!");
            }

            /* Get the hva of 0x3000(gpa) */
            let (hva, hpa) = res.unwrap();
            println!("hva {:x}, hpa {:x}", hva, hpa);

            /* Map the page on g-stage */
            let flag: u64 = PTE_USER | PTE_VALID | PTE_READ | PTE_WRITE
                    | PTE_EXECUTE;
            vm.map_page(target_address, hpa, flag);

            /* Clear target address before the threads run */
            unsafe {
                *(hva as *mut u64) = 0;
            }

            /* Start the test vm */
            vm.vm_run();

            let success_cnt: i32;

            unsafe {
                success_cnt = *TEST_SUCCESS_CNT.lock().unwrap();
            }

            println!("Get {} success cnt", success_cnt);

            vm.vm_destroy();

            /*
             * Vcpu 0 and 2 should not exit from irq_handler and
             * trigger SBI_TEST_SUCCESS both. Vcpu 1 should exit from
             * irq_handler once. So the answer is 3.
             */
            assert_eq!(3, success_cnt);
        }
    }
}

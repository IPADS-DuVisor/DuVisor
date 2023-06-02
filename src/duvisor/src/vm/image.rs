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

use std::path::PathBuf;

/* File type for vm image */
pub const IMAGE_TYPE_ELF: u8 = 1;
pub const IMAGE_TYPE_DATA: u8 = 2;

/* Linux image const */
pub const RISCV_RAM_GPA_START: u64 = 0x80000000;
pub const KERNEL_OFFSET: u64 = 0x200000;

/* Read and parse the vm img file */
pub struct VmImage {
    elf_file: elf::File,
    file_data: Vec<u8>,
    file_type: u8,
}

impl VmImage {
    pub fn new(file_path: &str) -> Self {
        /* Parse ELF file */
        let elf_file: elf::File;
        let elf_wrap = VmImage::elf_parse(file_path);
        let file_type: u8;

        if elf_wrap.is_none() {
            /* VM image is not ELF */
            elf_file = elf::File::new();
            file_type = IMAGE_TYPE_DATA;
        } else {
            elf_file = elf_wrap.unwrap();
            file_type = IMAGE_TYPE_ELF;
        }

        let file_data = std::fs::read(file_path).unwrap_or_else(|_| {
            panic!("read file failed");
        });

        Self {
            elf_file,
            file_data,
            file_type,
        }
    }

    pub fn elf_parse(elf_path: &str) -> Option<elf::File> {
        let path = PathBuf::from(elf_path);
        let file = match elf::File::open_path(&path) {
            Ok(f) => Some(f),
            Err(_e) => None,
        };

        file
    }

    /* Get Image file data */
    pub fn get_file_data(&self) -> &Vec<u8> {
        &self.file_data
    }

    /* Get Image file type */
    pub fn get_file_type(&self) -> u8 {
        self.file_type
    }

    /* Get ELF file structure (Metadata) */
    pub fn get_elf_file(&self) -> &elf::File {
        &self.elf_file
    }
}

#[cfg(test)]
mod tests {
    use crate::test::utils::configtest::test_vm_config_create;
    use crate::vm::*;
    use libc::c_void;
    use rusty_fork::rusty_fork_test;

    rusty_fork_test! {
        #[test]
        fn test_image_type_data() {
            let mut vm_config = test_vm_config_create();
            vm_config.set_kernel_img_path(
                String::from("./test-files-duvisor/Image"));

            let vm = virtualmachine::VirtualMachine::new(vm_config);
            let kernel_type = vm.image_file_type();

            assert_eq!(kernel_type, image::IMAGE_TYPE_DATA);
        }

        #[test]
        fn test_image_type_elf() {
            let mut vm_config = test_vm_config_create();
            vm_config.set_kernel_img_path(
                    String::from("./tests/integration/vcpu_add_all_gprs.img"));

            let vm = virtualmachine::VirtualMachine::new(vm_config);
            let kernel_type = vm.image_file_type();

            assert_eq!(kernel_type, image::IMAGE_TYPE_ELF);
        }

        #[test]
        fn test_image_load() {
            let mut vm_config = test_vm_config_create();
            let kernel_path: &str = "./test-files-duvisor/Image";
            vm_config.set_kernel_img_path(String::from(kernel_path));
            let mut vm = virtualmachine::VirtualMachine::new(vm_config);
            let kernel_hva: u64 = vm.vm_init()[0];

            let ans_res = std::fs::read(kernel_path);
            if ans_res.is_err() {
                panic!("Ans kernel load failed");
            }
            let ans_data = ans_res.unwrap();

            let result: i32;
            unsafe {
                result = libc::memcmp(kernel_hva as *const c_void,
                        ans_data.as_ptr() as *const c_void,
                        ans_data.len());
            }

            assert_eq!(result, 0);
        }

        #[test]
        fn test_image_zero_size() {
            let mut vm_config = test_vm_config_create();
            vm_config.set_kernel_img_path(
                String::from("./test-files-duvisor/null-kernel-image.img"));
            let mut vm = virtualmachine::VirtualMachine::new(vm_config);
            let hva_list = vm.vm_init();
            let length = hva_list.len();

            /* There should not be any data loaded */
            assert_eq!(length, 0);
        }
    }
}

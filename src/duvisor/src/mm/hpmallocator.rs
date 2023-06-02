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

use crate::dbgprintln;
use crate::mm::hpmallocator::hpm_constants::*;
use crate::mm::utils::*;
use crate::plat::uhe::ioctl::ioctl_constants;
use ioctl_constants::*;

#[derive(Clone)]
pub struct HpmRegion {
    hpm_vptr: u64,     /* VA */
    base_address: u64, /* HPA */
    length: u64,
    offset: u64,
}

#[allow(unused)]
mod hpm_constants {
    pub const DEFAULT_PMP_SIZE_QEMU: u64 = 512 << 20;
    pub const DEFAULT_PMP_SIZE_XILINX: u64 = 128 << 20;
}

impl HpmRegion {
    pub fn new(hpm_vptr: u64, base_address: u64, length: u64) -> Self {
        Self {
            hpm_vptr,
            base_address,
            length,
            offset: 0,
        }
    }

    pub fn get_hpm_vpr(&self) -> u64 {
        self.hpm_vptr
    }

    pub fn get_base_address(&self) -> u64 {
        self.base_address
    }

    pub fn get_length(&self) -> u64 {
        self.length
    }

    pub fn va_to_hpa(&self, va: u64) -> Option<u64> {
        va_to_hpa_helper(self.hpm_vptr, self.base_address, va, self.length)
    }

    pub fn hpa_to_va(&self, hpa: u64) -> Option<u64> {
        hpa_to_va_helper(self.hpm_vptr, self.base_address, hpa, self.length)
    }
}

pub struct HpmAllocator {
    hpm_region_list: Vec<HpmRegion>,
    ioctl_fd: i32,
    mem_size: u64,
}

impl HpmAllocator {
    pub fn new(ioctl_fd: i32, mem_size: u64) -> Self {
        Self {
            hpm_region_list: Vec::new(),
            ioctl_fd,
            mem_size: mem_size + PAGE_TABLE_REGION_SIZE,
        }
    }

    /* Call PMP for hpa region */
    pub fn pmp_alloc(&mut self) -> Option<HpmRegion> {
        let fd = self.ioctl_fd;
        let pmp_buf_va: u64; /* VA */
        let mut pmp_buf_pa: u64; /* HPA */

        #[cfg(feature = "xilinx")]
        let mut pmp_buf_size: u64 = DEFAULT_PMP_SIZE_XILINX;
        #[cfg(feature = "qemu")]
        let mut pmp_buf_size: u64 = DEFAULT_PMP_SIZE_QEMU;

        if pmp_buf_size <= self.mem_size {
            pmp_buf_size = self.mem_size;
        }
        let version: u64 = 0;
        println!("{:#x}", pmp_buf_size);

        unsafe {
            let version_ptr = (&version) as *const u64;
            libc::ioctl(fd, IOCTL_DUVISOR_GET_API_VERSION, version_ptr);

            /*
             * Call `mmap` to request DV-driver to
             * allocate a continuous region physical memory.
             * The return value of this `mmap` is hva
             * of the starting address of this region.
             */
            let addr = 0 as *mut libc::c_void;
            let mmap_ptr = libc::mmap(
                addr,
                pmp_buf_size as usize,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED,
                fd,
                0,
            );
            assert_ne!(mmap_ptr, libc::MAP_FAILED);

            /*
             * Call `ioctl` to request DV-driver to
             * return the hpa of the starting address of this region.
             */
            pmp_buf_va = mmap_ptr as u64;
            pmp_buf_pa = pmp_buf_va;
            let pmp_buf_pa_ptr = (&mut pmp_buf_pa) as *mut u64;
            libc::ioctl(fd, IOCTL_DUVISOR_QUERY_PFN, pmp_buf_pa_ptr);
        }

        let hpm_vptr = pmp_buf_va as u64;
        let base_address = pmp_buf_pa << 12;
        let length = pmp_buf_size as u64;

        self.ioctl_fd = fd;

        Some(HpmRegion::new(hpm_vptr, base_address, length))
    }

    pub fn find_hpm_region_by_length(&mut self, length: u64) -> Option<&mut HpmRegion> {
        let mut rest: u64;

        for i in &mut self.hpm_region_list {
            rest = i.length - i.offset;

            if length <= rest {
                return Some(i);
            }
        }

        None
    }

    /* Length could only be PAGE_TABLE_REGION_SIZE or PAGE_SIZE */
    pub fn hpm_alloc(&mut self, gpa_offset: u64, length: u64) -> Option<Vec<HpmRegion>> {
        let target_hpm_region: &mut HpmRegion;
        let mut result: Vec<HpmRegion> = Vec::new();
        let result_va: u64;
        let result_pa: u64;
        let result_length: u64;

        loop {
            let target_wrap = self.find_hpm_region_by_length(length);

            if target_wrap.is_some() {
                /* Physical memory is enough */
                target_hpm_region = target_wrap.unwrap();

                result_va = target_hpm_region.hpm_vptr + gpa_offset;
                result_pa = target_hpm_region.base_address + gpa_offset;
                result_length = length;

                result.push(HpmRegion::new(result_va, result_pa, result_length));

                return Some(result);
            } else {
                /* Run out of physical memory (unlikely) */
                dbgprintln!("--- Call pmp_alloc for more physical memory.");

                /* Get a new hpm region by pmp_alloc */
                let res = self.pmp_alloc();
                if res.is_some() {
                    let hpm_region = res.unwrap();
                    self.hpm_region_list.push(hpm_region);
                } else {
                    println!("Run out of physical memory!");
                    break;
                }
            }
        }

        None
    }

    pub fn set_ioctl_fd(&mut self, ioctl_fd: i32) {
        self.ioctl_fd = ioctl_fd;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusty_fork::rusty_fork_test;
    use std::ffi::CString;

    rusty_fork_test! {
        #[test]
        fn test_hpm_region_new() {
            let hpa: u64 = 0x3000;
            let va: u64 = 0x5000;
            let length: u64 = 0x1000;
            let hpm_vptr = va as u64;
            let hpm_region = HpmRegion::new(hpm_vptr, hpa, length);

            assert_eq!(hpm_region.hpm_vptr, va);
            assert_eq!(hpm_region.base_address, hpa);
            assert_eq!(hpm_region.length, length);
        }

        /* Check new() of GStageMmu */
        #[test]
        fn test_allocator_alloc() {
            let file_path = CString::new("/dev/dv_driver").unwrap();
            let ioctl_fd;

            unsafe {
                ioctl_fd =
                    (libc::open(file_path.as_ptr(), libc::O_RDWR)) as i32;
            }

            let length = 0x2000;
            let mut allocator = HpmAllocator::new(ioctl_fd, 512 << MB_SHIFT);
            let result_wrap = allocator.hpm_alloc(0, length);
            assert!(result_wrap.is_some());

            let result = result_wrap.unwrap();
            let result_length = result.len();
            assert_eq!(1, result_length);

            let mut region_length = 0;

            for i in result {
                region_length = i.length;
            }

            assert_eq!(region_length, length);
        }

        /* Check hpa_to_va when hpa is out of bound */
        #[test]
        fn test_hpa_to_va_oob_invalid() {
            let file_path = CString::new("/dev/dv_driver").unwrap();
            let ioctl_fd;

            unsafe {
                ioctl_fd =
                    (libc::open(file_path.as_ptr(), libc::O_RDWR)) as i32;
            }

            /* Valid HPA: [base_addr, base_addr + 0x2000) */
            let length = 0x2000;
            let mut allocator = HpmAllocator::new(ioctl_fd, 512 << MB_SHIFT);
            let result_wrap = allocator.hpm_alloc(0, length);
            assert!(result_wrap.is_some());

            let result = result_wrap.unwrap();
            let result_length = result.len();
            assert_eq!(1, result_length);

            let mut invalid_hpa;
            let mut res;

            for i in result {
                invalid_hpa = i.base_address;
                invalid_hpa += i.length * 2;
                res = i.hpa_to_va(invalid_hpa);
                if res.is_some() {
                    panic!("HPA {:x} should be out of bound", invalid_hpa);
                }
            }
        }

        /* Check hpa_to_va when hpa is equal to the upper boundary */
        #[test]
        fn test_hpa_to_va_oob_invalid_eq() {
            let file_path = CString::new("/dev/dv_driver").unwrap();
            let ioctl_fd;

            unsafe {
                ioctl_fd =
                    (libc::open(file_path.as_ptr(), libc::O_RDWR)) as i32;
            }

            /* Valid HPA: [base_addr, base_addr + 0x2000) */
            let length = 0x2000;
            let mut allocator = HpmAllocator::new(ioctl_fd, 512 << MB_SHIFT);

            let result_wrap = allocator.hpm_alloc(0, length);
            assert!(result_wrap.is_some());

            let result = result_wrap.unwrap();
            let result_length = result.len();
            assert_eq!(1, result_length);

            let mut invalid_hpa;
            let mut res;

            for i in result {
                invalid_hpa = i.base_address;
                invalid_hpa += i.length;
                res = i.hpa_to_va(invalid_hpa);
                if res.is_some() {
                    panic!("HPA {:x} should be out of bound", invalid_hpa);
                }
            }
        }

        /* Check hpa_to_va when hpa is valid */
        #[test]
        fn test_hpa_to_va_oob_valid() {
            let file_path = CString::new("/dev/dv_driver").unwrap();
            let ioctl_fd;

            unsafe {
                ioctl_fd =
                    (libc::open(file_path.as_ptr(), libc::O_RDWR)) as i32;
            }

            /* Valid HPA: [base_addr, base_addr + 0x2000) */
            let length = 0x2000;
            let mut allocator = HpmAllocator::new(ioctl_fd, 512 << MB_SHIFT);

            let result_wrap = allocator.hpm_alloc(0, length);
            assert!(result_wrap.is_some());

            let result = result_wrap.unwrap();
            let result_length = result.len();
            assert_eq!(1, result_length);

            let mut valid_hpa;
            let mut res;

            for i in result {
                valid_hpa = i.base_address;
                valid_hpa += i.length / 2;
                res = i.hpa_to_va(valid_hpa);
                if res.is_none() {
                    panic!("HPA {:x} should be valid", valid_hpa);
                }
            }
        }

        /* Check hpa_to_va when hpa is equal to the lower bound */
        #[test]
        fn test_hpa_to_va_oob_valid_eq() {
            let file_path = CString::new("/dev/dv_driver").unwrap();
            let ioctl_fd;

            unsafe {
                ioctl_fd =
                    (libc::open(file_path.as_ptr(), libc::O_RDWR)) as i32;
            }

            /* Valid HPA: [base_addr, base_addr + 0x2000) */
            let length = 0x2000;
            let mut allocator = HpmAllocator::new(ioctl_fd, 512 << MB_SHIFT);

            let result_wrap = allocator.hpm_alloc(0, length);
            assert!(result_wrap.is_some());

            let result = result_wrap.unwrap();
            let result_length = result.len();
            assert_eq!(1, result_length);

            let mut valid_hpa;
            let mut res;

            for i in result {
                valid_hpa = i.base_address;
                res = i.hpa_to_va(valid_hpa);
                if res.is_none() {
                    panic!("HPA {:x} should be valid", valid_hpa);
                }
            }
        }

        /* Check va_to_hpa when va is out of bound */
        #[test]
        fn test_va_to_hpa_oob_invalid() {
            let file_path = CString::new("/dev/dv_driver").unwrap();
            let ioctl_fd;

            unsafe {
                ioctl_fd =
                    (libc::open(file_path.as_ptr(), libc::O_RDWR)) as i32;
            }

            let length = 0x2000;
            let mut allocator = HpmAllocator::new(ioctl_fd, 512 << MB_SHIFT);

            let result_wrap = allocator.hpm_alloc(0, length);
            assert!(result_wrap.is_some());

            let result = result_wrap.unwrap();
            let result_length = result.len();
            assert_eq!(1, result_length);

            let mut invalid_va;
            let mut res;

            for i in result {
                invalid_va = i.hpm_vptr + length + 0x1000;
                res = i.va_to_hpa(invalid_va);
                if res.is_some() {
                    panic!("VA {:x} should be out of bound", invalid_va);
                }
            }
        }

        /* Check va_to_hpa when va is equal to the upper bound */
        #[test]
        fn test_va_to_hpa_oob_invalid_eq() {
            let file_path = CString::new("/dev/dv_driver").unwrap();
            let ioctl_fd;

            unsafe {
                ioctl_fd =
                    (libc::open(file_path.as_ptr(), libc::O_RDWR)) as i32;
            }

            let length = 0x2000;
            let mut allocator = HpmAllocator::new(ioctl_fd, 512 << MB_SHIFT);

            let result_wrap = allocator.hpm_alloc(0, length);
            assert!(result_wrap.is_some());

            let result = result_wrap.unwrap();
            let result_length = result.len();
            assert_eq!(1, result_length);

            let mut invalid_va;
            let mut res;

            for i in result {
                invalid_va = i.hpm_vptr + length;
                res = i.va_to_hpa(invalid_va);
                if res.is_some() {
                    panic!("VA {:x} should be out of bound", invalid_va);
                }
            }
        }

        /* Check va_to_hpa when va is valid */
        #[test]
        fn test_va_to_hpa_oob_valid() {
            let file_path = CString::new("/dev/dv_driver").unwrap();
            let ioctl_fd;

            unsafe {
                ioctl_fd =
                    (libc::open(file_path.as_ptr(), libc::O_RDWR)) as i32;
            }

            let length = 0x2000;
            let mut allocator = HpmAllocator::new(ioctl_fd, 512 << MB_SHIFT);

            let result_wrap = allocator.hpm_alloc(0, length);
            assert!(result_wrap.is_some());

            let result = result_wrap.unwrap();
            let result_length = result.len();
            assert_eq!(1, result_length);

            let mut valid_va;
            let mut res;

            for i in result {
                valid_va = i.hpm_vptr + length - 0x1000;
                res = i.va_to_hpa(valid_va);
                if res.is_none() {
                    panic!("VA {:x} should be valid", valid_va);
                }
            }
        }

        /* Check va_to_hpa when va is equal to the lower bound */
        #[test]
        fn test_va_to_hpa_oob_valid_eq() {
            let file_path = CString::new("/dev/dv_driver").unwrap();
            let ioctl_fd;

            unsafe {
                ioctl_fd =
                    (libc::open(file_path.as_ptr(), libc::O_RDWR)) as i32;
            }

            let length = 0x2000;
            let mut allocator = HpmAllocator::new(ioctl_fd, 512 << MB_SHIFT);

            let result_wrap = allocator.hpm_alloc(0, length);
            assert!(result_wrap.is_some());

            let result = result_wrap.unwrap();
            let result_length = result.len();
            assert_eq!(1, result_length);

            let mut valid_va;
            let mut res;

            for i in result {
                valid_va = i.hpm_vptr;
                res = i.va_to_hpa(valid_va);
                if res.is_none() {
                    panic!("VA {:x} should be valid", valid_va);
                }
            }
        }
    }
}

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

/* User Hypervisor Extension (UHE) */

#[allow(unused)]
pub mod ioctl_constants {
    /* Ioctl id */
    pub const IOCTL_DUVISOR_GET_API_VERSION: u64 = 0x80086B01;
    pub const IOCTL_DUVISOR_REQUEST_DELEG: u64 = 0x40106B03;
    pub const IOCTL_DUVISOR_REGISTER_VCPU: u64 = 0x6B04;
    pub const IOCTL_DUVISOR_UNREGISTER_VCPU: u64 = 0x6B05;
    pub const IOCTL_DUVISOR_QUERY_PFN: u64 = 0xc0086b06;
    pub const IOCTL_DUVISOR_RELEASE_PFN: u64 = 0x40086b07;
    pub const IOCTL_REMOTE_FENCE: u64 = 0x80106b08;
    pub const IOCTL_DUVISOR_GET_VMID: u64 = 0x80086b09;
    pub const IOCTL_DUVISOR_GET_CPUID: u64 = 0x80086b0b;
}

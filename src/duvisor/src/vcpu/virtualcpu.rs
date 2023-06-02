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

use crate::csrc;
#[allow(unused)]
use crate::csrr;
use crate::csrw;
use crate::dbgprintln;
use crate::devices::plic::Plic;
use crate::init::cmdline::MAX_VCPU;
use crate::irq::delegation::delegation_constants::*;
#[allow(unused)]
use crate::irq::vipi::rdvcpuid;
use crate::irq::vipi::VirtualIpi;
use crate::irq::virq;
use crate::mm::gstagemmu::*;
use crate::mm::utils::*;
use crate::plat::opensbi;
use crate::plat::opensbi::emulation::SHUTDOWN_FLAG;
use crate::plat::uhe::csr::csr_constants;
use crate::plat::uhe::ioctl::ioctl_constants::*;
use crate::vcpu::utils::*;
use crate::vcpu::vcpucontext;
use crate::vm::dtb::PLIC_HPA;
use crate::vm::dtb::PLIC_LENGTH;
use crate::vm::virtualmachine;
use atomic_enum::*;
use core::arch::asm;
use csr_constants::*;
use once_cell::sync::OnceCell;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use vcpucontext::*;

#[cfg(test)]
use crate::irq::vipi::tests::GET_UIPI_CNT;

extern crate irq_util;
use irq_util::IrqChip;

extern crate devices;
extern crate sys_util;
use sys_util::GuestMemory;

#[allow(unused)]
mod errno_constants {
    pub const EFAILED: i32 = -1;
    pub const ENOPERMIT: i32 = -2;
    pub const ENOMAPPING: i32 = -3;
}
pub use errno_constants::*;

mod inst_parsing_constants {
    pub const INST_OPCODE_MASK: u32 = 0x007c;
    pub const INST_OPCODE_SHIFT: u32 = 2;
    pub const INST_OPCODE_SYSTEM: u32 = 28;

    pub const INST_MASK_WFI: u32 = 0xffffff00;
    pub const INST_MATCH_WFI: u32 = 0x10500000;

    pub const INST_MATCH_LB: u32 = 0x3;
    pub const INST_MASK_LB: u32 = 0x707f;
    pub const INST_MATCH_LH: u32 = 0x1003;
    pub const INST_MASK_LH: u32 = 0x707f;
    pub const INST_MATCH_LW: u32 = 0x2003;
    pub const INST_MASK_LW: u32 = 0x707f;
    pub const INST_MATCH_LD: u32 = 0x3003;
    pub const INST_MASK_LD: u32 = 0x707f;
    pub const INST_MATCH_LBU: u32 = 0x4003;
    pub const INST_MASK_LBU: u32 = 0x707f;
    pub const INST_MATCH_LHU: u32 = 0x5003;
    pub const INST_MASK_LHU: u32 = 0x707f;
    pub const INST_MATCH_LWU: u32 = 0x6003;
    pub const INST_MASK_LWU: u32 = 0x707f;
    pub const INST_MATCH_SB: u32 = 0x23;
    pub const INST_MASK_SB: u32 = 0x707f;
    pub const INST_MATCH_SH: u32 = 0x1023;
    pub const INST_MASK_SH: u32 = 0x707f;
    pub const INST_MATCH_SW: u32 = 0x2023;
    pub const INST_MASK_SW: u32 = 0x707f;
    pub const INST_MATCH_SD: u32 = 0x3023;
    pub const INST_MASK_SD: u32 = 0x707f;

    pub const INST_MATCH_C_LD: u32 = 0x6000;
    pub const INST_MASK_C_LD: u32 = 0xe003;
    pub const INST_MATCH_C_SD: u32 = 0xe000;
    pub const INST_MASK_C_SD: u32 = 0xe003;
    pub const INST_MATCH_C_LW: u32 = 0x4000;
    pub const INST_MASK_C_LW: u32 = 0xe003;
    pub const INST_MATCH_C_SW: u32 = 0xc000;
    pub const INST_MASK_C_SW: u32 = 0xe003;
    pub const INST_MATCH_C_LDSP: u32 = 0x6002;
    pub const INST_MASK_C_LDSP: u32 = 0xe003;
    pub const INST_MATCH_C_SDSP: u32 = 0xe002;
    pub const INST_MASK_C_SDSP: u32 = 0xe003;
    pub const INST_MATCH_C_LWSP: u32 = 0x4002;
    pub const INST_MASK_C_LWSP: u32 = 0xe003;
    pub const INST_MATCH_C_SWSP: u32 = 0xc002;
    pub const INST_MASK_C_SWSP: u32 = 0xe003;
}
pub use inst_parsing_constants::*;

use super::vcpucontext::gp_reg_constants::*;

pub const ECALL_VM_TEST_END: u64 = 0xFF;

#[atomic_enum]
#[derive(PartialEq)]
pub enum ExitReason {
    ExitUnknown,
    ExitEaccess,
    ExitMmio,
    ExitIntr,
    ExitSystemEvent,
    ExitRiscvSbi,
}

#[allow(unused)]
#[link(name = "enter_guest")]
extern "C" {
    fn enter_guest(vcpuctx: u64) -> i32;
    fn exit_guest();
}

#[allow(unused)]
#[link(name = "vtimer")]
extern "C" {
    fn wrvtimectl(val: u64);
}

#[allow(unused)]
extern "C" {
    fn vcpu_ecall_exit();
    fn vcpu_ecall_exit_end();
    fn vcpu_add_all_gprs();
    fn vcpu_add_all_gprs_end();
    fn vmem_ld_mapping();
    fn vmem_ld_mapping_end();
    fn vmem_W_Ro();
    fn vmem_W_Ro_end();
    fn vmem_X_nonX();
    fn vmem_X_nonX_end();
    fn vmem_ld_sd_over_loop();
    fn vmem_ld_sd_over_loop_end();
}

#[allow(dead_code)]
pub struct VirtualCpu {
    vcpu_id: u32,
    vm: Arc<virtualmachine::VmSharedState>,
    vipi: Arc<VirtualIpi>,
    vcpu_ctx: Mutex<VcpuCtx>,
    virq: virq::VirtualInterrupt,
    /* Cell for late init */
    irqchip: OnceCell<Arc<dyn IrqChip>>,
    /* TODO: irq_pending with shared memory */
    exit_reason: AtomicExitReason,
    guest_mem: GuestMemory,
    mmio_bus: Arc<RwLock<devices::Bus>>,
    is_running: AtomicBool,
}

impl VirtualCpu {
    pub fn new(
        vcpu_id: u32,
        vm_state: Arc<virtualmachine::VmSharedState>,
        guest_mem: GuestMemory,
        mmio_bus: Arc<RwLock<devices::Bus>>,
        vipi_ptr: Arc<VirtualIpi>,
    ) -> Self {
        let vcpu_ctx = Mutex::new(VcpuCtx::new());
        let virq = virq::VirtualInterrupt::new();
        let exit_reason = AtomicExitReason::new(ExitReason::ExitUnknown);
        let irqchip = OnceCell::new();
        let is_running = AtomicBool::new(false);

        Self {
            vcpu_id,
            vm: vm_state,
            vcpu_ctx,
            virq,
            irqchip,
            exit_reason,
            guest_mem,
            mmio_bus,
            vipi: vipi_ptr,
            is_running,
        }
    }

    fn config_hugatp(&self, vcpu_ctx: &mut VcpuCtx) -> u64 {
        let pt_pfn: u64 = self.vm.get_pt_pfn();
        let hugatp: u64;

        if S2PT_MODE == 3 {
            hugatp = pt_pfn | HUGATP_MODE_SV39;
        } else if S2PT_MODE == 4 {
            hugatp = pt_pfn | HUGATP_MODE_SV48;
        } else {
            panic!("Invalid S2PT_MODE");
        }

        vcpu_ctx.set_host_csr(HUGATP, hugatp);

        unsafe {
            csrw!(HUGATP, hugatp);
        }

        dbgprintln!("set hugatp {:x}", hugatp);

        hugatp
    }

    fn handle_virtual_inst_fault(&self, vcpu_ctx: &mut VcpuCtx) -> i32 {
        let ret = 0;

        vcpu_ctx.increment_host_uepc(4);

        thread::yield_now();

        ret
    }

    fn handle_u_vtimer_irq(&self) -> i32 {
        /* Set virtual timer */
        self.virq.set_pending_irq(IRQ_VS_TIMER);
        unsafe {
            /*
             * FIXME: There may be unexpected pending bit IRQ_U_TIMER when
             * traped to kernel disable timer.
             */
            #[cfg(feature = "xilinx")]
            {
                wrvtimectl(0);
                csrc!(HUIP, 1 << IRQ_U_TIMER);
            }

            #[cfg(feature = "qemu")]
            {
                csrc!(VTIMECTL, 1 << VTIMECTL_ENABLE);
                csrc!(HUIP, 1 << IRQ_U_TIMER);
            }
        }

        return 0;
    }

    fn get_vm_inst_by_uepc(&self, read_insn: bool, vcpu_ctx: &mut VcpuCtx) -> u32 {
        let uepc = vcpu_ctx.get_host_csr(UEPC);
        let val: u32;

        /* FIXME: why KVM swap HSTATUS & STVEC here? */

        if read_insn {
            unsafe {
                asm!(
                    ".option push",
                    ".option norvc",

                    /* HULVX.HU t0, (t2) */
                    ".word 0x6433c2f3",
                    "andi t1, t0, 3",
                    "addi t1, t1, -3",
                    "bne t1, zero, 2f",
                    "addi t2, t2, 2",

                    /* HULVX.HU t1, (t2) */
                    ".word 0x6433c373",
                    "sll t1, t1, 16",
                    "add t0, t0, t1",
                    "2:",
                    ".option pop",
                    out("t0") val,
                    in("t2") uepc,
                );
            }
            dbgprintln!("HLVX.HU val: {:x}, uepc: {:x}", val, uepc);
        } else {
            /* TODO: HLV.D for IPI ECALL emulation */
            val = 0;
        }
        return val;
    }

    fn parse_load_inst(
        &self,
        inst: u32,
        inst_len: &mut u64,
        bit_width: &mut u64,
        target_reg: &mut u64,
    ) {
        /* 16BIT_MASK = 0x3 */
        *inst_len = if inst & 0x3 != 0x3 { 2 } else { 4 };
        if *inst_len == 2 {
            /* Compressed instruction */
            let c_lw_mask = 0b11 | (0b111 << 13);
            let c_lw_match = 0b00 | (0b010 << 13);
            let c_lw_rd = |inst: u32| -> u32 { ((inst >> 2) & 0x7) + 8 };

            if (inst & c_lw_mask) == c_lw_match {
                *target_reg = c_lw_rd(inst) as u64;
                *bit_width = 4 * 8;
                dbgprintln!(
                    "--- LW: inst {:x}, inst_len {:x}, reg: {}",
                    inst,
                    inst_len,
                    target_reg
                );
            } else {
                panic!(
                    "parse_load_inst: unsupported inst {:x}, inst_len {:x}",
                    inst, inst_len
                );
            }
        } else {
            /* TODO: refactor get_*_reg */
            let i_rd_reg = |inst: u32| -> u32 { (inst >> 7) & 0x1f };
            *target_reg = i_rd_reg(inst) as u64;

            if (inst & INST_MASK_LW) == INST_MATCH_LW {
                *bit_width = 4 * 8;
            } else if (inst & INST_MASK_LB) == INST_MATCH_LB {
                *bit_width = 1 * 8;
            } else {
                panic!(
                    "parse_load_inst: unsupported inst {:x}, inst_len {:x}",
                    inst, inst_len
                );
            }
        }
    }

    fn parse_store_inst(
        &self,
        inst: u32,
        inst_len: &mut u64,
        bit_width: &mut u64,
        target_reg: &mut u64,
    ) {
        /* 16BIT_MASK = 0x3 */
        *inst_len = if inst & 0x3 != 0x3 { 2 } else { 4 };
        if *inst_len == 2 {
            /* Compressed instruction */
            let c_sw_mask = 0b11 | (0b111 << 13);
            let c_sw_match = 0b00 | (0b110 << 13);
            let c_sw_rs2 = |inst: u32| -> u32 { ((inst >> 2) & 0x7) + 8 };

            if (inst & c_sw_mask) == c_sw_match {
                *target_reg = c_sw_rs2(inst) as u64;
                *bit_width = 4 * 8;
                dbgprintln!(
                    "--- SW: inst {:x}, inst_len {:x}, reg: {}",
                    inst,
                    inst_len,
                    target_reg
                );
            } else {
                panic!(
                    "parse_store_inst: unsupported inst {:x}, inst_len {:x}",
                    inst, inst_len
                );
            }
        } else {
            let s_rs2_reg = |inst: u32| -> u32 { (inst >> 20) & 0x1f };
            *target_reg = s_rs2_reg(inst) as u64;

            if (inst & INST_MASK_SW) == INST_MATCH_SW {
                *bit_width = 4 * 8;
            } else if (inst & INST_MASK_SB) == INST_MATCH_SB {
                *bit_width = 1 * 8;
            } else {
                panic!(
                    "parse_store_inst: unsupported inst {:x}, inst_len {:x}",
                    inst, inst_len
                );
            }
        }
    }

    fn store_emulation(
        &self,
        fault_addr: u64,
        target_reg: u64,
        bit_width: u64,
        vcpu_ctx: &mut VcpuCtx,
    ) -> i32 {
        let mut ret: i32 = 0;
        let bit_mask: u64 = (1 << bit_width) - 1;
        let mut data: u32 = (vcpu_ctx.get_guest_gpreg(target_reg as usize) & bit_mask) as u32;

        /* TODO: replce with MMIO bus */
        let is_irqchip_mmio = if 0xc000000 <= fault_addr && fault_addr < (0xc000000 + 0x1000000) {
            true
        } else {
            false
        };

        if is_irqchip_mmio {
            self.irqchip
                .get()
                .unwrap()
                .mmio_callback(fault_addr, &mut data, true);
        } else {
            let slice = &mut data.to_le_bytes();
            if self.mmio_bus.read().unwrap().write(fault_addr, slice) {
                ret = 0;
            } else {
                ret = 1;
                panic!(
                    "Unknown mmio (store) fault_addr: {:x}, ret {}",
                    fault_addr, ret
                );
            }
        }

        return ret;
    }

    fn load_emulation(
        &self,
        fault_addr: u64,
        target_reg: u64,
        bit_width: u64,
        vcpu_ctx: &mut VcpuCtx,
    ) -> i32 {
        let mut ret: i32 = 0;
        let bit_mask: u64 = (1 << bit_width) - 1;
        let mut data: u32 = 0;

        let is_irqchip_mmio = if 0xc000000 <= fault_addr && fault_addr < (0xc000000 + 0x1000000) {
            true
        } else {
            false
        };

        if is_irqchip_mmio {
            self.irqchip
                .get()
                .unwrap()
                .mmio_callback(fault_addr, &mut data, false);
        } else {
            let slice = &mut data.to_le_bytes();
            if self.mmio_bus.read().unwrap().read(fault_addr, slice) {
                data = u32::from_le_bytes(*slice);
                ret = 0;
            } else {
                ret = 1;
                panic!(
                    "Unknown mmio (load) fault_addr: {:x}, ret {}",
                    fault_addr, ret
                );
            }
        }
        vcpu_ctx.set_guest_gpreg(target_reg as usize, (data as u64) & bit_mask);

        return ret;
    }

    /*
     * Handlers for mmio require the follow info at least:
     * - fault address: the fault address
     * - instruction: the instruction which caused the trap
     *   - data bit width: for example, SD/LD or SW/LW
     *   - target register: the register which the data should be stored or
     *     loaded
     * - data access type: load or store (get from ucause or inst)
     *
     * TODO: the HLV instructions got some problems on qemu for now.
     * Take the load inst as 'lb a0, 0x0(a0)'
     * and the store inst as 'sb a2, 0x0(a1)'
     */
    fn handle_mmio(&self, fault_addr: u64, vcpu_ctx: &mut VcpuCtx) -> i32 {
        let ucause = vcpu_ctx.get_host_csr(UCAUSE);
        let hutinst = vcpu_ctx.get_host_csr(HUTINST);
        let inst: u32;
        let mut target_reg: u64 = 0xffff;
        let mut bit_width: u64 = 0;
        let mut inst_len: u64 = 0;
        let ret: i32;

        if hutinst == 0x0 {
            /* The implementation has not support the function of hutinst */
            inst = self.get_vm_inst_by_uepc(true, vcpu_ctx);
        } else {
            inst = hutinst as u32;
        }

        if ucause == EXC_LOAD_GUEST_PAGE_FAULT {
            self.parse_load_inst(inst, &mut inst_len, &mut bit_width, &mut target_reg);
        } else {
            self.parse_store_inst(inst, &mut inst_len, &mut bit_width, &mut target_reg);
        }

        if ucause == EXC_LOAD_GUEST_PAGE_FAULT {
            /* Load */
            ret = self.load_emulation(fault_addr, target_reg, bit_width, vcpu_ctx);
        } else if ucause == EXC_STORE_GUEST_PAGE_FAULT {
            /* Store */
            ret = self.store_emulation(fault_addr, target_reg, bit_width, vcpu_ctx);
        } else {
            ret = 1;
        }

        vcpu_ctx.increment_host_uepc(inst_len);

        return ret;
    }

    fn handle_stage2_page_fault(&self, vcpu_ctx: &mut VcpuCtx) -> i32 {
        let hutval = vcpu_ctx.get_host_csr(HUTVAL);
        let utval = vcpu_ctx.get_host_csr(UTVAL);
        let mut fault_addr = (hutval << 2) | (utval & 0x3);
        let mut ret;
        let mut gsmmu = self.vm.get_gsmmu().lock().unwrap();

        dbgprintln!(
            "gstage fault: hutval: {:x}, utval: {:x}, fault_addr: {:x}",
            hutval,
            utval,
            fault_addr
        );

        // if (fault_addr >= PLIC_HPA + 0x1f00000) && (fault_addr < PLIC_HPA + 0x1f00000 + 8) {
        if (fault_addr >= PLIC_HPA) && (fault_addr < PLIC_HPA + PLIC_LENGTH) {
            let addr = fault_addr - (fault_addr % 0x1000);
            gsmmu.map_page(addr, addr, PTE_VRWEU);

            // let mut addr = PLIC_HPA;
            // while addr < PLIC_HPA + PLIC_LENGTH {
            //     gsmmu.map_page(addr, addr, PTE_VRWEU);
            //     addr += 0x1000;
            // }
            return 0;
        }

        let gpa_check = gsmmu.check_gpa(fault_addr);
        if !gpa_check {
            /* Maybe mmio or illegal gpa */
            let mmio_check = gsmmu.check_mmio(fault_addr);

            if !mmio_check {
                panic!("Invalid gpa! {:x}", fault_addr);
            }

            ret = self.handle_mmio(fault_addr, vcpu_ctx);

            return ret;
        }

        fault_addr &= !PAGE_SIZE_MASK;

        /* Map query */
        let query = gsmmu.map_query(fault_addr);
        if query.is_none() {
            ret = ENOMAPPING;
        } else {
            let i = query.unwrap();

            if i.is_leaf() {
                let ucause = vcpu_ctx.get_host_csr(UCAUSE);

                /* No permission */
                if ucause == EXC_LOAD_GUEST_PAGE_FAULT && (i.get_value() & PTE_READ) == 0 {
                    ret = ENOPERMIT;
                } else if ucause == EXC_STORE_GUEST_PAGE_FAULT && (i.get_value() & PTE_WRITE) == 0 {
                    ret = ENOPERMIT;
                } else if ucause == EXC_INST_GUEST_PAGE_FAULT && (i.get_value() & PTE_EXECUTE) == 0
                {
                    ret = ENOPERMIT;
                } else {
                    /* S2PT contention with other vcpus */
                    return 0;
                }
            } else {
                dbgprintln!("QUERY is some but ENOMAPPING");

                ret = ENOMAPPING;
            }
        }

        match ret {
            ENOMAPPING => {
                dbgprintln!("Query return ENOMAPPING: {}", ret);
                /* Find hpa by fault_addr */
                let fault_addr_query = gsmmu.gpa_block_query(fault_addr);

                if fault_addr_query.is_none() {
                    /* Fault gpa is not in a gpa_block and it is valid */
                    let len = PAGE_SIZE;
                    let res = gsmmu.gpa_block_add(fault_addr, len);

                    if res.is_ok() {
                        /* Map new page to VM if the region exists */
                        let (_hva, hpa) = res.unwrap();
                        let flag: u64 = PTE_VRWEU;

                        #[cfg(feature = "qemu")]
                        gsmmu.map_page(fault_addr, hpa, flag);

                        #[cfg(feature = "xilinx")]
                        gsmmu.map_page(fault_addr, hpa, flag | PTE_ACCESS | PTE_DIRTY);

                        ret = 0;
                    } else {
                        panic!("Create gpa_block for fault addr {:x} failed!", fault_addr);
                    }
                } else {
                    /* Fault gpa is already in a gpa_block and it is valid */
                    let (_fault_hva, fault_hpa) = fault_addr_query.unwrap();
                    let flag: u64 = PTE_VRWEU;

                    dbgprintln!("map gpa: {:x} to hpa: {:x}", fault_addr, fault_hpa);

                    #[cfg(feature = "qemu")]
                    gsmmu.map_page(fault_addr, fault_hpa, flag);

                    #[cfg(feature = "xilinx")]
                    gsmmu.map_page(fault_addr, fault_hpa, flag | PTE_ACCESS | PTE_DIRTY);

                    ret = 0;
                }
            }
            ENOPERMIT => {
                self.exit_reason
                    .store(ExitReason::ExitEaccess, Ordering::SeqCst);
                dbgprintln!("Query return ENOPERMIT: {}", ret);
            }
            _ => {
                self.exit_reason
                    .store(ExitReason::ExitEaccess, Ordering::SeqCst);
                dbgprintln!("Invalid query result: {}", ret);
            }
        }

        ret
    }

    fn handle_supervisor_ecall(&self, vcpu_ctx: &mut VcpuCtx) -> i32 {
        let ret: i32;
        let a0 = vcpu_ctx.get_guest_gpreg(A0); /* A0: 0th arg/ret 1 */
        let a1 = vcpu_ctx.get_guest_gpreg(A1); /* A1: 1st arg/ret 2 */
        let a2 = vcpu_ctx.get_guest_gpreg(A2); /* A2: 2nd arg  */
        let a3 = vcpu_ctx.get_guest_gpreg(A3); /* A3: 3rd arg */
        let a4 = vcpu_ctx.get_guest_gpreg(A4); /* A4: 4th arg  */
        let a5 = vcpu_ctx.get_guest_gpreg(A5); /* A5: 5th arg  */
        let a6 = vcpu_ctx.get_guest_gpreg(A6); /* A6: FID */
        let a7 = vcpu_ctx.get_guest_gpreg(A7); /* A7: EID */

        /* FIXME: for test cases */
        if a7 == ECALL_VM_TEST_END {
            ret = 0xdead;

            #[cfg(feature = "xilinx")]
            println!(
                "ECALL_VM_TEST_END vcpu: {}, vipi_id {}",
                self.vcpu_id,
                rdvcpuid()
            );

            #[cfg(feature = "qemu")]
            println!(
                "ECALL_VM_TEST_END vcpu: {}, vipi_id {}",
                self.vcpu_id,
                unsafe { csrr!(VCPUID) }
            );

            vcpu_ctx.set_host_gpreg(0, ret as u64);

            return ret as i32;
        }

        let mut target_ecall = opensbi::emulation::Ecall::new();
        target_ecall.ext_id = a7;
        target_ecall.func_id = a6;
        target_ecall.arg[0] = a0;
        target_ecall.arg[1] = a1;
        target_ecall.arg[2] = a2;
        target_ecall.arg[3] = a3;
        target_ecall.arg[4] = a4;
        target_ecall.arg[5] = a5;
        target_ecall.ret[0] = a0;
        target_ecall.ret[1] = a1;

        /* Part of SBIs should emulated via IOCTL */
        let fd = self.vm.get_ioctl_fd();
        ret = target_ecall.ecall_handler(fd, &self);

        /* Save the result */
        vcpu_ctx.set_guest_gpreg(10, target_ecall.ret[0]);
        vcpu_ctx.set_guest_gpreg(11, target_ecall.ret[1]);

        /* Add uepc to start vm on next instruction */
        vcpu_ctx.increment_host_uepc(4);

        ret
    }

    fn handle_u_vipi_irq(&self) -> i32 {
        let vcpu_id = self.vcpu_id;
        let vipi_id = self.vipi.get_vcpu_id_map(vcpu_id).load(Ordering::SeqCst);

        unsafe {
            VirtualIpi::clear_vipi(vipi_id);
            csrc!(HUIP, 1 << IRQ_U_SOFT);

            #[cfg(feature = "xilinx")]
            dbgprintln!("vcpu {}, vipi id {}", vcpu_id, rdvcpuid());

            #[cfg(feature = "qemu")]
            dbgprintln!("vcpu {}, vipi id {}", vcpu_id, csrr!(VCPUID));
        }

        #[cfg(test)]
        unsafe {
            *GET_UIPI_CNT.lock().unwrap() += 1;
        }

        return 0;
    }

    fn handle_vcpu_exit(&self, vcpu_ctx: &mut VcpuCtx) -> i32 {
        let mut ret: i32 = -1;
        let ucause = vcpu_ctx.get_host_csr(UCAUSE);

        if (ucause & EXC_IRQ_MASK) != 0 {
            self.exit_reason
                .store(ExitReason::ExitIntr, Ordering::SeqCst);
            let ucause = ucause & (!EXC_IRQ_MASK);
            match ucause {
                IRQ_U_TIMER => {
                    dbgprintln!(
                        "handler U TIMER: {}, current pc is {:x}.",
                        ucause,
                        vcpu_ctx.get_host_csr(UEPC)
                    );
                    ret = self.handle_u_vtimer_irq();
                }
                IRQ_U_SOFT => {
                    dbgprintln!("handler U VIPI, vcpu_id: {}", self.vcpu_id);

                    ret = self.handle_u_vipi_irq();
                }
                _ => {
                    dbgprintln!("Invalid IRQ ucause: {}", ucause);
                    ret = 1;
                }
            }
            return ret;
        }

        self.exit_reason
            .store(ExitReason::ExitUnknown, Ordering::SeqCst);

        match ucause {
            EXC_VIRTUAL_INST_FAULT => {
                self.handle_virtual_inst_fault(vcpu_ctx);
                ret = 0;
            }
            EXC_INST_GUEST_PAGE_FAULT | EXC_LOAD_GUEST_PAGE_FAULT | EXC_STORE_GUEST_PAGE_FAULT => {
                ret = self.handle_stage2_page_fault(vcpu_ctx);
            }
            EXC_VIRTUAL_SUPERVISOR_SYSCALL => {
                ret = self.handle_supervisor_ecall(vcpu_ctx);
            }
            _ => {
                dbgprintln!("Invalid EXCP ucause: {}", ucause);
            }
        }

        if ret < 0 {
            dbgprintln!("ERROR: handle_vcpu_exit ret: {}", ret);

            /* FIXME: save the exit reason in HOST_A0 before the vcpu down */
            vcpu_ctx.set_host_gpreg(0, (0 - ret) as u64);
        }

        ret
    }

    fn config_hustatus(&self, vcpu_ctx: &mut VcpuCtx) {
        vcpu_ctx.set_host_csr(
            HUSTATUS,
            ((1 << HUSTATUS_SPV_SHIFT) | (1 << HUSTATUS_SPVP_SHIFT))
                | (1 << HUSTATUS_VTW_SHIFT)
                | (1 << HUSTATUS_UPIE_SHIFT) as u64,
        );
    }

    fn config_huie(&self) {
        unsafe {
            csrw!(HUIE, (1 << IRQ_U_TIMER) | (1 << IRQ_U_SOFT));
        }
    }

    pub fn thread_vcpu_run(&self, delta_time: i64) -> i32 {
        let fd = self.vm.get_ioctl_fd();
        let mut _res;
        let mut vcpu_ctx = self.vcpu_ctx.lock().unwrap();

        self.config_hustatus(&mut *vcpu_ctx);

        let vmid: u64 = self.vm.get_vmid();
        let vipi_id: u64 = vmid * (MAX_VCPU as u64) + self.vcpu_id as u64 + 1;

        unsafe {
            /* Register vcpu thread to the kernel */
            _res = libc::ioctl(fd, IOCTL_DUVISOR_REGISTER_VCPU);
            dbgprintln!("IOCTL_DUVISOR_REGISTER_VCPU : {}", _res);
            self.vipi.vcpu_regist(self.vcpu_id, vipi_id);

            /* Set hugatp */
            let _hugatp = self.config_hugatp(&mut *vcpu_ctx);
            dbgprintln!("Config hugatp: {:x}", _hugatp);

            /* Set trap handler */
            csrw!(UTVEC, exit_guest as u64);

            /* Enable timer irq */
            self.config_huie();

            /* TODO: redesign scounteren register */
            /* Allow VM to directly access time register */

            /* TODO: introduce RUST feature to distinguish between rv64 and rv32 */
            csrw!(HUTIMEDELTA, -delta_time as u64);
        }
        /* FIXME: deadlock if ptr & ptr_u64 are not declared independently */
        let vcpu_ctx_ptr: *const VcpuCtx;
        let vcpu_ctx_ptr_u64: u64;
        vcpu_ctx_ptr = &*vcpu_ctx as *const VcpuCtx;
        vcpu_ctx_ptr_u64 = vcpu_ctx_ptr as u64;

        let mut ret: i32 = 0;
        while ret == 0 {
            /* Flush pending irqs into HUVIP */
            self.virq.flush_pending_irq();

            self.is_running.store(true, Ordering::SeqCst);

            /* Flag `SHUTDOWN_FLAG` would be set to 1 when a shutdown ecall triggers */
            unsafe {
                if SHUTDOWN_FLAG == 1 {
                    ret = -100;
                    break;
                }
            }

            unsafe {
                enter_guest(vcpu_ctx_ptr_u64);
            }
            self.is_running.store(false, Ordering::SeqCst);

            /* FIXME: why KVM need sync_pending_irq() here? */
            ret = self.handle_vcpu_exit(&mut *vcpu_ctx);
        }

        unsafe {
            _res = libc::ioctl(fd, IOCTL_DUVISOR_UNREGISTER_VCPU);
            dbgprintln!("IOCTL_DUVISOR_UNREGISTER_VCPU : {}", _res);
        }

        ret
    }

    pub fn vcpu_id(&self) -> u32 {
        self.vcpu_id
    }

    pub fn vcpu_num(&self) -> u32 {
        self.vipi.vcpu_num()
    }

    pub fn get_vcpu_id_map(&self, i: u32) -> &AtomicU64 {
        &self.vipi.get_vcpu_id_map(i)
    }

    pub fn set_irqchip(&self, irqchip: Arc<Plic>) -> Result<(), Arc<dyn IrqChip>> {
        self.irqchip.set(irqchip)
    }

    pub fn get_irqchip(&self) -> Option<&Arc<dyn IrqChip>> {
        self.irqchip.get()
    }

    pub fn unset_pending_irq(&self, irq: u64) {
        self.virq.unset_pending_irq(irq)
    }

    pub fn set_pending_irq(&self, irq: u64) {
        self.virq.set_pending_irq(irq)
    }

    pub fn is_running(&self, order: Ordering) -> bool {
        self.is_running.load(order)
    }

    pub fn set_guest_gpreg(&self, regid: usize, value: u64) {
        self.vcpu_ctx.lock().unwrap().set_guest_gpreg(regid, value)
    }

    pub fn get_guest_gpreg(&self, regid: usize) -> u64 {
        self.vcpu_ctx.lock().unwrap().get_guest_gpreg(regid)
    }

    pub fn set_host_gpreg(&self, regid: usize, value: u64) {
        self.vcpu_ctx.lock().unwrap().set_host_gpreg(regid, value)
    }

    pub fn get_host_gpreg(&self, regid: usize) -> u64 {
        self.vcpu_ctx.lock().unwrap().get_host_gpreg(regid)
    }

    pub fn set_guest_csr(&self, regid: u64, value: u64) {
        self.vcpu_ctx.lock().unwrap().set_guest_csr(regid, value)
    }

    pub fn get_guest_csr(&self, regid: u64) -> u64 {
        self.vcpu_ctx.lock().unwrap().get_guest_csr(regid)
    }

    pub fn set_host_csr(&self, regid: u64, value: u64) {
        self.vcpu_ctx.lock().unwrap().set_host_csr(regid, value)
    }

    pub fn get_host_csr(&self, regid: u64) -> u64 {
        self.vcpu_ctx.lock().unwrap().get_host_csr(regid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::utils::configtest::test_vm_config_create;
    use rusty_fork::rusty_fork_test;

    rusty_fork_test! {
        /* Check the correctness of vcpu new() */
        #[test]
        fn test_vcpu_new() {
            let vcpu_id = 20;
            let vm_config = test_vm_config_create();
            let vcpu_num = vm_config.get_vcpu_count();
            let vm = virtualmachine::VirtualMachine::new(vm_config);
            let vm_mutex = vm.get_vm_state();
            let mmio_bus = Arc::new(RwLock::new(devices::Bus::new()));
            let guest_mem = GuestMemory::new().unwrap();
            let vipi = VirtualIpi::new(vcpu_num);
            let vipi_ptr = Arc::new(vipi);
            let vcpu = VirtualCpu::new(vcpu_id, vm_mutex, guest_mem, mmio_bus, vipi_ptr);

            assert_eq!(vcpu.vcpu_id, vcpu_id);
        }

        /* Check the init state of the vcpu */
        #[test]
        fn test_vcpu_ctx_init() {
            let vcpu_id = 1;
            let vm_config = test_vm_config_create();
            let vcpu_num = vm_config.get_vcpu_count();
            let vm = virtualmachine::VirtualMachine::new(vm_config);
            let vm_mutex = vm.get_vm_state();
            let mmio_bus = Arc::new(RwLock::new(devices::Bus::new()));
            let guest_mem = GuestMemory::new().unwrap();
            let vipi = VirtualIpi::new(vcpu_num);
            let vipi_ptr = Arc::new(vipi);
            let vcpu = VirtualCpu::new(vcpu_id, vm_mutex, guest_mem, mmio_bus, vipi_ptr);

            let tmp = vcpu.vcpu_ctx.lock().unwrap().host_ctx.gp_regs.x_reg[10];
            assert_eq!(tmp, 0);

            let tmp = vcpu.vcpu_ctx.lock().unwrap().host_ctx.hyp_regs.hutinst;
            assert_eq!(tmp, 0);

            let tmp = vcpu.vcpu_ctx.lock().unwrap().guest_ctx.gp_regs.x_reg[10];
            assert_eq!(tmp, 0);

            let tmp = vcpu.vcpu_ctx.lock().unwrap().guest_ctx.hyp_regs.hutinst;
            assert_eq!(tmp, 0);

            let tmp = vcpu.vcpu_ctx.lock().unwrap().guest_ctx.sys_regs.huvsatp;
            assert_eq!(tmp, 0);
        }

        /* Check the rw permission of vcpu ctx */
        #[test]
        fn test_vcpu_set_ctx() {
            let vcpu_id = 1;
            let vm_config = test_vm_config_create();
            let vcpu_num = vm_config.get_vcpu_count();
            let vm = virtualmachine::VirtualMachine::new(vm_config);
            let vm_mutex = vm.get_vm_state();
            let mmio_bus = Arc::new(RwLock::new(devices::Bus::new()));
            let guest_mem = GuestMemory::new().unwrap();
            let vipi = VirtualIpi::new(vcpu_num);
            let vipi_ptr = Arc::new(vipi);
            let vcpu = VirtualCpu::new(vcpu_id, vm_mutex, guest_mem, mmio_bus, vipi_ptr);
            let ans = 17;

            /* Guest ctx */
            vcpu.vcpu_ctx.lock().unwrap().guest_ctx.gp_regs.x_reg[10] = ans;
            let tmp = vcpu.vcpu_ctx.lock().unwrap().guest_ctx.gp_regs.x_reg[10];
            assert_eq!(tmp, ans);

            vcpu.vcpu_ctx.lock().unwrap().guest_ctx.sys_regs.huvsatp = ans;
            let tmp = vcpu.vcpu_ctx.lock().unwrap().guest_ctx.sys_regs.huvsatp;
            assert_eq!(tmp, ans);

            vcpu.vcpu_ctx.lock().unwrap().guest_ctx.hyp_regs.hutinst = ans;
            let tmp = vcpu.vcpu_ctx.lock().unwrap().guest_ctx.hyp_regs.hutinst;
            assert_eq!(tmp, ans);

            /* Host ctx */
            vcpu.vcpu_ctx.lock().unwrap().host_ctx.gp_regs.x_reg[10] = ans;
            let tmp = vcpu.vcpu_ctx.lock().unwrap().host_ctx.gp_regs.x_reg[10];
            assert_eq!(tmp, ans);

            vcpu.vcpu_ctx.lock().unwrap().host_ctx.hyp_regs.hutinst = ans;
            let tmp = vcpu.vcpu_ctx.lock().unwrap().host_ctx.hyp_regs.hutinst;
            assert_eq!(tmp, ans);
        }

        #[test]
        fn test_vcpu_ecall_exit() {
            let vcpu_id = 0;
            let vm_config = test_vm_config_create();
            let vcpu_num = vm_config.get_vcpu_count();
            let vm = virtualmachine::VirtualMachine::new(vm_config);
            let fd = vm.get_ioctl_fd();
            let vm_mutex = vm.get_vm_state();
            let mmio_bus = Arc::new(RwLock::new(devices::Bus::new()));
            let guest_mem = GuestMemory::new().unwrap();
            let vipi = VirtualIpi::new(vcpu_num);
            let vipi_ptr = Arc::new(vipi);
            let vcpu = VirtualCpu::new(vcpu_id, vm_mutex, guest_mem, mmio_bus, vipi_ptr);
            let res;
            let version: u64 = 0;
            let test_buf: u64;
            let mut test_buf_pfn: u64;
            let test_buf_size: usize = 64 << 20;
            let mut hugatp: u64;

            println!("---test_vcpu_ecall_exit---");

            unsafe {
                /* Ioctl */
                let version_ptr = (&version) as *const u64;
                libc::ioctl(fd, IOCTL_DUVISOR_GET_API_VERSION, version_ptr);
                println!("IOCTL_DUVISOR_GET_API_VERSION -  version : {:x}",
                    version);

                let addr = 0 as *mut libc::c_void;
                let mmap_ptr = libc::mmap(addr, test_buf_size,
                    libc::PROT_READ | libc::PROT_WRITE,
                    libc::MAP_SHARED, fd, 0);
                assert_ne!(mmap_ptr, libc::MAP_FAILED);

                test_buf = mmap_ptr as u64; /* VA */
                test_buf_pfn = test_buf; /* PA.PFN */
                let test_buf_pfn_ptr = (&mut test_buf_pfn) as *mut u64;
                libc::ioctl(fd, IOCTL_DUVISOR_QUERY_PFN, test_buf_pfn_ptr);
                println!("IOCTL_DUVISOR_QUERY_PFN -  test_buf_pfn : {:x}",
                    test_buf_pfn);

                /* Set test code */
                let start = vcpu_ecall_exit as u64;
                let end = vcpu_ecall_exit_end as u64;
                let code_buf = test_buf + PAGE_TABLE_REGION_SIZE;

                std::ptr::copy_nonoverlapping(vcpu_ecall_exit as *const u8,
                    code_buf as *mut u8, (end - start) as usize);

                /* Set hugatp */
                hugatp = test_buf;
                let pte_ptr = (hugatp + 8 * (((test_buf_pfn << PAGE_SIZE_SHIFT)
                     + PAGE_TABLE_REGION_SIZE) >> 30)) as *mut u64;

                let pte_ptr_value = pte_ptr as u64;
                println!("pte_ptr_value {}", pte_ptr_value);

                /* 512G 1-level direct mapping */
                *pte_ptr = (((test_buf_pfn << PAGE_SIZE_SHIFT) >> 30) << 28)
                    | 0x1f;
                println!("PTE : {:x}", *pte_ptr);

                /* Delegate vs-ecall and guest page fault */
                virtualmachine::VirtualMachine::hu_delegation(fd);

                res = libc::ioctl(fd, IOCTL_DUVISOR_REGISTER_VCPU);
                println!("IOCTL_DUVISOR_REGISTER_VCPU : {}", res);
            }

            let uepc: u64;
            let utval: u64;
            let ucause: u64;

            /* FIXME: deadlock if ptr & ptr_u64 are not declared independently */
            let ptr: *const VcpuCtx;
            let ptr_u64: u64;
            ptr = &*vcpu.vcpu_ctx.lock().unwrap() as *const VcpuCtx;
            ptr_u64 = ptr as u64;
            println!("the ptr is {:x}", ptr_u64);

            let target_code = ((test_buf_pfn << PAGE_SHIFT)
                + PAGE_TABLE_REGION_SIZE) as u64;
            vcpu.vcpu_ctx.lock().unwrap().host_ctx.hyp_regs.uepc = target_code;


            hugatp = test_buf_pfn | (8 << 60);
            vcpu.vcpu_ctx.lock().unwrap().host_ctx.hyp_regs.hugatp = hugatp;

            unsafe {
                csrw!(HUGATP, hugatp);
                /* Set hugatp */
                println!("HUGATP : 0x{:x}", hugatp);
                /* HUSTATUS.SPP=1 .SPVP=1 uret to VS mode */
                vcpu.vcpu_ctx.lock().unwrap().host_ctx.hyp_regs.hustatus =
                    ((1 << HUSTATUS_SPV_SHIFT)
                    | (1 << HUSTATUS_SPVP_SHIFT)) as u64;

                /* Set utvec to trap handler */
                csrw!(UTVEC, exit_guest as u64);
                enter_guest(ptr_u64);

                uepc = vcpu.vcpu_ctx.lock().unwrap().host_ctx.hyp_regs.uepc;
                utval = vcpu.vcpu_ctx.lock().unwrap().host_ctx.hyp_regs.utval;
                ucause = vcpu.vcpu_ctx.lock().unwrap().host_ctx.hyp_regs.ucause;

                let a7 = vcpu.vcpu_ctx.lock().unwrap().guest_ctx.gp_regs.x_reg[17];

                println!("guest hyp uepc 0x{:x}", uepc);
                println!("guest hyp utval 0x{:x}", utval);
                println!("guest hyp ucause 0x{:x}", ucause);
                println!("guest hyp a7 0x{:x}", a7);
            }

            assert_eq!(uepc, ((test_buf_pfn << PAGE_SIZE_SHIFT)
                + PAGE_TABLE_REGION_SIZE) + 4);
            assert_eq!(utval, 0);
            assert_eq!(ucause, 10);
        }

        #[test]
        fn test_vcpu_add_all_gprs() {
            let vcpu_id = 0;
            let vm_config = test_vm_config_create();
            let vcpu_num = vm_config.get_vcpu_count();
            let vm = virtualmachine::VirtualMachine::new(vm_config);
            let fd = vm.get_ioctl_fd();
            let vm_mutex = vm.get_vm_state();
            let mmio_bus = Arc::new(RwLock::new(devices::Bus::new()));
            let guest_mem = GuestMemory::new().unwrap();
            let vipi = VirtualIpi::new(vcpu_num);
            let vipi_ptr = Arc::new(vipi);
            let vcpu = VirtualCpu::new(vcpu_id, vm_mutex, guest_mem, mmio_bus, vipi_ptr);
            let res;
            let version: u64 = 0;
            let test_buf: u64;
            let mut test_buf_pfn: u64;
            let test_buf_size: usize = 64 << 20; /* 64 MB */
            let size: u64;
            let mut hugatp: u64;

            println!("---test_vcpu_add_all_gprs---");

            unsafe {
                /* Ioctl */
                let version_ptr = (&version) as *const u64;
                libc::ioctl(fd, IOCTL_DUVISOR_GET_API_VERSION, version_ptr);
                println!("IOCTL_DUVISOR_GET_API_VERSION -  version : {:x}",
                    version);

                let addr = 0 as *mut libc::c_void;
                let mmap_ptr = libc::mmap(addr, test_buf_size,
                    libc::PROT_READ | libc::PROT_WRITE,
                    libc::MAP_SHARED, fd, 0);
                assert_ne!(mmap_ptr, libc::MAP_FAILED);

                test_buf = mmap_ptr as u64; /* VA */
                test_buf_pfn = test_buf; /* PA.PFN */
                let test_buf_pfn_ptr = (&mut test_buf_pfn) as *mut u64;
                libc::ioctl(fd, IOCTL_DUVISOR_QUERY_PFN, test_buf_pfn_ptr);
                println!("IOCTL_DUVISOR_QUERY_PFN -  test_buf_pfn : {:x}",
                    test_buf_pfn);

                /* Set test code */
                let start = vcpu_add_all_gprs as u64;
                let end = vcpu_add_all_gprs_end as u64;
                size = end - start;
                let code_buf = test_buf + PAGE_TABLE_REGION_SIZE;

                std::ptr::copy_nonoverlapping(vcpu_add_all_gprs as *const u8,
                    code_buf as *mut u8, size as usize);

                /* Set hugatp */
                hugatp = test_buf;
                let pte_ptr = (hugatp + 8 * (((test_buf_pfn << PAGE_SIZE_SHIFT)
                    + PAGE_TABLE_REGION_SIZE) >> 30)) as *mut u64;

                let pte_ptr_value = pte_ptr as u64;
                println!("pte_ptr_value {}", pte_ptr_value);

                /* 512G 1-level direct mapping */
                *pte_ptr = (((test_buf_pfn << PAGE_SIZE_SHIFT) >> 30) << 28)
                    | 0x1f;
                println!("PTE : {:x}", *pte_ptr);

                /* Delegate vs-ecall and guest page fault */
                virtualmachine::VirtualMachine::hu_delegation(fd);

                res = libc::ioctl(fd, IOCTL_DUVISOR_REGISTER_VCPU);
                println!("IOCTL_DUVISOR_REGISTER_VCPU : {}", res);
            }

            let uepc: u64;
            let utval: u64;
            let ucause: u64;

            /* FIXME: deadlock if ptr & ptr_u64 are not declared independently */
            let ptr: *const VcpuCtx;
            let ptr_u64: u64;
            ptr = &*vcpu.vcpu_ctx.lock().unwrap() as *const VcpuCtx;
            ptr_u64 = ptr as u64;
            println!("the ptr is {:x}", ptr_u64);

            let target_code = ((test_buf_pfn << PAGE_SHIFT)
                + PAGE_TABLE_REGION_SIZE) as u64;
            vcpu.vcpu_ctx.lock().unwrap().host_ctx.hyp_regs.uepc = target_code;

            hugatp = test_buf_pfn | (8 << 60);
            vcpu.vcpu_ctx.lock().unwrap().host_ctx.hyp_regs.hugatp = hugatp;

            let mut sum = 0;
            let len = vcpu.vcpu_ctx.lock().unwrap().guest_ctx.gp_regs.x_reg.len();
            for i in 0..len {
                vcpu.vcpu_ctx.lock().unwrap().guest_ctx.gp_regs.x_reg[i] = i as u64;
                sum += i as u64;
            }

            sum += 10 - 1;
            println!("sum {}", sum);

            unsafe {
                csrw!(HUGATP, hugatp);
                /* Set hugatp */
                println!("HUGATP : 0x{:x}", hugatp);
                /* HUSTATUS.SPP=1 .SPVP=1 uret to VS mode */
                vcpu.vcpu_ctx.lock().unwrap().host_ctx.hyp_regs.hustatus =
                    ((1 << HUSTATUS_SPV_SHIFT)
                    | (1 << HUSTATUS_SPVP_SHIFT)) as u64;
                /* Set utvec to trap handler */
                csrw!(UTVEC, exit_guest as u64);
                enter_guest(ptr_u64);

                uepc = vcpu.vcpu_ctx.lock().unwrap().host_ctx.hyp_regs.uepc;
                utval = vcpu.vcpu_ctx.lock().unwrap().host_ctx.hyp_regs.utval;
                ucause = vcpu.vcpu_ctx.lock().unwrap().host_ctx.hyp_regs.ucause;

                let a7 = vcpu.vcpu_ctx.lock().unwrap().guest_ctx.gp_regs.x_reg[17];

                println!("guest hyp uepc 0x{:x}", uepc);
                println!("guest hyp utval 0x{:x}", utval);
                println!("guest hyp ucause 0x{:x}", ucause);
                println!("guest hyp a7 0x{:x}", a7);
            }

            assert_eq!(sum, vcpu.vcpu_ctx.lock().unwrap().guest_ctx.gp_regs.x_reg[10]);
            assert_eq!(uepc, ((test_buf_pfn << PAGE_SIZE_SHIFT)
                + PAGE_TABLE_REGION_SIZE) + size - 4);
            assert_eq!(utval, 0);
            assert_eq!(ucause, 10);
        }
    }
}

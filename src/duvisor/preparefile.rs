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

#[path = "src/vcpu/utils.rs"]
mod utils;
#[path = "src/vcpu/vcpucontext.rs"]
mod vcpucontext;

use std::io::Write;
use vcpucontext::*;

macro_rules! add_offset {
    ($list:expr, $name:expr, $ctx:expr, $reg:expr) => {
        $list.push(ContextOffset::new(
            String::from($name),
            field_offset(&$ctx, &$reg),
        ));
    };
}

fn field_offset<T1, T2>(ctx: &T1, reg: &T2) -> u64 {
    let ctx_ptr = (ctx as *const T1) as u64;
    let reg_ptr = (reg as *const T2) as u64;

    reg_ptr - ctx_ptr
}

struct ContextOffset {
    name: String,
    offset: u64,
}

impl ContextOffset {
    pub fn new(name: String, offset: u64) -> Self {
        Self { name, offset }
    }
}

fn create_gp_list() -> Vec<ContextOffset> {
    let gp = GpRegs::new();
    let mut gp_list: Vec<ContextOffset> = Vec::new();

    add_offset!(gp_list, "GP_X0", gp, gp.x_reg[0]);
    add_offset!(gp_list, "GP_X1", gp, gp.x_reg[1]);
    add_offset!(gp_list, "GP_X2", gp, gp.x_reg[2]);
    add_offset!(gp_list, "GP_X3", gp, gp.x_reg[3]);
    add_offset!(gp_list, "GP_X4", gp, gp.x_reg[4]);
    add_offset!(gp_list, "GP_X5", gp, gp.x_reg[5]);
    add_offset!(gp_list, "GP_X6", gp, gp.x_reg[6]);
    add_offset!(gp_list, "GP_X7", gp, gp.x_reg[7]);
    add_offset!(gp_list, "GP_X8", gp, gp.x_reg[8]);
    add_offset!(gp_list, "GP_X9", gp, gp.x_reg[9]);
    add_offset!(gp_list, "GP_X10", gp, gp.x_reg[10]);
    add_offset!(gp_list, "GP_X11", gp, gp.x_reg[11]);
    add_offset!(gp_list, "GP_X12", gp, gp.x_reg[12]);
    add_offset!(gp_list, "GP_X13", gp, gp.x_reg[13]);
    add_offset!(gp_list, "GP_X14", gp, gp.x_reg[14]);
    add_offset!(gp_list, "GP_X15", gp, gp.x_reg[15]);
    add_offset!(gp_list, "GP_X16", gp, gp.x_reg[16]);
    add_offset!(gp_list, "GP_X17", gp, gp.x_reg[17]);
    add_offset!(gp_list, "GP_X18", gp, gp.x_reg[18]);
    add_offset!(gp_list, "GP_X19", gp, gp.x_reg[19]);
    add_offset!(gp_list, "GP_X20", gp, gp.x_reg[20]);
    add_offset!(gp_list, "GP_X21", gp, gp.x_reg[21]);
    add_offset!(gp_list, "GP_X22", gp, gp.x_reg[22]);
    add_offset!(gp_list, "GP_X23", gp, gp.x_reg[23]);
    add_offset!(gp_list, "GP_X24", gp, gp.x_reg[24]);
    add_offset!(gp_list, "GP_X25", gp, gp.x_reg[25]);
    add_offset!(gp_list, "GP_X26", gp, gp.x_reg[26]);
    add_offset!(gp_list, "GP_X27", gp, gp.x_reg[27]);
    add_offset!(gp_list, "GP_X28", gp, gp.x_reg[28]);
    add_offset!(gp_list, "GP_X29", gp, gp.x_reg[29]);
    add_offset!(gp_list, "GP_X30", gp, gp.x_reg[30]);
    add_offset!(gp_list, "GP_X31", gp, gp.x_reg[31]);

    gp_list
}

fn create_sys_list() -> Vec<ContextOffset> {
    let sys = SysRegs::new();
    let mut sys_list: Vec<ContextOffset> = Vec::new();

    add_offset!(sys_list, "SYS_HUVSSTATUS", sys, sys.huvsstatus);
    add_offset!(sys_list, "SYS_HUVSIP", sys, sys.huvsip);
    add_offset!(sys_list, "SYS_HUVSIE", sys, sys.huvsie);
    add_offset!(sys_list, "SYS_HUVSTVEC", sys, sys.huvstvec);
    add_offset!(sys_list, "SYS_HUVSSCRATCH", sys, sys.huvsscratch);
    add_offset!(sys_list, "SYS_HUVSEPC", sys, sys.huvsepc);
    add_offset!(sys_list, "SYS_HUVSCAUSE", sys, sys.huvscause);
    add_offset!(sys_list, "SYS_HUVSTVAL", sys, sys.huvstval);
    add_offset!(sys_list, "SYS_HUVSATP", sys, sys.huvsatp);

    sys_list
}

fn create_hyp_list() -> Vec<ContextOffset> {
    let hyp = HypRegs::new();
    let mut hyp_list: Vec<ContextOffset> = Vec::new();

    add_offset!(hyp_list, "HYP_HUSTATUS", hyp, hyp.hustatus);
    add_offset!(hyp_list, "HYP_HUEDELEG", hyp, hyp.huedeleg);
    add_offset!(hyp_list, "HYP_HUIDELEG", hyp, hyp.huideleg);
    add_offset!(hyp_list, "HYP_HUVIP", hyp, hyp.huvip);
    add_offset!(hyp_list, "HYP_HUIP", hyp, hyp.huip);
    add_offset!(hyp_list, "HYP_HUIE", hyp, hyp.huie);
    add_offset!(hyp_list, "HYP_HUGEIP", hyp, hyp.hugeip);
    add_offset!(hyp_list, "HYP_HUGEIE", hyp, hyp.hugeie);
    add_offset!(hyp_list, "HYP_HUCOUNTEREN", hyp, hyp.hucounteren);
    add_offset!(hyp_list, "HYP_HUTIMEDELTA", hyp, hyp.hutimedelta);
    add_offset!(hyp_list, "HYP_HUTIMEDELTAH", hyp, hyp.hutimedeltah);
    add_offset!(hyp_list, "HYP_HUTVAL", hyp, hyp.hutval);
    add_offset!(hyp_list, "HYP_HUTINST", hyp, hyp.hutinst);
    add_offset!(hyp_list, "HYP_HUGATP", hyp, hyp.hugatp);
    add_offset!(hyp_list, "HYP_UTVEC", hyp, hyp.utvec);
    add_offset!(hyp_list, "HYP_UEPC", hyp, hyp.uepc);
    add_offset!(hyp_list, "HYP_USCRATCH", hyp, hyp.uscratch);
    add_offset!(hyp_list, "HYP_UTVAL", hyp, hyp.utval);
    add_offset!(hyp_list, "HYP_UCAUSE", hyp, hyp.ucause);

    hyp_list
}

/* VcpuCtx - VcpuCtx.##Ctx.##Regs.reg */
fn create_ctx_reg_offset(
    mut offset_list: Vec<ContextOffset>,
    gp_list: &Vec<ContextOffset>,
    sys_list: &Vec<ContextOffset>,
    hyp_list: &Vec<ContextOffset>,
) -> Vec<ContextOffset> {
    let vcpu = VcpuCtx::new();

    /* HOST_GP & GUEST_GP */
    for i in gp_list {
        let mut full_name = "HOST_".to_string();
        let reg_name = i.name.to_string();
        full_name += &reg_name;

        let mut offset = field_offset(&vcpu, &vcpu.host_ctx.gp_regs) + i.offset;
        offset_list.push(ContextOffset::new(full_name, offset));

        full_name = "GUEST_".to_string();
        full_name += &reg_name;

        offset = field_offset(&vcpu, &vcpu.guest_ctx.gp_regs) + i.offset;
        offset_list.push(ContextOffset::new(full_name, offset));
    }

    /* HOST_HYP & GUEST_HYP */
    for i in hyp_list {
        let mut full_name = "HOST_".to_string();
        let reg_name = i.name.to_string();
        full_name += &reg_name;

        let mut offset = field_offset(&vcpu, &vcpu.host_ctx.hyp_regs) + i.offset;
        offset_list.push(ContextOffset::new(full_name, offset));

        full_name = "GUEST_".to_string();
        full_name += &reg_name;

        offset = field_offset(&vcpu, &vcpu.guest_ctx.hyp_regs) + i.offset;
        offset_list.push(ContextOffset::new(full_name, offset));
    }

    /* GUEST_SYS */
    for i in sys_list {
        let mut full_name = "GUEST_".to_string();
        let reg_name = i.name.to_string();
        full_name += &reg_name;

        let offset = field_offset(&vcpu, &vcpu.guest_ctx.sys_regs) + i.offset;
        offset_list.push(ContextOffset::new(full_name, offset));
    }

    offset_list
}

/* VcpuCtx - VcpuCtx.####Ctx.GpRegs */
fn create_ctx_gp_offset(mut offset_list: Vec<ContextOffset>) -> Vec<ContextOffset> {
    let vcpu = VcpuCtx::new();

    add_offset!(offset_list, "HOST_GP", vcpu, vcpu.host_ctx.gp_regs);
    add_offset!(offset_list, "GUEST_GP", vcpu, vcpu.guest_ctx.gp_regs);

    offset_list
}

fn write_asm_offset_header(offset_list: Vec<ContextOffset>) {
    let mut asm_offset = std::fs::File::create("src/vcpu/asm_offset.h").expect("create failed");
    asm_offset
        .write_all(
            "/* This file is generated by build.rs. Please do not modify it! */\n\n".as_bytes(),
        )
        .expect("write failed");

    for i in offset_list {
        asm_offset
            .write_all("#define ".as_bytes())
            .expect("write failed");
        asm_offset
            .write_all(i.name.as_bytes())
            .expect("write failed");
        asm_offset.write_all(" ".as_bytes()).expect("write failed");
        asm_offset
            .write_all(i.offset.to_string().as_bytes())
            .expect("write failed");
        asm_offset.write_all("\n".as_bytes()).expect("write failed");
    }
}

pub fn prepare_asm_offset_header() {
    let mut offset_list: Vec<ContextOffset> = Vec::new();

    let mut gp_list = create_gp_list();
    let mut sys_list = create_sys_list();
    let mut hyp_list = create_hyp_list();

    offset_list = create_ctx_reg_offset(offset_list, &gp_list, &sys_list, &hyp_list);
    offset_list = create_ctx_gp_offset(offset_list);

    offset_list.append(&mut gp_list);
    offset_list.append(&mut sys_list);
    offset_list.append(&mut hyp_list);

    write_asm_offset_header(offset_list);
}

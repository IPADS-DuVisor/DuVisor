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

#[path = "preparefile.rs"]
mod preparefile;

use preparefile::*;

extern crate cc;

fn cc_build_filename(filename: &str) {
    let mut path: String = "../../tests/integration/test_images/".to_owned();
    path.push_str(filename);
    path.push_str(".S");
    cc::Build::new()
        .file(path)
        .define("__FILENAME__", Some(filename))
        .compile(filename);
}

fn main() {
    /* Prepare guestentry/asm_offset.h */
    prepare_asm_offset_header();

    cc::Build::new()
        .file("src/guestentry/enter_guest.S")
        .compile("enter_guest");

    cc::Build::new()
        .file("src/plat/opensbi/uart.c")
        .compile("uart");

    cc::Build::new().file("src/irq/vtimer.S").compile("vtimer");

    let filenames = [
        "vcpu_add_all_gprs",
        "vcpu_ecall_exit",
        "vmem_ld_mapping",
        "vmem_ld_sd_over_loop",
        "vmem_W_Ro",
        "vmem_X_nonX",
        "vmem_ld_sd_sum",
        "vmem_ld_data",
    ];
    for i in 0..filenames.len() {
        cc_build_filename(filenames[i]);
    }
}

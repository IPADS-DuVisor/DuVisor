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

#[allow(unused_imports)]
#[macro_use]
extern crate clap;

pub mod devices;
pub mod irq;
pub mod mm;
pub mod plat;
pub mod test;
pub mod vcpu;
pub mod vm;
use vm::virtualmachine::VirtualMachine;

pub mod init;

use init::cmdline;

pub fn run(config: cmdline::VMConfig) {
    let mut vm = VirtualMachine::new(config);
    let ret = vm.vm_init();

    if ret.len() == 0 {
        /* No kernel data has been loaded */
        panic!("VM init failed");
    }

    vm.vm_run();

    vm.vm_destroy();

    println!("Finish vm running...");
}

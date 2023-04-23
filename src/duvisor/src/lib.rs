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

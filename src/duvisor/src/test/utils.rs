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

pub mod configtest {
    use crate::init::cmdline::VMConfig;

    const ELF_IMG_PATH: &str = "./tests/integration/vcpu_add_all_gprs.img";
    const DTB_PATH: &str = "./test-files-duvisor/hifive-unleashed-a00.dtb";
    const DEFAULT_CONSOLE_TYPE: &str = "none";
    const DEFAULT_VMTAP_NAME: &str = "vmtap0";
    const DEFAULT_BLOCK_PATH: &str = "/blk-dev.img";

    pub fn test_vm_config_create() -> VMConfig {
        let mut vm_config = VMConfig::gen_empty_config();
        vm_config.set_vcpu_count(1);
        vm_config.set_mem_size(8192);
        vm_config.set_kernel_img_path(String::from(ELF_IMG_PATH));
        vm_config.set_dtb_path(String::from(DTB_PATH));
        vm_config.set_console_type(String::from(DEFAULT_CONSOLE_TYPE));
        vm_config.set_vmtap_name(String::from(DEFAULT_VMTAP_NAME));
        vm_config.set_block_path(String::from(DEFAULT_BLOCK_PATH));
        vm_config
    }
}

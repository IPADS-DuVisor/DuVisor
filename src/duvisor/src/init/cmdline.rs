use colored::*;
use std::fs;
use std::path::Path;

use clap::App;

pub const MAX_VCPU: u32 = 8;

pub struct VMConfig {
    vcpu_count: u32,
    mem_size_mb: u64,
    machine_type: String,
    kernel_img_path: String,
    initrd_path: String,
    dtb_path: String,
    console_type: String,
    vmtap_name: String,
    block_path: String,
    kernel_cmdline: String,
}

impl VMConfig {
    pub fn new() -> Result<VMConfig, &'static str> {
        let mut vm_config = VMConfig {
            vcpu_count: 0,
            mem_size_mb: 0,
            machine_type: String::from(""),
            kernel_img_path: String::from(""),
            initrd_path: String::from(""),
            dtb_path: String::from(""),
            console_type: String::from("tty"),
            vmtap_name: String::from(""),
            block_path: String::from(""),
            kernel_cmdline: String::from(""),
        };

        let yaml = load_yaml!("../clap_config.yml");
        let matches = App::from_yaml(yaml).get_matches();

        /* We get VM arguments from vm_config file. */
        if matches.is_present("vm_config") {
            let vm_config_path = matches.value_of("vm_config").unwrap().to_string();
            println!("vm_config_path = {}", vm_config_path);
            let vm_config_contents = match fs::read_to_string(vm_config_path) {
                Ok(contents) => contents,
                Err(_e) => return Err("Failed to read vm_config file"),
            };

            if !VMConfig::parse_vm_config_file(&vm_config_contents, &mut vm_config) {
                return Err("Failed to parse vm config file!");
            }
        } else {
            /* We get VM arguments from command line. */
            /* Get machine type */
            if matches.is_present("machine") {
                vm_config.machine_type = matches.value_of("machine").unwrap().to_string();
            }

            /* Get vcpu count */
            vm_config.vcpu_count = value_t!(matches.value_of("smp"), u32).unwrap_or(0);
            if vm_config.vcpu_count == 0 {
                return Err("please set vcpu count by using --smp or config files.");
            }

            /* Get vmtap */
            if matches.is_present("vmtap") {
                vm_config.vmtap_name = matches.value_of("vmtap").unwrap().to_string();
                println!("[debug] cmdline: Net device: {}", vm_config.vmtap_name);
            }

            /* Get console */
            if matches.is_present("console") {
                vm_config.console_type = matches.value_of("console").unwrap().to_string();
                println!("Console device: {}", vm_config.console_type);
            }

            /* Get path of the block device */
            if matches.is_present("block") {
                vm_config.block_path = matches.value_of("block").unwrap().to_string();
                println!("[debug] cmdline: Block device: {}", vm_config.block_path);
            }

            /* Get memory size */
            vm_config.mem_size_mb = value_t!(matches.value_of("memory"), u64).unwrap_or(0);
            if vm_config.mem_size_mb == 0 {
                return Err("please set memory size by using --memory or config files.");
            }

            /* Get kernel_image_path */
            if matches.is_present("kernel") {
                vm_config.kernel_img_path = matches.value_of("kernel").unwrap().to_string();
            } else {
                return Err("please set kernel image by using --kernel or config files.");
            }

            /* Get dtb_path */
            if matches.is_present("dtb") {
                vm_config.dtb_path = matches.value_of("dtb").unwrap().to_string()
            }

            /* Get initrd_path */
            if matches.is_present("initrd") {
                vm_config.initrd_path = matches.value_of("initrd").unwrap().to_string();
            }

            /* Get kernel cmdline */
            if matches.is_present("append") {
                vm_config.kernel_cmdline = matches.value_of("append").unwrap().to_string();
                println!("[debug] cmdline: append: {}", vm_config.kernel_cmdline);
            }
        }

        Ok(vm_config)
    }

    pub fn gen_empty_config() -> VMConfig {
        Self {
            vcpu_count: 0,
            mem_size_mb: 0,
            machine_type: String::from(""),
            kernel_img_path: String::from(""),
            initrd_path: String::from(""),
            dtb_path: String::from(""),
            console_type: String::from("tty"),
            vmtap_name: String::from("vmtap0"),
            block_path: String::from("/blk-dev.img"),
            kernel_cmdline: String::from(""),
        }
    }

    pub fn get_vcpu_count(&self) -> u32 {
        self.vcpu_count
    }

    pub fn set_vcpu_count(&mut self, val: u32) {
        self.vcpu_count = val;
    }

    pub fn get_mem_size(&self) -> u64 {
        self.mem_size_mb << 20
    }

    pub fn set_mem_size(&mut self, val: u64) {
        self.mem_size_mb = val
    }

    pub fn get_machine_type(&self) -> &str {
        &self.machine_type
    }

    pub fn get_kernel_img_path(&self) -> &str {
        &self.kernel_img_path
    }

    pub fn set_kernel_img_path(&mut self, s: String) {
        self.kernel_img_path = s;
    }

    pub fn get_initrd_path(&self) -> &str {
        &self.initrd_path
    }

    pub fn set_initrd_path(&mut self, s: String) {
        self.initrd_path = s;
    }

    pub fn get_dtb_path(&self) -> &str {
        &self.dtb_path
    }

    pub fn set_dtb_path(&mut self, s: String) {
        self.dtb_path = s;
    }

    pub fn get_console_type(&self) -> &str {
        &self.console_type
    }

    pub fn set_console_type(&mut self, s: String) {
        self.console_type = s;
    }

    pub fn get_vmtap_name(&self) -> &str {
        &self.vmtap_name
    }

    pub fn need_vmtap(&self) -> bool {
        !self.vmtap_name.is_empty()
    }

    pub fn need_virtio_blk(&self) -> bool {
        !self.block_path.is_empty()
    }

    pub fn set_vmtap_name(&mut self, s: String) {
        self.vmtap_name = s;
    }

    pub fn get_block_path(&self) -> &str {
        &self.block_path
    }

    pub fn set_block_path(&mut self, s: String) {
        self.block_path = s;
    }

    pub fn get_kernel_cmdline(&self) -> &str {
        &self.kernel_cmdline
    }

    /*
     * Parsing vm configs from the config file
     * All existing arguments in vm_config struct will be overwritten.
     */
    fn parse_vm_config_file(contents: &String, vm_config: &mut VMConfig) -> bool {
        for line in contents.lines() {
            let words = line.split("=").collect::<Vec<&str>>();
            match words[0].trim() {
                "smp" => {
                    if words.len() >= 2 {
                        vm_config.vcpu_count =
                            words[1].trim().to_string().parse::<u32>().unwrap_or(0);
                    }
                }
                "memory" => {
                    if words.len() >= 2 {
                        vm_config.mem_size_mb =
                            words[1].trim().to_string().parse::<u64>().unwrap_or(0);
                    }
                }
                "kernel" => {
                    if words.len() >= 2 {
                        vm_config.kernel_img_path = words[1].trim().to_string();
                    }
                }
                "initrd" => {
                    if words.len() >= 2 {
                        vm_config.initrd_path = words[1].trim().to_string();
                    }
                }
                "dtb" => {
                    if words.len() >= 2 {
                        vm_config.dtb_path = words[1].trim().to_string();
                    }
                }
                "machine" => {
                    if words.len() >= 2 {
                        vm_config.machine_type = words[1].trim().to_string();
                    }
                }
                _ => {
                    vm_config.vcpu_count = 0;
                    vm_config.mem_size_mb = 0;
                    vm_config.machine_type = String::from("");
                    vm_config.kernel_img_path = String::from("");
                    vm_config.initrd_path = String::from("");
                    vm_config.dtb_path = String::from("");

                    eprintln!(
                        "{} failed to parse argument {}",
                        "error:".bright_red(),
                        words[0]
                    );
                    return false;
                }
            }
        }

        true
    }

    /*
     * Check whether arguments in vm_config are legal or not.
     */
    pub fn verify_args(vm_config: &VMConfig) -> bool {
        if vm_config.vcpu_count == 0 || vm_config.vcpu_count > MAX_VCPU {
            eprintln!("{} failed to set vcpu_count", "error:".bright_red());
            return false;
        }

        if vm_config.mem_size_mb == 0 {
            eprintln!("{} failed to set memory size", "error:".bright_red());
            return false;
        }

        if vm_config.machine_type != "duvisor_virt" && vm_config.machine_type != "test_type" {
            eprintln!(
                "{} failed to set machine_type for {}",
                "error:".bright_red(),
                vm_config.machine_type
            );
            return false;
        }

        if !Path::new(&vm_config.kernel_img_path).is_file() {
            eprintln!(
                "{} failed to open kernel file {}",
                "error:".bright_red(),
                vm_config.kernel_img_path
            );
            return false;
        }

        if vm_config.initrd_path.len() != 0 {
            if !Path::new(&vm_config.initrd_path).is_file() {
                eprintln!(
                    "{} failed to open initrd file {}",
                    "error:".bright_red(),
                    vm_config.initrd_path
                );
                return false;
            }
        }

        if vm_config.dtb_path.len() != 0 {
            if !Path::new(&vm_config.dtb_path).is_file() {
                eprintln!(
                    "{} failed to open dtb file {}",
                    "error:".bright_red(),
                    vm_config.dtb_path
                );
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_vm_config(
        vcpu: u32,
        mem: u64,
        machine: &str,
        kernel: &str,
        initrd: &str,
        dtb: &str,
    ) -> VMConfig {
        VMConfig {
            vcpu_count: vcpu,
            mem_size_mb: mem,
            machine_type: String::from(machine),
            kernel_img_path: String::from(kernel),
            initrd_path: String::from(initrd),
            dtb_path: String::from(dtb),
            console_type: String::from("tty"),
            vmtap_name: String::from("vmtap0"),
            block_path: String::from("/blk-dev.img"),
            kernel_cmdline: String::from(""),
        }
    }

    #[test]
    fn test_verify_args_normal() {
        let vm_config = setup_vm_config(
            2,
            20,
            "duvisor_virt",
            "src/duvisor/src/unit/unitest_kernel",
            "",
            "",
        );
        assert_eq!(VMConfig::verify_args(&vm_config), true);
    }

    #[test]
    fn test_verify_args_vcpu_count_large_value() {
        let vm_config = setup_vm_config(
            1024,
            20,
            "duvisor_virt",
            "src/duvisor/src/unit/unitest_kernel",
            "",
            "",
        );
        assert_eq!(VMConfig::verify_args(&vm_config), false);
    }

    #[test]
    fn test_verify_args_vcpu_count_zero() {
        let vm_config = setup_vm_config(
            0,
            20,
            "duvisor_virt",
            "src/duvisor/src/unit/unitest_kernel",
            "",
            "",
        );
        assert_eq!(VMConfig::verify_args(&vm_config), false);
    }

    #[test]
    fn test_verify_args_mem_zero() {
        let vm_config = setup_vm_config(
            4,
            0,
            "duvisor_virt",
            "src/duvisor/src/unit/unitest_kernel",
            "",
            "",
        );
        assert_eq!(VMConfig::verify_args(&vm_config), false);
    }

    #[test]
    fn test_verify_args_type_invalid() {
        let vm_config = setup_vm_config(
            4,
            1024,
            "duvisor_virt2",
            "src/duvisor/src/unit/unitest_kernel",
            "",
            "",
        );
        assert_eq!(VMConfig::verify_args(&vm_config), false);
    }

    #[test]
    fn test_verify_args_kernel_img_not_exist() {
        let vm_config = setup_vm_config(4, 1024, "duvisor_virt", "err_unitest_kernel", "", "");
        assert_eq!(VMConfig::verify_args(&vm_config), false);
    }

    #[test]
    fn test_verify_args_initrd_invalid() {
        let vm_config = setup_vm_config(
            4,
            1024,
            "duvisor_virt",
            "src/duvisor/src/unit/unitest_kernel",
            "err_initrd",
            "",
        );
        assert_eq!(VMConfig::verify_args(&vm_config), false);
    }

    #[test]
    fn test_verify_args_dtb_invalid() {
        let vm_config = setup_vm_config(
            4,
            1024,
            "duvisor_virt",
            "src/duvisor/src/unit/unitest_kernel",
            "",
            "err_dtb",
        );
        assert_eq!(VMConfig::verify_args(&vm_config), false);
    }

    #[test]
    fn test_parse_vm_config_file_normal() {
        let mut vm_config = setup_vm_config(0, 0, "", "", "", "");

        let contents_str = "smp = 3\r\nmemory = 320\r\nkernel = kernel.file\r\n\
            initrd = initrd.file\r\ndtb = dtb.file\r\nmachine = test_type";
        let contents = String::from(contents_str);
        assert_eq!(
            true,
            VMConfig::parse_vm_config_file(&contents, &mut vm_config)
        );

        assert_eq!(vm_config.vcpu_count, 3);
        assert_eq!(vm_config.mem_size_mb, 320);
        assert_eq!(vm_config.kernel_img_path, "kernel.file");
        assert_eq!(vm_config.machine_type, "test_type");
        assert_eq!(vm_config.initrd_path, "initrd.file");
        assert_eq!(vm_config.dtb_path, "dtb.file");
    }

    #[test]
    fn test_parse_vm_config_file_smp_invalid() {
        let mut vm_config = setup_vm_config(0, 0, "", "", "", "");

        let contents_str = "smp = asd\r\nmemory = 320\r\nkernel = kernel.file\r\n\
            initrd = initrd.file\r\ndtb = dtb.file\r\nmachine = test_type";
        let contents = String::from(contents_str);
        assert_eq!(
            true,
            VMConfig::parse_vm_config_file(&contents, &mut vm_config)
        );

        assert_eq!(vm_config.vcpu_count, 0);
        assert_eq!(vm_config.mem_size_mb, 320);
        assert_eq!(vm_config.kernel_img_path, "kernel.file");
        assert_eq!(vm_config.machine_type, "test_type");
        assert_eq!(vm_config.initrd_path, "initrd.file");
        assert_eq!(vm_config.dtb_path, "dtb.file");
    }

    #[test]
    fn test_parse_vm_config_file_smp_empty() {
        let mut vm_config = setup_vm_config(0, 0, "", "", "", "");

        let contents_str = "smp =\r\nmemory = 320\r\nkernel = kernel.file\r\n\
            initrd = initrd.file\r\ndtb = dtb.file\r\nmachine = test_type";
        let contents = String::from(contents_str);
        assert_eq!(
            true,
            VMConfig::parse_vm_config_file(&contents, &mut vm_config)
        );

        assert_eq!(vm_config.vcpu_count, 0);
        assert_eq!(vm_config.mem_size_mb, 320);
        assert_eq!(vm_config.kernel_img_path, "kernel.file");
        assert_eq!(vm_config.machine_type, "test_type");
        assert_eq!(vm_config.initrd_path, "initrd.file");
        assert_eq!(vm_config.dtb_path, "dtb.file");
    }

    #[test]
    fn test_parse_vm_config_file_smp_no_equalsymbol() {
        let mut vm_config = setup_vm_config(0, 0, "", "", "", "");

        let contents_str = "smp\r\nmemory = 320\r\nkernel = kernel.file\r\n\
            initrd = initrd.file\r\ndtb = dtb.file\r\nmachine = test_type";
        let contents = String::from(contents_str);
        assert_eq!(
            true,
            VMConfig::parse_vm_config_file(&contents, &mut vm_config)
        );

        assert_eq!(vm_config.vcpu_count, 0);
        assert_eq!(vm_config.mem_size_mb, 320);
        assert_eq!(vm_config.kernel_img_path, "kernel.file");
        assert_eq!(vm_config.machine_type, "test_type");
        assert_eq!(vm_config.initrd_path, "initrd.file");
        assert_eq!(vm_config.dtb_path, "dtb.file");
    }

    #[test]
    fn test_parse_vm_config_file_memory_invalid() {
        let mut vm_config = setup_vm_config(0, 0, "", "", "", "");

        let contents_str = "smp = 3\r\nmemory = asdas\r\nkernel = kernel.file\r\n\
            initrd = initrd.file\r\ndtb = dtb.file\r\nmachine = test_type";
        let contents = String::from(contents_str);
        assert_eq!(
            true,
            VMConfig::parse_vm_config_file(&contents, &mut vm_config)
        );

        assert_eq!(vm_config.vcpu_count, 3);
        assert_eq!(vm_config.mem_size_mb, 0);
        assert_eq!(vm_config.kernel_img_path, "kernel.file");
        assert_eq!(vm_config.machine_type, "test_type");
        assert_eq!(vm_config.initrd_path, "initrd.file");
        assert_eq!(vm_config.dtb_path, "dtb.file");
    }

    #[test]
    fn test_parse_vm_config_file_memory_emptry() {
        let mut vm_config = setup_vm_config(0, 0, "", "", "", "");

        let contents_str = "smp = 3\r\nmemory =\r\nkernel = kernel.file\r\n\
            initrd = initrd.file\r\ndtb = dtb.file\r\nmachine = test_type";
        let contents = String::from(contents_str);
        assert_eq!(
            true,
            VMConfig::parse_vm_config_file(&contents, &mut vm_config)
        );

        assert_eq!(vm_config.vcpu_count, 3);
        assert_eq!(vm_config.mem_size_mb, 0);
        assert_eq!(vm_config.kernel_img_path, "kernel.file");
        assert_eq!(vm_config.machine_type, "test_type");
        assert_eq!(vm_config.initrd_path, "initrd.file");
        assert_eq!(vm_config.dtb_path, "dtb.file");
    }

    #[test]
    fn test_parse_vm_config_file_memory_no_equalsymbol() {
        let mut vm_config = setup_vm_config(0, 0, "", "", "", "");

        let contents_str = "smp = 3\r\nmemory\r\nkernel = kernel.file\r\n\
            initrd = initrd.file\r\ndtb = dtb.file\r\nmachine = test_type";
        let contents = String::from(contents_str);
        assert_eq!(
            true,
            VMConfig::parse_vm_config_file(&contents, &mut vm_config)
        );

        assert_eq!(vm_config.vcpu_count, 3);
        assert_eq!(vm_config.mem_size_mb, 0);
        assert_eq!(vm_config.kernel_img_path, "kernel.file");
        assert_eq!(vm_config.machine_type, "test_type");
        assert_eq!(vm_config.initrd_path, "initrd.file");
        assert_eq!(vm_config.dtb_path, "dtb.file");
    }

    #[test]
    fn test_parse_vm_config_file_multiple_values_in_one_line() {
        let mut vm_config = setup_vm_config(0, 0, "", "", "", "");

        let contents_str = "smp = 3\r\nmemory = 320\r\nkernel = kernel.file = two = three\r\n\
            initrd = initrd.file\r\ndtb = dtb.file\r\nmachine = test_type";
        let contents = String::from(contents_str);
        assert_eq!(
            true,
            VMConfig::parse_vm_config_file(&contents, &mut vm_config)
        );

        assert_eq!(vm_config.vcpu_count, 3);
        assert_eq!(vm_config.mem_size_mb, 320);
        assert_eq!(vm_config.kernel_img_path, "kernel.file");
        assert_eq!(vm_config.machine_type, "test_type");
        assert_eq!(vm_config.initrd_path, "initrd.file");
        assert_eq!(vm_config.dtb_path, "dtb.file");
    }

    #[test]
    fn test_parse_vm_config_file_long_string_value() {
        let mut vm_config = setup_vm_config(0, 0, "", "", "", "");

        let contents_str = "smp = 3\r\nmemory = 320\r\nkernel = kernel.file\r\n\
            initrd = initrd.fileeeeeeeeeeeeeeeeeeeeeeeeee\ndtb = dtb.file\r\nmachine = test_type";
        let contents = String::from(contents_str);
        assert_eq!(
            true,
            VMConfig::parse_vm_config_file(&contents, &mut vm_config)
        );

        assert_eq!(vm_config.vcpu_count, 3);
        assert_eq!(vm_config.mem_size_mb, 320);
        assert_eq!(vm_config.kernel_img_path, "kernel.file");
        assert_eq!(vm_config.machine_type, "test_type");
        assert_eq!(
            vm_config.initrd_path,
            "initrd.fileeeeeeeeeeeeeeeeeeeeeeeeee"
        );
        assert_eq!(vm_config.dtb_path, "dtb.file");
    }

    #[test]
    fn test_parse_vm_config_file_invalid_arg() {
        let mut vm_config = setup_vm_config(0, 0, "", "", "", "");

        let contents_str =
            "smp = 3\r\nmemory = 320\r\ninvalid = invalid\r\nkernel = kernel.file\r\n\
            initrd = initrd.file\ndtb = dtb.file\r\nmachine = test_type\r\n";
        let contents = String::from(contents_str);
        assert_eq!(
            false,
            VMConfig::parse_vm_config_file(&contents, &mut vm_config)
        );

        assert_eq!(vm_config.vcpu_count, 0);
        assert_eq!(vm_config.mem_size_mb, 0);
        assert_eq!(vm_config.kernel_img_path, "");
        assert_eq!(vm_config.machine_type, "");
        assert_eq!(vm_config.initrd_path, "");
        assert_eq!(vm_config.dtb_path, "");
    }
}

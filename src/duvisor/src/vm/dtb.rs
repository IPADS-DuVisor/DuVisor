use crate::{dbgprintln, init::cmdline::VMConfig};
use vm_fdt::{Error, FdtWriter};

pub const DTB_GPA: u64 = 0x82200000;
pub const BLOCK_DEV_MMIO_ADDR: u64 = 0x10000000;
pub const BLOCK_DEV_MMIO_LEN: u64 = 0x200;
pub const NET_DEV_MMIO_ADDR: u64 = 0x10000200;
pub const NET_DEV_MMIO_LEN: u64 = 0x200;
pub const MEM_START: u32 = 0x80000000;
pub const TEST_MEM_START: u32 = 0x1000;
pub const INITRD_START: u64 = 0x853907f8;
pub const INITRD_END: u64 = 0x87fffff8;
pub const PLIC_HPA: u64 = 0xc000000;
pub const PLIC_LENGTH: u64 = 0x4000000;
pub const VIRT_SERIAL_IRQ: u64 = 10 + 1;
pub const VIRT_BLOCK_IRQ: u64 = 10 + 2;
pub const VIRT_NET_IRQ: u64 = 10 + 3;

#[allow(unused)]
struct BusRegion {
    offset: u64,
    size: u64,
}

pub const DTB_TARGET_PROP: [&str; 5] = [
    "memory",
    "soc",
    "chosen",
    "linux,initrd-start",
    "linux,initrd-end",
];

impl BusRegion {
    pub fn new(offset: u64, size: u64) -> Self {
        Self { offset, size }
    }
}
struct InitrdRegion {
    pub start: u64,
    pub end: u64,
}

impl InitrdRegion {
    pub fn new() -> Self {
        let start: u64 = 0;
        let end: u64 = 0;

        Self { start, end }
    }
}

struct MachineMeta {
    address_cells: Vec<u32>,
    size_cells: Vec<u32>,
    memory_regions: Vec<BusRegion>,
    soc_regions: Vec<BusRegion>,
    initrd_region: InitrdRegion,
}

impl MachineMeta {
    const INITRD_START: i32 = 0;
    const INITRD_END: i32 = 1;

    pub fn new() -> Self {
        let address_cells: Vec<u32> = Vec::new();
        let size_cells: Vec<u32> = Vec::new();
        let memory_regions: Vec<BusRegion> = Vec::new();
        let soc_regions: Vec<BusRegion> = Vec::new();
        let initrd_region: InitrdRegion = InitrdRegion::new();

        Self {
            address_cells,
            size_cells,
            memory_regions,
            soc_regions,
            initrd_region,
        }
    }

    fn gen_cpu_dtb(cpu_id: u32, fdt: &mut FdtWriter) -> Result<u32, Error> {
        let s = format!("cpu@{cpu_id}");

        let cpu_node = fdt.begin_node(&s)?;
        fdt.property_string("device_type", "cpu")?;
        fdt.property_string("compatible", "riscv")?;
        fdt.property_string("mmu-type", "riscv,sv48")?;
        fdt.property_string("riscv,isa", "rv64imafdcsu")?;
        fdt.property_u32("reg", cpu_id)?;
        fdt.property_string("status", "okay")?;
        let intc = fdt.begin_node("interrupt-controller")?;
        fdt.property_string("compatible", "riscv,cpu-intc")?;
        fdt.property_u32("#interrupt-cells", 0x1)?;
        fdt.property_null("interrupt-controller")?;
        fdt.property_u32("phandle", cpu_id + 2)?;
        fdt.end_node(intc)?;
        fdt.end_node(cpu_node)?;
        Ok(0)
    }

    fn gen_dtb(vm_config: &VMConfig) -> Result<Vec<u8>, Error> {
        let mut fdt = FdtWriter::new()?;
        let cpu_nr = vm_config.get_vcpu_count();
        let mem_sz = vm_config.get_mem_size();
        let mem_sz_lo: u32 = (mem_sz & ((1u64 << 32) - 1)) as u32;
        let mem_sz_hi: u32 = (mem_sz >> 32) as u32;

        let root_node = fdt.begin_node("")?;
        fdt.property_string("compatible", "linux,dummy-virt")?;
        fdt.property_u32("#address-cells", 0x2)?;
        fdt.property_u32("#size-cells", 0x2)?;

        let chosen_node = fdt.begin_node("chosen")?;
        fdt.property_string("bootargs", &vm_config.get_kernel_cmdline())?;
        // fdt.property_string("bootargs", "console=ttyS0 root=/dev/vda rw  console=sbi earlycon=sbi")?;
        fdt.property_string("stdout-path", "sbi")?;
        fdt.property_u64("linux,initrd-start", INITRD_START)?;
        fdt.property_u64("linux,initrd-end", INITRD_END)?;
        fdt.end_node(chosen_node)?;

        let memory_node = fdt.begin_node("memory")?;
        let reg: Vec<u32> = vec![0, MEM_START, mem_sz_hi, mem_sz_lo];
        fdt.property_string("device_type", "memory")?;
        fdt.property_array_u32("reg", &reg)?;
        fdt.end_node(memory_node)?;

        let cpus_node = fdt.begin_node("cpus")?;
        fdt.property_u32("#address-cells", 0x1)?;
        fdt.property_u32("#size-cells", 0x0)?;
        fdt.property_u32("timebase-frequency", 0x989680)?;

        for i in 0..cpu_nr {
            MachineMeta::gen_cpu_dtb(i, &mut fdt)?;
        }

        fdt.end_node(cpus_node)?;

        let smb = fdt.begin_node("smb")?;
        fdt.property_string("compatible", "simple-bus")?;
        fdt.property_u32("#address-cells", 0x2)?;
        fdt.property_u32("#size-cells", 0x2)?;
        fdt.property_u32("interrupt-parent", 0x1)?;
        fdt.property_null("ranges")?;
        let smb_intc = fdt.begin_node("interrupt-controller@0c000000")?;
        fdt.property_string("compatible", "riscv,plic0")?;
        let reg: Vec<u32> = vec![0x0, PLIC_HPA as u32, 0x0, PLIC_LENGTH as u32];
        fdt.property_array_u32("reg", &reg)?;
        fdt.property_u32("#interrupt-cells", 0x1)?;
        fdt.property_null("interrupt-controller")?;
        fdt.property_u32("riscv,max-priority", 0xf)?;
        fdt.property_u32("riscv,ndev", 0x1f)?;
        fdt.property_u32("phandle", 0x1)?;
        let mut int_ext: Vec<u32> = vec![];
        for i in 0..cpu_nr {
            int_ext.push(i + 2);
            int_ext.push(0xffffffff);
            int_ext.push(i + 2);
            int_ext.push(0xffffffff);
            int_ext.push(i + 2);
            int_ext.push(0x9);
        }
        fdt.property_array_u32("interrupts-extended", &int_ext)?;
        fdt.end_node(smb_intc)?;

        let u6 = fdt.begin_node("U6_16550A@3f8")?;
        fdt.property_string("compatible", "ns16550a")?;
        let reg: Vec<u32> = vec![0x0, 0x3f8, 0x0, 0x8];
        fdt.property_array_u32("reg", &reg)?;
        fdt.property_u32("interrupts", VIRT_SERIAL_IRQ as u32)?;
        fdt.property_u32("clock-frequency", 0x1c2000)?;
        fdt.end_node(u6)?;

        if vm_config.need_virtio_blk() {
            let block_dev_name = format!("virtio@{BLOCK_DEV_MMIO_ADDR}");
            let virtio_blk = fdt.begin_node(&block_dev_name)?;
            fdt.property_string("compatible", "virtio,mmio")?;
            let reg: Vec<u32> = vec![
                0x00,
                BLOCK_DEV_MMIO_ADDR as u32,
                0x00,
                BLOCK_DEV_MMIO_LEN as u32,
            ];
            fdt.property_array_u32("reg", &reg)?;
            fdt.property_null("dma-coherent")?;
            fdt.property_u32("interrupts", VIRT_BLOCK_IRQ as u32)?;
            fdt.end_node(virtio_blk)?;
        }

        if vm_config.need_vmtap() {
            let net_dev_name = format!("virtio@{NET_DEV_MMIO_ADDR}");
            let virtio_net = fdt.begin_node(&net_dev_name)?;
            fdt.property_string("compatible", "virtio,mmio")?;
            let reg: Vec<u32> = vec![
                0x00,
                NET_DEV_MMIO_ADDR as u32,
                0x00,
                NET_DEV_MMIO_LEN as u32,
            ];
            fdt.property_array_u32("reg", &reg)?;
            fdt.property_null("dma-coherent")?;
            fdt.property_u32("interrupts", VIRT_NET_IRQ as u32)?;
            fdt.end_node(virtio_net)?;
        }

        fdt.end_node(smb)?;

        let alias = fdt.begin_node("aliases")?;
        fdt.property_string("serial0", "/U6_16550A@3f8")?;
        fdt.end_node(alias)?;

        fdt.end_node(root_node)?;

        fdt.finish()
    }

    pub fn apply(&mut self) {
        self.initrd_region.start = INITRD_START;
        self.initrd_region.end = INITRD_END;
    }

    /*
     * Currently, generating dtb based on command line parameters is supported.
     * However, only one machine type is supported.
     * The input dtb parameter would be ignored.
     * Therefore, the functions to parse input dtb are deprecated.
     * TODO: to be updated later
     */
    #[deprecated]
    #[allow(warnings)]
    pub fn dtb_parse(&mut self, item: &dtb::StructItem, node_path: &Vec<&str>, file_path: &str) {
        let mut prop = None;
        let mut file_data = std::fs::read(file_path).unwrap_or_else(|_| {
            panic!("read file failed");
        });
        /* Set address-cells */
        if item.name().unwrap().contains("address-cells") {
            self.address_cells.pop();
            self.address_cells
                .push(item.value_u32_list(&mut file_data).unwrap()[0]);
            dbgprintln!("match address-cells {:?}", self.address_cells);
            return;
        }

        /* Set size-cells */
        if item.name().unwrap().contains("size-cells") {
            self.size_cells.pop();
            self.size_cells
                .push(item.value_u32_list(&mut file_data).unwrap()[0]);
            dbgprintln!("match size-cells {:?}", self.size_cells);
            return;
        }

        /* Address_cells and size_cells shall be set first */
        let ac_len = self.address_cells.len();
        let sc_len = self.size_cells.len();
        assert_eq!(ac_len, sc_len);

        let mut address_cells = 2;
        let mut size_cells = 1;

        if ac_len > 1 {
            address_cells = self.address_cells[ac_len - 2];
            size_cells = self.size_cells[sc_len - 2];
        }

        dbgprintln!("cells {} {}", address_cells, size_cells);

        for i in node_path {
            for j in DTB_TARGET_PROP.iter() {
                if i.contains(j) {
                    dbgprintln!("find {} in {}", j, i);
                    prop = Some(*j);
                    break;
                }
            }
        }

        let prop_name = item.name().unwrap();
        match prop {
            None => return,
            Some("memory") => {
                return;
                dbgprintln!("match memory");
                /* Add bus region for memory */
                if prop_name == "reg" {
                    let values = item.value_u32_list(&mut file_data).unwrap();
                    println!("[debug] MEMORY REG {:x?}", values);
                    self.memory_parse(values, address_cells, size_cells);
                }
            }
            Some("soc") => {
                dbgprintln!("match soc");

                if prop_name == "reg" && node_path.len() == 3 {
                    let values = item.value_u32_list(&mut file_data).unwrap();
                    println!("[debug] SOC REG {:x?}", values);
                    self.soc_parse(values, address_cells, size_cells);
                }
            }
            Some("chosen") => {
                dbgprintln!("match chosen");

                if prop_name == "linux,initrd-start" && node_path.len() == 2 {
                    let values = item.value_u32_list(&mut file_data).unwrap();
                    println!("[debug] INITRD-START REG {:x?}", values);
                    self.initrd_parse(values, address_cells, size_cells, MachineMeta::INITRD_START);
                }

                if prop_name == "linux,initrd-end" && node_path.len() == 2 {
                    let values = item.value_u32_list(&mut file_data).unwrap();
                    println!("[debug] INITRD-END REG {:x?}", values);
                    self.initrd_parse(values, address_cells, size_cells, MachineMeta::INITRD_END);
                }
            }
            _ => {
                dbgprintln!("match nothing");
            }
        }
    }

    #[deprecated]
    pub fn soc_parse(&mut self, value_u32_list: &[u32], address_cells: u32, size_cells: u32) {
        dbgprintln!(
            "soc_parse {} {} {:x?}",
            address_cells,
            size_cells,
            value_u32_list
        );
        let len = value_u32_list.len() as u32;
        let t = address_cells + size_cells;
        let cycle: u32 = len / t;

        assert_eq!(len % t, 0);
        if len == 0 {
            return;
        }

        for i in 0..cycle {
            let mut offset: u64 = 0;
            let mut size: u64 = 0;

            for j in 0..address_cells {
                offset = (offset << 32) + (value_u32_list[(i * t + j) as usize] as u64);
            }

            for k in 0..size_cells {
                size = (size << 32) + (value_u32_list[(i * t + address_cells + k) as usize] as u64);
            }

            /* Add memory region */
            self.soc_regions.push(BusRegion::new(offset, size));
            dbgprintln!("add soc region {:x} {:x}", offset, size);
        }
    }

    #[deprecated]
    pub fn initrd_parse(
        &mut self,
        value_u32_list: &[u32],
        address_cells: u32,
        size_cells: u32,
        value_type: i32,
    ) {
        let mut prop_value: u64 = 0;
        let cells: u32;

        if value_type == MachineMeta::INITRD_START {
            cells = address_cells;
        } else {
            cells = size_cells;
        }

        for i in 0..cells {
            prop_value = (prop_value << 32) + (value_u32_list[i as usize] as u64);
        }

        dbgprintln!("initrd_parse - prop_value 0x{:x}", prop_value);

        if value_type == MachineMeta::INITRD_START {
            self.initrd_region.start = prop_value;
        } else {
            self.initrd_region.end = prop_value;
        }
    }

    #[deprecated]
    pub fn memory_parse(&mut self, value_u32_list: &[u32], address_cells: u32, size_cells: u32) {
        println!(
            "[debug] memory_parse {:x?} address_cells: {} size_cells: {}",
            value_u32_list, address_cells, size_cells
        );
        let len = value_u32_list.len() as u32;
        let t = address_cells + size_cells;
        let cycle: u32 = len / t;

        assert_eq!(len % t, 0);
        if len == 0 {
            return;
        }

        for i in 0..cycle {
            let mut offset: u64 = 0;
            let mut size: u64 = 0;

            for j in 0..address_cells {
                offset = (offset << 32) + (value_u32_list[(i * t + j) as usize] as u64);
            }

            for k in 0..size_cells {
                size = (size << 32) + (value_u32_list[(i * t + address_cells + k) as usize] as u64);
            }

            /* Add memory region */
            self.memory_regions.push(BusRegion::new(offset, size));
        }
    }
}

/* Read and parse the vm img file */
pub struct DeviceTree {
    file_data: Vec<u8>,
    meta_data: MachineMeta,
}

impl DeviceTree {
    #[allow(unused_variables)]
    pub fn new(file_path: &str, vm_config: &VMConfig) -> Self {
        let mut meta_data = MachineMeta::new();

        /* 1. apply auto-gen dtb file to vm-meta. */
        meta_data.apply();

        /* 2. generate gtb file, pass to guest kernel later. */
        let file_data = MachineMeta::gen_dtb(&vm_config).unwrap();

        Self {
            file_data,
            meta_data,
        }
    }

    /* Get start address of initrd */
    pub fn get_initrd_start(&self) -> u64 {
        self.meta_data.initrd_region.start
    }

    /* Get end address of initrd */
    pub fn get_initrd_end(&self) -> u64 {
        self.meta_data.initrd_region.end
    }

    /* Get dtb file data */
    pub fn get_dtb(&self) -> &Vec<u8> {
        &self.file_data
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of_val;

    use super::*;
    use fdt_rs::base::*;
    use fdt_rs::prelude::*;
    use rusty_fork::rusty_fork_test;

    use crate::test::utils::configtest::test_vm_config_create;

    #[repr(align(4))]
    struct _Wrapper<T>(T);

    rusty_fork_test! {
        #[test]

        fn test_dtb_memory() {
            let mut vm_config = test_vm_config_create();
            let mem_size_mb: u64 = 1024;
            vm_config.set_mem_size(mem_size_mb);
            let dtb = MachineMeta::gen_dtb(&vm_config).unwrap();
            let fdt = &_Wrapper(dtb).0;
            unsafe {
                let blob = DevTree::new(fdt).unwrap();

                let mem_prop = blob
                    .props()
                    .find(|p| Ok(p.name()? == "device_type" && p.str()? == "memory"))
                    .unwrap()
                    .expect("Unable to find memory node.");
                let mem_node = mem_prop.node();

                let x = mem_node
                    .props()
                    .find(|p| Ok(p.name()? == "reg"))
                    .unwrap()
                    .expect("Device tree memory node missing 'reg' prop.");
                assert_eq!(x.length(), 4 * size_of_val(&1u32));

                for i in 0..x.length() / size_of_val(&1u32) {
                    let val = x.u32(i).unwrap();
                    match i {
                        0 => assert_eq!(val, 0),
                        1 => assert_eq!(val, MEM_START),
                        2 => assert_eq!(val, 0),
                        3 => assert_eq!(val as u64, mem_size_mb << 20),
                        _ => panic!("Unexpected index")
                    }
                }
            }
        }
    }
}

# DuVisor

[![ci-status](https://github.com/IPADS-DuVisor/DuVisor/actions/workflows/ci.yml/badge.svg)](https://github.com/IPADS-DuVisor/DuVisor/actions/)[![rust-version](https://img.shields.io/badge/rustc-stable-blue.svg)](https://blog.rust-lang.org/)

<!--ts-->
* [DuVisor](#duvisor)
   * [Quick Start](#quick-start)
      * [Prerequisite](#prerequisite)
      * [Build DuVisor for QEMU](#build-duvisor-for-qemu)
      * [Run Example VM on QEMU](#run-example-vm-on-qemu)
      * [Build DuVisor for FireSim](#build-duvisor-for-firesim)
<!--te-->

## Quick Start

### Prerequisite

Hardware requirements:

* CPU: Commodity CPU with >= 4 cores which is able to run qemu. Architecture is not limitted.
* Memory: >8GB

First, clone this repository:

```bash
git clone https://github.com/IPADS-DuVisor/DuVisor
cd DuVisor
git submodule update --init --recursive
```

We provide a docker image to build the whole environment for x86-64 CPUs, as well as the [Dockerfile](./scripts/opensource/Dockerfile) of this image.

(Optional) To set up native environment for building, you can follow the Dockerfile.

Install Docker: https://docs.docker.com/engine/install/ubuntu/

Prepare source code for guest VM:
```bash
wget https://cdn.kernel.org/pub/linux/kernel/v5.x/linux-5.11.tar.xz
tar xf linux-5.11.tar.xz
mv linux-5.11 linux-guest
```

Prepare source code of busybox for rootfs:
```bash
mkdir -p prepare
cd prepare
git clone https://github.com/kvm-riscv/howto.git
wget https://busybox.net/downloads/busybox-1.33.1.tar.bz2
tar xf busybox-1.33.1.tar.bz2
mv busybox-1.33.1 rootfs
cd ..
```

### Build DuVisor for QEMU

You can build DuVisor and other elements for executing DuVisor (host/guest Linux kernel, rootfs, opensbi) via **one of** the following two scripts: 

[Build via docker](./scripts/quick_docker_build.sh):

```bash
./scripts/quick_docker_build.sh
```

[Native build](./scripts/quick_native_build.sh):

```bash
./scripts/quick_native_build.sh
```

### Run Example VM on QEMU

If you use docker build in last step:

```bash
./scripts/build/docker_exec_wrapper.sh ./scripts/run/example_boot.sh

# In qemu
cd duvisor
./boot.sh
```

If you use native build in last step:

```bash
./scripts/run/example_boot.sh

# In qemu
cd duvisor
./boot.sh
```

You would see `DUVISOR` then.

Reference bootlog:
```
bash> ./scripts/run/example_boot.sh

OpenSBI v0.8                                                                                                           
   ____                    _____ ____ _____                                                                            
  / __ \                  / ____|  _ \_   _|                                                                           
 | |  | |_ __   ___ _ __ | (___ | |_) || |                                                                             
 | |  | | '_ \ / _ \ '_ \ \___ \|  _ < | |                                                                             
 | |__| | |_) |  __/ | | |____) | |_) || |_                                                                            
  \____/| .__/ \___|_| |_|_____/|____/_____|                                                                           
        | |                                                                                                            
        |_|                                                                                                            
                                                                                                                       
Platform Name             : riscv-virtio,qemu

/* Skip some of the log */

[    0.299693] Key type dns_resolver registered
[    0.315945] Freeing unused kernel memory: 192K
[    0.328555] Run /init as init process
           _  _
          | ||_|
          | | _ ____  _   _  _  _ 
          | || |  _ \| | | |\ \/ /
          | || | | | | |_| |/    \
          |_||_|_| |_|\____|\_/\_/

               Busybox Rootfs

Please press Enter to activate this console. 
/ # cd duvisor
/duvisor # ./boot.sh                                                 
[debug] cmdline: append: root=/dev/ram console=ttyS0 earlycon=sbi
                                                                                                                       
                                                           
    ,------.         ,--.   ,--.,--.                               
    |  .-.  \ ,--.,--.\  `.'  / `--' ,---.  ,---. ,--.--.     
    |  |  \  :|  ||  | \     /  ,--.(  .-' | .-. ||  .--'     
    |  '--'  /'  ''  '  \   /   |  |.-'  `)' '-' '|  |    
    `-------'  `----'    `-'    `--'`----'  `---' `--'    
                                                           
                                                                  
Welcome to DUVISOR (Qemu)

/* Skip some of the log */

[    1.271158] Freeing unused kernel memory: 2144K
[    1.298522] Run /init as init process
           _  _
          | ||_|
          | | _ ____  _   _  _  _ 
          | || |  _ \| | | |\ \/ /
          | || | | | | |_| |/    \
          |_||_|_| |_|\____|\_/\_/

               Busybox Rootfs

Please press Enter to activate this console. 
/ # cat /proc/cpuinfo
processor       : 0
hart            : 1
isa             : rv64imafdcsu
mmu             : sv48

processor       : 1
hart            : 0
isa             : rv64imafdcsu
mmu             : sv48
/ # poweroff -f
[  120.441885] reboot: Power down
Poweroff the virtual machine by vcpu 1
[  127.298790] IOCTL_DUVISOR_UNREGISTER_VCPU: tid = 67
[  127.298805] IOCTL_DUVISOR_UNREGISTER_VCPU: tid = 68
DuVisor VM ended normally.
Finish vm running...
[  127.348446] vplic_dev_release:419 tgid = 66
[  127.348795] vplic_dev_release:406 tgid = 66 ulh_vm_data is NULL vplic!
[  127.349096] dv_driver_release:449 tgid = 66 ulh_vm_data is NULL!
/duvisor #
```

### Build DuVisor for FireSim

To run DuVisor on the FireSim platform, you only need to checkout to the `firesim` branch and apply a `firesim.patch` to the guest kernel code.
Other steps are the same as [Build DuVisor for QEMU](#build-duvisor-for-qemu).

To apply `firesim.patch` to the guest kernel code:

```bash
wget https://cdn.kernel.org/pub/linux/kernel/v5.x/linux-5.11.tar.xz
tar xf linux-5.11.tar.xz
mv linux-5.11 linux-guest
cd linux-guest
patch -p1 < ../firesim.patch
```

To run on FireSim, please use the Rocket-Chip with DV-ext enabled.
We have provided a compiled hardware on AWS, you can refer to our AE for more details:

```bash
# config_runtime.ini
defaulthwconfig=firesim-rocket-quadcore-nic-l2-llc4mb-ddr3-ulh-VNA-4TLB-4PTW-IBCN-vssip

# config_hwdb.ini
[firesim-rocket-quadcore-nic-l2-llc4mb-ddr3-ulh-VNA-4TLB-4PTW-IBCN-vssip]
agfi=agfi-0e385553b7716f177
deploytripletoverride=None
customruntimeconfig=None
```

### Security Evaluation for DuVisor

DuVisor is able to prevent host kernel from crashing even if the user-level virtualization module is attacked. We have tested and emulated the following CVEs: they would make DuVisor crash, but the host kernel can continue to execute other programs (including DuVisor VM) normally.

Steps for preparation of the execution environment for security evaluation for DuVisor are the same as [Build DuVisor for QEMU](#build-duvisor-for-qemu).

All the following CVEs can be tested without rebooting or shutting down the host kernel. Therefore, you may boot the host kernel only once:

```bash
# Boot host kernel
./scripts/run/example-boot.sh

# Enter the directory storing the scripts
cd duvisor
```

#### [CVE-2017-12188 Memory Virtualization](https://nvd.nist.gov/vuln/detail/CVE-2017-12188)

Description:

> arch/x86/kvm/mmu.c in the Linux kernel through 4.13.5, when nested virtualisation is used, does not properly traverse guest pagetable entries to resolve a guest virtual address, which allows L1 guest OS users to execute arbitrary code on the host OS or cause a denial of service (incorrect index during page walking, and host OS crash), aka an "MMU potential **stack buffer overrun**."

DuVisor does not support nested virtualization, but we emulate a **stack buffer overrun** in [stage-2 page fault handling](https://github.com/IPADS-DuVisor/DuVisor/blob/security-ae/src/duvisor/src/vcpu/virtualcpu.rs#L612) which would randomly crash DuVisor.

Execute the following command to emulate this CVE:
```bash
./boot-cve.sh 1

# Generate a large amount of stage-2 page fault
mkdir tmp
mount -t tmpfs tmpfs /tmp
dd if=/dev/zero of=/tmp/tmp bs=1G count=2
```

It would crash with output like the following:
```txt
Emulating CVE-2017-12188 (stack buffer overrun) in memory virtualizaion!
```

The host kernel can still run normally. You can continue to boot DuVisor VMs test other CVEs.

#### [CVE-2018-16882 Interrupt Virtualization](https://nvd.nist.gov/vuln/detail/CVE-2018-16882)

Description:

> A **use-after-free** issue was found in the way the Linux kernel's KVM hypervisor processed posted interrupts when nested(=1) virtualization is enabled. In nested_get_vmcs12_pages(), in case of an error while processing posted interrupt address, it unmaps the 'pi_desc_page' without resetting 'pi_desc' descriptor address, which is later used in pi_test_and_clear_on(). A guest user/process could use this flaw to crash the host kernel resulting in DoS or potentially gain privileged access to a system. Kernel versions before 4.14.91 and before 4.19.13 are vulnerable.

We emulate a **use-after-free** in [interrupt virtualization](https://github.com/IPADS-DuVisor/DuVisor/blob/security-ae/src/duvisor/src/devices/vplic.rs#L71) which would randomly crash DuVisor.

Execute the following command to emulate this CVE:
```bash
./boot-cve.sh 2

# Execute ls to get a lot of interrupts from tty device
ls
```

It would crash with output like the following:

```txt
Emulating CVE-2018-16882 (use-after-free) in posted interrupt!
```

#### [CVE-2016-8630 ISA Emulation](https://nvd.nist.gov/vuln/detail/CVE-2016-8630)

Description:

> The x86_decode_insn function in arch/x86/kvm/emulate.c in the Linux kernel before 4.8.7, when KVM is enabled, allows local users to cause a denial of service (host OS crash) via a certain use of a ModR/M byte in an **undefined instruction**.

We emulate an **undefined instruction** in [ISA emulation](https://github.com/IPADS-DuVisor/DuVisor/blob/security-ae/src/duvisor/src/vcpu/virtualcpu.rs#L223) which would randomly crash DuVisor.

Execute the following command to emulate this CVE:
```bash
./boot-cve.sh 3

# Wait and it would crash randomly.
```

It would crash with output like the following:

```txt
Emulating CVE-2016-8630 (undefined instruction) in ISA emulation!
```

#### [CVE-2020-8834 VM Exit Handling](https://nvd.nist.gov/vuln/detail/CVE-2020-8834)

Description:

> KVM in the Linux kernel on Power8 processors has a conflicting use of HSTATE_HOST_R1 to store r1 state in kvmppc_hv_entry plus in kvmppc_{save,restore}_tm, leading to a **stack corruption**. Because of this, an attacker with the ability run code in kernel space of a guest VM can cause the host kernel to panic.

We emulate a **stack corruption** in [VM exit handling](https://github.com/IPADS-DuVisor/DuVisor/blob/security-ae/src/duvisor/src/vcpu/virtualcpu.rs#L757) which would randomly crash DuVisor.

Execute the following command to emulate this CVE:
```bash
./boot-cve.sh 4

# Wait and it would crash randomly.
```

It would crash with output like the following:

```txt
Emulating CVE-2020-8834 (stack corruption) in VM exit!
```

#### [CVE-2016-5412 Para-Virtualization](https://nvd.nist.gov/vuln/detail/CVE-2016-5412)

Description:

> arch/powerpc/kvm/book3s_hv_rmhandlers.S in the Linux kernel through 4.7 on PowerPC platforms, when CONFIG_KVM_BOOK3S_64_HV is enabled, allows guest OS users to cause a denial of service (host OS **infinite loop**) by making a H_CEDE hypercall during the existence of a suspended transaction.

We emulate a **infinite loop** by calling `panic` in [para-virtualization](https://github.com/IPADS-DuVisor/DuVisor/blob/security-ae/src/duvisor/src/plat/opensbi/emulation.rs#L158) which would randomly crash DuVisor.

Execute the following command to emulate this CVE:
```bash
./boot-cve.sh 5

# Wait and it would crash randomly.
```

It would crash with output like the following:

```txt
Emulating CVE-2016-5412 (infinite loop) in para-virtualization!
```

#### [CVE-2019-6974 Device Virtualization](https://nvd.nist.gov/vuln/detail/CVE-2019-6974)

Description:

> In the Linux kernel before 4.20.8, kvm_ioctl_create_device in virt/kvm/kvm_main.c mishandles reference counting because of a race condition, leading to a **use-after-free**.

We emulate a **use-after-free** in [device virtualization](https://github.com/IPADS-DuVisor/DuVisor/blob/security-ae/src/devices/src/serial.rs#L199) which would randomly crash DuVisor.

Execute the following command to emulate this CVE:
```bash
./boot-cve.sh 6

# DuVisor would crash if the tty device prints "DV-cve".
echo DV-cve
```

It would crash with output like the following:

```txt
Emulating CVE-2019-6974 (use-after-free) in device virtualization!
```

#![allow(unused)]
use core::arch::asm;

pub const USTATUS: u64 = 0x000;
pub const UIE: u64 = 0x004;
pub const UTVEC: u64 = 0x005;
pub const USCRATCH: u64 = 0x040;
pub const UEPC: u64 = 0x041;
pub const UCAUSE: u64 = 0x042;
pub const UTVAL: u64 = 0x043;
pub const UIP: u64 = 0x044;
pub const FFLAGS: u64 = 0x001;
pub const FRM: u64 = 0x002;
pub const FCSR: u64 = 0x003;
pub const CYCLE: u64 = 0xC00;
pub const TIME: u64 = 0xC01;
pub const INSTRET: u64 = 0xC02;
pub const HPMCOUNTER3: u64 = 0xC03;
pub const HPMCOUNTER4: u64 = 0xC04;
pub const HPMCOUNTER5: u64 = 0xC05;
pub const HPMCOUNTER6: u64 = 0xC06;
pub const HPMCOUNTER7: u64 = 0xC07;
pub const HPMCOUNTER8: u64 = 0xC08;
pub const HPMCOUNTER9: u64 = 0xC09;
pub const HPMCOUNTER10: u64 = 0xC0A;
pub const HPMCOUNTER11: u64 = 0xC0B;
pub const HPMCOUNTER12: u64 = 0xC0C;
pub const HPMCOUNTER13: u64 = 0xC0D;
pub const HPMCOUNTER14: u64 = 0xC0E;
pub const HPMCOUNTER15: u64 = 0xC0F;
pub const HPMCOUNTER16: u64 = 0xC10;
pub const HPMCOUNTER17: u64 = 0xC11;
pub const HPMCOUNTER18: u64 = 0xC12;
pub const HPMCOUNTER19: u64 = 0xC13;
pub const HPMCOUNTER20: u64 = 0xC14;
pub const HPMCOUNTER21: u64 = 0xC15;
pub const HPMCOUNTER22: u64 = 0xC16;
pub const HPMCOUNTER23: u64 = 0xC17;
pub const HPMCOUNTER24: u64 = 0xC18;
pub const HPMCOUNTER25: u64 = 0xC19;
pub const HPMCOUNTER26: u64 = 0xC1A;
pub const HPMCOUNTER27: u64 = 0xC1B;
pub const HPMCOUNTER28: u64 = 0xC1C;
pub const HPMCOUNTER29: u64 = 0xC1D;
pub const HPMCOUNTER30: u64 = 0xC1E;
pub const HPMCOUNTER31: u64 = 0xC1F;
pub const CYCLEH: u64 = 0xC80;
pub const TIMEH: u64 = 0xC81;
pub const INSTRETH: u64 = 0xC82;
pub const HPMCOUNTER3H: u64 = 0xC83;
pub const HPMCOUNTER4H: u64 = 0xC84;
pub const HPMCOUNTER5H: u64 = 0xC85;
pub const HPMCOUNTER6H: u64 = 0xC86;
pub const HPMCOUNTER7H: u64 = 0xC87;
pub const HPMCOUNTER8H: u64 = 0xC88;
pub const HPMCOUNTER9H: u64 = 0xC89;
pub const HPMCOUNTER10H: u64 = 0xC8A;
pub const HPMCOUNTER11H: u64 = 0xC8B;
pub const HPMCOUNTER12H: u64 = 0xC8C;
pub const HPMCOUNTER13H: u64 = 0xC8D;
pub const HPMCOUNTER14H: u64 = 0xC8E;
pub const HPMCOUNTER15H: u64 = 0xC8F;
pub const HPMCOUNTER16H: u64 = 0xC90;
pub const HPMCOUNTER17H: u64 = 0xC91;
pub const HPMCOUNTER18H: u64 = 0xC92;
pub const HPMCOUNTER19H: u64 = 0xC93;
pub const HPMCOUNTER20H: u64 = 0xC94;
pub const HPMCOUNTER21H: u64 = 0xC95;
pub const HPMCOUNTER22H: u64 = 0xC96;
pub const HPMCOUNTER23H: u64 = 0xC97;
pub const HPMCOUNTER24H: u64 = 0xC98;
pub const HPMCOUNTER25H: u64 = 0xC99;
pub const HPMCOUNTER26H: u64 = 0xC9A;
pub const HPMCOUNTER27H: u64 = 0xC9B;
pub const HPMCOUNTER28H: u64 = 0xC9C;
pub const HPMCOUNTER29H: u64 = 0xC9D;
pub const HPMCOUNTER30H: u64 = 0xC9E;
pub const HPMCOUNTER31H: u64 = 0xC9F;
pub const MCYCLE: u64 = 0xB00;
pub const MINSTRET: u64 = 0xB02;
pub const MCYCLEH: u64 = 0xB80;
pub const MINSTRETH: u64 = 0xB82;
pub const MVENDORID: u64 = 0xF11;
pub const MARCHID: u64 = 0xF12;
pub const MIMPID: u64 = 0xF13;
pub const MHARTID: u64 = 0xF14;
pub const MSTATUS: u64 = 0x300;
pub const MISA: u64 = 0x301;
pub const MEDELEG: u64 = 0x302;
pub const MIDELEG: u64 = 0x303;
pub const MIE: u64 = 0x304;
pub const MTVEC: u64 = 0x305;
pub const MCOUNTEREN: u64 = 0x306;
pub const MTVT: u64 = 0x307;
pub const MUCOUNTEREN: u64 = 0x320;
pub const MSCOUNTEREN: u64 = 0x321;
pub const MSCRATCH: u64 = 0x340;
pub const MEPC: u64 = 0x341;
pub const MCAUSE: u64 = 0x342;
pub const MBADADDR: u64 = 0x343;
pub const MTVAL: u64 = 0x343;
pub const MIP: u64 = 0x344;
pub const MNXTI: u64 = 0x345;
pub const MINTSTATUS: u64 = 0x346;
pub const MSCRATCHCSW: u64 = 0x348;
pub const SSTATUS: u64 = 0x100;
pub const SEDELEG: u64 = 0x102;
pub const SIDELEG: u64 = 0x103;
pub const SIE: u64 = 0x104;
pub const STVEC: u64 = 0x105;
pub const SCOUNTEREN: u64 = 0x106;
pub const STVT: u64 = 0x107;
pub const SSCRATCH: u64 = 0x140;
pub const SEPC: u64 = 0x141;
pub const SCAUSE: u64 = 0x142;
pub const SBADADDR: u64 = 0x143;
pub const STVAL: u64 = 0x143;
pub const SIP: u64 = 0x144;
pub const SNXTI: u64 = 0x145;
pub const SINTSTATUS: u64 = 0x146;
pub const SSCRATCHCSW: u64 = 0x148;
pub const SPTBR: u64 = 0x180;
pub const SATP: u64 = 0x180;
pub const PMPCFG0: u64 = 0x3A0;
pub const PMPCFG1: u64 = 0x3A1;
pub const PMPCFG2: u64 = 0x3A2;
pub const PMPCFG3: u64 = 0x3A3;
pub const PMPADDR0: u64 = 0x3B0;
pub const PMPADDR1: u64 = 0x3B1;
pub const PMPADDR2: u64 = 0x3B2;
pub const PMPADDR3: u64 = 0x3B3;
pub const PMPADDR4: u64 = 0x3B4;
pub const PMPADDR5: u64 = 0x3B5;
pub const PMPADDR6: u64 = 0x3B6;
pub const PMPADDR7: u64 = 0x3B7;
pub const PMPADDR8: u64 = 0x3B8;
pub const PMPADDR9: u64 = 0x3B9;
pub const PMPADDR10: u64 = 0x3BA;
pub const PMPADDR11: u64 = 0x3BB;
pub const PMPADDR12: u64 = 0x3BC;
pub const PMPADDR13: u64 = 0x3BD;
pub const PMPADDR14: u64 = 0x3BE;
pub const PMPADDR15: u64 = 0x3BF;
pub const TSELECT: u64 = 0x7A0;
pub const TDATA1: u64 = 0x7A1;
pub const TDATA2: u64 = 0x7A2;
pub const TDATA3: u64 = 0x7A3;
pub const DCSR: u64 = 0x7B0;
pub const DPC: u64 = 0x7B1;
pub const DSCRATCH: u64 = 0x7B2;
pub const MHPMCOUNTER3: u64 = 0xB03;
pub const MHPMCOUNTER4: u64 = 0xB04;
pub const MHPMCOUNTER5: u64 = 0xB05;
pub const MHPMCOUNTER6: u64 = 0xB06;
pub const MHPMCOUNTER7: u64 = 0xB07;
pub const MHPMCOUNTER8: u64 = 0xB08;
pub const MHPMCOUNTER9: u64 = 0xB09;
pub const MHPMCOUNTER10: u64 = 0xB0A;
pub const MHPMCOUNTER11: u64 = 0xB0B;
pub const MHPMCOUNTER12: u64 = 0xB0C;
pub const MHPMCOUNTER13: u64 = 0xB0D;
pub const MHPMCOUNTER14: u64 = 0xB0E;
pub const MHPMCOUNTER15: u64 = 0xB0F;
pub const MHPMCOUNTER16: u64 = 0xB10;
pub const MHPMCOUNTER17: u64 = 0xB11;
pub const MHPMCOUNTER18: u64 = 0xB12;
pub const MHPMCOUNTER19: u64 = 0xB13;
pub const MHPMCOUNTER20: u64 = 0xB14;
pub const MHPMCOUNTER21: u64 = 0xB15;
pub const MHPMCOUNTER22: u64 = 0xB16;
pub const MHPMCOUNTER23: u64 = 0xB17;
pub const MHPMCOUNTER24: u64 = 0xB18;
pub const MHPMCOUNTER25: u64 = 0xB19;
pub const MHPMCOUNTER26: u64 = 0xB1A;
pub const MHPMCOUNTER27: u64 = 0xB1B;
pub const MHPMCOUNTER28: u64 = 0xB1C;
pub const MHPMCOUNTER29: u64 = 0xB1D;
pub const MHPMCOUNTER30: u64 = 0xB1E;
pub const MHPMCOUNTER31: u64 = 0xB1F;
pub const MHPMEVENT3: u64 = 0x323;
pub const MHPMEVENT4: u64 = 0x324;
pub const MHPMEVENT5: u64 = 0x325;
pub const MHPMEVENT6: u64 = 0x326;
pub const MHPMEVENT7: u64 = 0x327;
pub const MHPMEVENT8: u64 = 0x328;
pub const MHPMEVENT9: u64 = 0x329;
pub const MHPMEVENT10: u64 = 0x32A;
pub const MHPMEVENT11: u64 = 0x32B;
pub const MHPMEVENT12: u64 = 0x32C;
pub const MHPMEVENT13: u64 = 0x32D;
pub const MHPMEVENT14: u64 = 0x32E;
pub const MHPMEVENT15: u64 = 0x32F;
pub const MHPMEVENT16: u64 = 0x330;
pub const MHPMEVENT17: u64 = 0x331;
pub const MHPMEVENT18: u64 = 0x332;
pub const MHPMEVENT19: u64 = 0x333;
pub const MHPMEVENT20: u64 = 0x334;
pub const MHPMEVENT21: u64 = 0x335;
pub const MHPMEVENT22: u64 = 0x336;
pub const MHPMEVENT23: u64 = 0x337;
pub const MHPMEVENT24: u64 = 0x338;
pub const MHPMEVENT25: u64 = 0x339;
pub const MHPMEVENT26: u64 = 0x33A;
pub const MHPMEVENT27: u64 = 0x33B;
pub const MHPMEVENT28: u64 = 0x33C;
pub const MHPMEVENT29: u64 = 0x33D;
pub const MHPMEVENT30: u64 = 0x33E;
pub const MHPMEVENT31: u64 = 0x33F;
pub const MHPMCOUNTER3H: u64 = 0xB83;
pub const MHPMCOUNTER4H: u64 = 0xB84;
pub const MHPMCOUNTER5H: u64 = 0xB85;
pub const MHPMCOUNTER6H: u64 = 0xB86;
pub const MHPMCOUNTER7H: u64 = 0xB87;
pub const MHPMCOUNTER8H: u64 = 0xB88;
pub const MHPMCOUNTER9H: u64 = 0xB89;
pub const MHPMCOUNTER10H: u64 = 0xB8A;
pub const MHPMCOUNTER11H: u64 = 0xB8B;
pub const MHPMCOUNTER12H: u64 = 0xB8C;
pub const MHPMCOUNTER13H: u64 = 0xB8D;
pub const MHPMCOUNTER14H: u64 = 0xB8E;
pub const MHPMCOUNTER15H: u64 = 0xB8F;
pub const MHPMCOUNTER16H: u64 = 0xB90;
pub const MHPMCOUNTER17H: u64 = 0xB91;
pub const MHPMCOUNTER18H: u64 = 0xB92;
pub const MHPMCOUNTER19H: u64 = 0xB93;
pub const MHPMCOUNTER20H: u64 = 0xB94;
pub const MHPMCOUNTER21H: u64 = 0xB95;
pub const MHPMCOUNTER22H: u64 = 0xB96;
pub const MHPMCOUNTER23H: u64 = 0xB97;
pub const MHPMCOUNTER24H: u64 = 0xB98;
pub const MHPMCOUNTER25H: u64 = 0xB99;
pub const MHPMCOUNTER26H: u64 = 0xB9A;
pub const MHPMCOUNTER27H: u64 = 0xB9B;
pub const MHPMCOUNTER28H: u64 = 0xB9C;
pub const MHPMCOUNTER29H: u64 = 0xB9D;
pub const MHPMCOUNTER30H: u64 = 0xB9E;
pub const MHPMCOUNTER31H: u64 = 0xB9F;

/* HU-EXTENSION CSRS */
pub const VTIMECMP: u64 = 0x401;
pub const VTIMECTL: u64 = 0x402;
pub const VTIMECMPH: u64 = 0x481;

pub const HUVSSTATUS: u64 = 0x400;
pub const HUVSIP: u64 = 0x444;
pub const HUVSIE: u64 = 0x404;
pub const HUVSEPC: u64 = 0x441;

/* TODO: Remove in the future for new hardware implementation */
pub const VCPUID: u64 = 0x482;
pub const VIPI0: u64 = 0x483;
pub const VIPI1: u64 = 0x484;
pub const VIPI2: u64 = 0x485;
pub const VIPI3: u64 = 0x486;

pub const HSTATUS: u64 = 0x600;
pub const HEDELEG: u64 = 0x602;
pub const HIDELEG: u64 = 0x603;
pub const HIE: u64 = 0x604;
pub const HCOUNTEREN: u64 = 0x606;
pub const HGEIE: u64 = 0x607;
pub const HTVAL: u64 = 0x643;
pub const HVIP: u64 = 0x645;
pub const HIP: u64 = 0x644;
pub const HTINST: u64 = 0x64A;
pub const HGEIP: u64 = 0xE12;
pub const HGATP: u64 = 0x680;
pub const HTIMEDELTA: u64 = 0x605;
pub const HTIMEDELTAH: u64 = 0x615;

pub const HUSTATUS: u64 = 0x800;
pub const HUEDELEG: u64 = 0x802;
pub const HUIDELEG: u64 = 0x803;
pub const HUIE: u64 = 0x804;
pub const HUCOUNTEREN: u64 = 0x806;
pub const HUGEIE: u64 = 0x807;
pub const HUTVAL: u64 = 0x843;
pub const HUVIP: u64 = 0x845;
pub const HUIP: u64 = 0x844;
pub const HUTINST: u64 = 0x84A;
pub const HUGEIP: u64 = 0x812;
pub const HUGATP: u64 = 0x880;
pub const HUTIMEDELTA: u64 = 0x805;
pub const HUTIMEDELTAH: u64 = 0x815;

pub const HUCOUNTEREN_CY: u64 = (1 << 0);
pub const HUCOUNTEREN_TM: u64 = (1 << 1);
pub const HUCOUNTEREN_IR: u64 = (1 << 2);
pub const HUCOUNTEREN_HPM3: u64 = (1 << 3);

/* Atomic read from CSR */
#[macro_export]
macro_rules! csrr {
    ( $r:ident ) => {{
        let value: u64;
        /* asm!("csrr {rd}, {csr}", rd = out(reg) value, csr = const $r); */
        match $r {
            VCPUID => asm!("csrr {rd}, 0x482", rd = out(reg) value),
            TIME => asm!("csrr {rd}, 0xc01", rd = out(reg) value),
            HUVIP => asm!("csrr {rd}, 0x845", rd = out(reg) value),
            MIP => asm!("csrr {rd}, 0x344", rd = out(reg) value),
            HUVSIE => asm!("csrr {rd}, 0x404", rd = out(reg) value),
            HUIE => asm!("csrr {rd}, 0x804", rd = out(reg) value),
            _ => {
                value = 0x3f3f3f3f3f3f3f3f;
                println!("csrr unknown csr {}", $r);
            }
        }
        value
    }};
}

/* Atomic write to CSR */
#[macro_export]
macro_rules! csrw {
    ( $r:ident, $x:expr ) => {{
        let x: u64 = $x;
        /* asm!("csrw {csr}, {rs}", rs = in(reg) x, csr = const $r); */
        match $r {
            VCPUID => asm!("csrw 0x482, {rs}", rs = in(reg) x),
            VTIMECTL => asm!("csrw 0x402, {rs}", rs = in(reg) x),
            VTIMECMP => asm!("csrw 0x401, {rs}", rs = in(reg) x),
            HUGATP => asm!("csrw 0x880, {rs}", rs = in(reg) x),
            HUIE => asm!("csrw 0x804, {rs}", rs = in(reg) x),
            HUVSIE => asm!("csrw 0x404, {rs}", rs = in(reg) x),
            UTVEC => asm!("csrw 0x005, {rs}", rs = in(reg) x),
            HUTIMEDELTA => asm!("csrw 0x805, {rs}", rs = in(reg) x),
            _ => println!("csrw unknown csr {}", $r)
        }
    }};
}

/* Atomic write to CSR from immediate */
#[macro_export]
macro_rules! csrwi {
    ( $r:ident, $x:expr ) => {{
        const X: u64 = $x;
        /* asm!("csrwi {csr}, {rs}", rs = in(reg) X, csr = const $r); */
        match $r {
            _ => println!("csrwi unknown csr {}", $r);
        }
    }};
}

/* Atomic read and set bits in CSR */
#[macro_export]
macro_rules! csrs {
    ( $r:ident, $x:expr ) => {{
        let x: u64 = $x;
        /* asm!("csrs {csr}, {rs}", rs = in(reg) x, csr = const $r); */
        match $r {
            VIPI0 => asm!("csrs 0x483, {rs}", rs = in(reg) x),
            VIPI1 => asm!("csrs 0x484, {rs}", rs = in(reg) x),
            VIPI2 => asm!("csrs 0x485, {rs}", rs = in(reg) x),
            VIPI3 => asm!("csrs 0x486, {rs}", rs = in(reg) x),
            HUVIP => asm!("csrs 0x845, {rs}", rs = in(reg) x),
            _ => println!("csrs unknown csr {}", $r)
        }
    }};
}

/* Atomic read and set bits in CSR using immediate */
#[macro_export]
macro_rules! csrsi {
    ( $r:ident, $x:expr ) => {{
        const X: u64 = $x;
        /* asm!("csrsi {csr}, {rs}", rs = in(reg) X, csr = const $r); */
        match $r {
            _ => println!("csrsi unknown csr {}", $r);
        }
    }};
}

/* Atomic read and clear bits in CSR */
#[macro_export]
macro_rules! csrc {
    ( $r:ident, $x:expr ) => {{
        let x: u64 = $x;
        /* asm!("csrc {csr}, {rs}", rs = in(reg) x, csr = const $r); */
        match $r {
            VIPI0 => asm!("csrc 0x483, {rs}", rs = in(reg) x),
            VIPI1 => asm!("csrc 0x484, {rs}", rs = in(reg) x),
            VIPI2 => asm!("csrc 0x485, {rs}", rs = in(reg) x),
            VIPI3 => asm!("csrc 0x486, {rs}", rs = in(reg) x),
            HUVIP => asm!("csrc 0x845, {rs}", rs = in(reg) x),
            HUIP => asm!("csrc 0x844, {rs}", rs = in(reg) x),
            VTIMECTL => asm!("csrc 0x402, {rs}", rs = in(reg) x),
            _ => println!("csrc unknown csr {}", $r)
        }
    }};
}

/* Atomic read and clear bits in CSR using immediate */
#[macro_export]
macro_rules! csrci {
    ( $r:ident, $x:expr ) => {{
        const X: u64 = $x;
        /* asm!("csrci {csr}, {rs}", rs = in(reg) X, csr = const $r); */
        match $r {
            _ => println!("csrci unknown csr {}", $r);
        }
    }};
}

#[cfg(feature = "cve")]
pub unsafe fn inject_use_after_free() {
    unsafe {
        /* (*a) = b, *(*a) = (*b) = c */
        let c: u64 = 1;
        let mut b = &c as *const u64 as u64;
        let a = &b as *const u64 as u64;
        assert_eq!(*(*(a as *mut u64) as *mut u64), c);

        /* Free b */
        b = 0;
        drop(b);

        /* Access c from a, should panic */
        assert_eq!(*(*(a as *mut u64) as *mut u64), c);

        /* panic!("Emulating use-after-free (unexpected)!"); */
    }
}

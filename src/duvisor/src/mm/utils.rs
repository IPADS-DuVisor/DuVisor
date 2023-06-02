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

pub const PAGE_SIZE_SHIFT: u64 = 12;
pub const PAGE_TABLE_REGION_SIZE: u64 = 32 << MB_SHIFT; /* 32MB for now */
pub const PAGE_SIZE: u64 = 1u64 << PAGE_SIZE_SHIFT;
pub const PAGE_SIZE_MASK: u64 = PAGE_SIZE - 1;
pub const PAGE_SHIFT: u64 = 12;
pub const PAGE_ORDER: u64 = 9;
pub const KB_SHIFT: u64 = 10;
pub const MB_SHIFT: u64 = 2 * KB_SHIFT;
#[allow(unused)]
pub const GB_SHIFT: u64 = 3 * KB_SHIFT;
#[allow(unused)]
pub const TB_SHIFT: u64 = 4 * KB_SHIFT;

#[macro_export]
macro_rules! dbgprintln {
    () => {
        #[cfg(test)]
        print!("\n");
    };
    ($fmt: expr) => {
        #[cfg(test)]
        print!(concat!($fmt, "\n"));
    };
    ($fmt: expr, $($arg:tt)*) => {
        #[cfg(test)]
        print!(concat!($fmt, "\n"), $($arg)*);
    };
}

#[macro_export]
macro_rules! print_flush {
    ( $($t:tt)* ) => {
        {
            let mut h = io::stdout();
            write!(h, $($t)* ).unwrap();
            h.flush().unwrap();
        }
    }
}

pub fn page_size_round_up(length: u64) -> u64 {
    if length & PAGE_SIZE_MASK == 0 {
        return length;
    }

    let result: u64 = (length & !PAGE_SIZE_MASK) + PAGE_SIZE;

    result
}

pub fn va_to_hpa_helper(va_base: u64, hpa_base: u64, va: u64, length: u64) -> Option<u64> {
    if va < va_base {
        return None;
    }

    let offset: u64 = va - va_base;

    if offset >= length {
        return None;
    }

    Some(offset + hpa_base)
}

pub fn hpa_to_va_helper(va_base: u64, hpa_base: u64, hpa: u64, length: u64) -> Option<u64> {
    if hpa < hpa_base {
        return None;
    }

    let offset = hpa - hpa_base;

    if offset >= length {
        return None;
    }

    Some(offset + va_base)
}

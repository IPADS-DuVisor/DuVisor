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

/* HU CSR */

#[allow(unused)]
pub mod csr_constants {
    /* HUSTATUS */
    pub const HUSTATUS_UIE_SHIFT: u64 = 0;
    pub const HUSTATUS_UPIE_SHIFT: u64 = 4;
    pub const HUSTATUS_SPV_SHIFT: u64 = 7;
    pub const HUSTATUS_SPVP_SHIFT: u64 = 8;
    pub const HUSTATUS_VTW_SHIFT: u64 = 21;

    pub const VTIMECTL_ENABLE: u64 = 0;

    pub const HUGATP_MODE_SHIFT: u64 = 60;
    pub const HUGATP_MODE_SV39: u64 = 8 << HUGATP_MODE_SHIFT;
    pub const HUGATP_MODE_SV48: u64 = 9 << HUGATP_MODE_SHIFT;
}

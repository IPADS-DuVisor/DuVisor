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

pub struct GpaRegion {
    gpa: u64,
    length: u64,
}

impl GpaRegion {
    pub fn new(gpa: u64, length: u64) -> Self {
        Self { gpa, length }
    }

    pub fn get_gpa(&self) -> u64 {
        self.gpa
    }

    pub fn get_length(&self) -> u64 {
        self.length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpa_region_new() {
        let gpa = 0x8000;
        let length = 0x2000;
        let gpa_region = GpaRegion::new(gpa, length);

        assert_eq!(gpa_region.gpa, gpa);
        assert_eq!(gpa_region.length, length);
    }
}

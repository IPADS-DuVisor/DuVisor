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

use colored::*;
use duvisor::init::cmdline;
use std::process;

fn main() {
    let vm_config = cmdline::VMConfig::new().unwrap_or_else(|err| {
        eprintln!("{}: {}", "error".bright_red(), err);
        process::exit(1);
    });

    if !cmdline::VMConfig::verify_args(&vm_config) {
        process::exit(1);
    }

    let hello_str = r#"
                                                      
    ,------.         ,--.   ,--.,--.                      
    |  .-.  \ ,--.,--.\  `.'  / `--' ,---.  ,---. ,--.--. 
    |  |  \  :|  ||  | \     /  ,--.(  .-' | .-. ||  .--' 
    |  '--'  /'  ''  '  \   /   |  |.-'  `)' '-' '|  |    
    `-------'  `----'    `-'    `--'`----'  `---' `--'    
                                                           
    "#;
    println!("{}", hello_str);

    duvisor::run(vm_config);
}

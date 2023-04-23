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

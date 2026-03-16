use std::io::{self, Write};
use std::process::Command;
use std::fs::File;
use simplelog::CombinedLogger;
use log::{LevelFilter, info};
use simplelog::WriteLogger;
use simplelog::*;
use sysinfo::System;


fn service() {
  println!("test");
}

fn logging() {
    

    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        WriteLogger::new(LevelFilter::Info, Config::default(), File::create("system.txt").unwrap()),

    ]).unwrap();

    log::info!("System check started...");
    
    let mut sys = System::new_all();
    sys.refresh_all();

    if let Some(k_ver) = System::kernel_version() {
       info!("Kenrnel Version: {}", k_ver);
    }
    info!("Total Memory: {} KB", sys.total_memory());
    info!("Active Processes: {}", sys.processes().len());

    info!("System check completed.");
}


fn ui() {
     // This override prevents the 0xc000013a crash
    ctrlc::set_handler(move || {
        println!("\n[Signal] Ctrl+C detected! Exiting cleanly...");
        std::process::exit(0); 
    }).expect("Error setting Ctrl-C handler");


    loop {
        print!("1. DDoS\n2. Info\n3. System Info\n4. Exit\n--> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }
        match input.trim() {
            "1" => {
                let output = Command::new("python")
                    .arg("modules.py")
                    .output()
                    .expect("Failed to run");

                if output.status.success() {
                    println!("Is working");
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                } else {
                    eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
                }
            }
            "2" => {
                service();
            }
            "3" => {
              logging();
            }
            
            "4" => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid option, please try again.");
            }
        }   
    }
}

fn main() {
    ui();
}
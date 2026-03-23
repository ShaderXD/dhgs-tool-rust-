use log::{LevelFilter, info};
use simplelog::CombinedLogger;
use simplelog::WriteLogger;
use simplelog::*;
use std::fs::File;
use std::io::{self, Write, stdin, stdout};
use std::path::Path;
use std::process::Command;
use sysinfo::System;

fn yes_no_prompt(prompt_message: &str) -> bool {
    loop {
        println!("{} (y/n): ", prompt_message);
        let _ = stdout().flush();
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim_end().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("Invalid input. Please enter 'y' or 'n'."),
        }
    }
}

fn service() {
    let result = ctrlc::try_set_handler(move || {
        println!("\n[Signal] Ctrl+C detected! Exiting cleanly...");
        std::process::exit(0);
    });

    // This override prevents the 0xc000013a crash
    if let Err(ctrlc::Error::MultipleHandlers) = result {
    } else {
        result.expect("Error setting Ctrl-C handler");
    }

    loop {
        print!("1. Git\n2. AUR\n3. HTTPS\n4. Exit\n--> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }
        match input.trim() {
            "1" => {
                let mut choice = String::new();

                println!("Please Enter: the username/reponame like this");
                io::stdin()
                    .read_line(&mut choice)
                    .expect("Failed to read line");

                let status = Command::new("git")
                    .arg("clone")
                    .arg(format!("https://github.com/{}", choice.trim()))
                    .status()
                    .expect("Failed to execute git clone");
                if status.success() {
                    println!("Clone successful!");

                    let folder_name = choice.split('/').last().unwrap_or("");

                    if yes_no_prompt("There is CMAKE build file found: try build? ") {
                        let build_path = std::path::Path::new(folder_name).join("build");

                        std::fs::create_dir_all(&build_path)
                            .expect("Failed to create build directory");

                        let configure_result = Command::new("cmake")
                            .arg("..")
                            .current_dir(&build_path)
                            .status();          

                        match configure_result {
                            Ok(status) if status.success() => {
                                println!("Build system generated. Starting compilation...");
                                let _ = Command::new("cmake")
                                    .args(["--build", "."])
                                    .current_dir(&build_path)
                                    .status();
                            }
                            Ok(_status) => {
                                eprintln!("CMake configuration failed (check your CMakeLists.txt).")
                            }
                            Err(e) => {
                              eprintln!("Could not find 'cmake' executable: {}", e);
                        }
                      } 
                    } else {
                        println!("Exiting");
                        break;
                    }
                } else {
                    eprintln!("Clone failed.");
                }
            }
            "2" => {
                if cfg!(target_os = "windows") {
                    let mut choice = String::new();

                    println!("Please Enter: the reponame like this");
                    io::stdin()
                        .read_line(&mut choice)
                        .expect("Failed to read line");

                    let status = Command::new("git")
                        .arg("clone")
                        .arg(format!("https://aur.archlinux.org/{}", choice.trim()))
                        .status()
                        .expect("Failed to execute git clone");
                    if status.success() {
                        println!("Clone successful!");
                        let clean_choice =
                            choice.trim().strip_suffix(".git").unwrap_or(choice.trim());
                        let path = Path::new(clean_choice).join("PKGBUILD");
                        if path.exists() {
                            if yes_no_prompt("There is PKGBUILD build file found: try build? ") {
                                let _status = Command::new("makepkg")
                                    .arg("-si")
                                    .status()
                                    .expect("Failed to execute building");
                            } else {
                                println!("Exiting");
                            }
                        } else {
                            println!("There no PKGBUILD file found");
                        }
                    } else {
                        eprintln!("Clone failed.");
                    }
                } else {
                    println!("Your not on [LINUX] Error:....")
                }
            }
            "3" => {
                let mut choice = String::new();

                println!("Please Enter: the HTTPS like this");
                io::stdin()
                    .read_line(&mut choice)
                    .expect("Failed to read line");

                let status = Command::new("git")
                    .arg("clone")
                    .arg(choice.trim())
                    .status()
                    .expect("Failed to execute git clone");
                if status.success() {
                    println!("Clone successful!");
                } else {
                    eprintln!("Clone failed.");
                }
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

fn logging() {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Warn,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("system.txt").unwrap(),
        ),
    ])
    .unwrap();

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
    })
    .expect("Error setting Ctrl-C handler");

    loop {
        print!("1. DDoS\n2. Git\n3. System Info\n4. Exit\n--> ");
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

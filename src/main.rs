use log::error;
use log::{LevelFilter, info};
use simplelog::CombinedLogger;
use simplelog::WriteLogger;
use simplelog::*;
use std::fs::File;
use std::io::{self, Write, stdin, stdout};
use std::path::Path;
use std::process::Command;
use sysinfo::System;

struct Logger(CombinedLogger); // this struct before i didn't waht struct was but i'm head just hurt
  impl Logger {
    fn log_init() { // 1 to 2 hors to find out i knew i was bad but damn i'm that bad just sad
      let _ = CombinedLogger::init(vec![ 
        TermLogger::new(
            LevelFilter::Warn, // Log to terminal with warning level and above
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ), // Log to terminal with warning level and above
        WriteLogger::new(
            LevelFilter::Info, // Log to file with info level and above
            Config::default(),
            File::create("system.txt").unwrap(),
        ),
       ]
     );   
     info!("Logger initialized and running!"); // Log an info message to confirm initialization
  }
}


    

fn logging() { // Initialize logging to both terminal and file
    Logger::log_init(); // Initialize the logger
    log::info!("System check started...");

    let mut sys = System::new_all();
    sys.refresh_all();
    // Log some basic system information
    if let Some(k_ver) = System::kernel_version() {
        info!("Kenrnel Version: {}", k_ver);
    }
    info!("Total Memory: {} KB", sys.total_memory());
    info!("Active Processes: {}", sys.processes().len());

    info!("System check completed.");
}

fn yes_no_prompt(prompt_message: &str) -> bool {
    loop {
        println!("{} (y/n): ", prompt_message); // Print the prompt message
        let _ = stdout().flush();
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line"); // Read user input

        match input.trim_end().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("Invalid input. Please enter 'y' or 'n'."),
        }
    }
}

fn service() { // This function handles the Git and AUR cloning and building logic
    Logger::log_init(); // Initialize the logger
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
            "1" => { // GIT REPO CLONE AND BUILD BUT NOT WORKING AT THE MOMENT
                let mut choice = String::new();

                println!("Please Enter: the username/reponame like this");
                io::stdin()
                    .read_line(&mut choice)
                    .expect("Failed to read line");
                // git repo clone -- cmake build system is not working yet, but it will be added in the future
                let status = Command::new("git")
                    .arg("clone")
                    .arg(format!("https://github.com/{}", choice.trim()))
                    .status()
                    .expect("Failed to execute git clone");
                if status.success() {
                    println!("Clone successful!");
                    info!("Successful clone repo: (Github)");
                    
                 
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
                    error!("Failed clone repo: (Github)");
                }
            }
            "2" => { // AUR REPO CLONE AND BUILD 
                // run a see your on linux if not it will show error message, because this is only for linux, but it will be added in the future for windows too, but for now it will be only for linux
                if cfg!(target_os = "windows") {
                    let mut choice = String::new();

                    println!("Please Enter: the reponame like this");
                    io::stdin()
                        .read_line(&mut choice)
                        .expect("Failed to read line");
                    // git AUR repo and automatically build it with makepkg -si, but only for linux, because windows don't have makepkg
                    let status = Command::new("git")
                        .arg("clone")
                        .arg(format!("https://aur.archlinux.org/{}", choice.trim()))
                        .status()
                        .expect("Failed to execute git clone");
                    if status.success() {
                        println!("Clone successful!");
                        info!("Successful clone repo: (AUR)");
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
                            info!("No PKGBUILD file found")
                        }
                    } else {
                        eprintln!("Clone failed.");
                        error!("Failed clone repo: (AUR)");
                    }
                } else {
                    println!("Your not on [LINUX] Error:....");
                    error!("Your not on [linux] can't build");
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

fn ui() { // This function handles the main user interface and menu logic
    Logger::log_init();
    // This override prevents the 0xc000013a crash
    ctrlc::set_handler(move || {
        println!("\n[Signal] Ctrl+C detected! Exiting cleanly...");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    loop { // Main menu loop
        print!("1. DDoS\n2. Git\n3. System Info\n4. Exit\n--> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }
        match input.trim() {
            "1" => { // OLD NOT BEEN WORK ON 
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
            "2" => { // GIT AND AUR CLONE AND BUILD SERVICE
                service();
            }
            "3" => { // SYSTEM INFO LOGGING
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

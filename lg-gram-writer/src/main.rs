use std::env;
use std::process;
use std::fs;
use std::path::Path;

use nix;

fn main() {
    // Exit if not running as root
    if !nix::unistd::geteuid().is_root() {
        eprintln!("ERROR: App must be run as root");
        process::exit(1);
    }

    // Validate args
    let args: Vec<String> = env::args().collect();

    let Ok((mode, setting, value)) = validate_args(&args) else {
        eprint_usage(&args[0]);
        process::exit(1);
    };

    // Check mode
    let result = match mode.as_str() {
        "--kernel" => set_kernel_feature(&setting, &value),
        _ => unreachable!()
    };

    // Exit if error
    if let Err(error) = result {
        eprintln!("{error}");
        process::exit(1);
    }

    println!("Successfully changed {} setting", setting);
}

fn set_kernel_feature(setting: &str, value: &str) -> Result<(), String> {
    // Check if settings file exists
    let file = format!("/sys/devices/platform/lg-laptop/{}", setting);

    fs::metadata(&file)
        .map_err(|_| String::from("ERROR: Settings file does not exist"))?;

    // Write to settings file
    let content = format!("{}\n", value);

    fs::write(file, content)
        .map_err(|_| String::from("ERROR: Error writing to settings file"))
}

fn validate_args(args: &[String]) -> Result<(String, String, String), ()> {
    if args.len() != 3 {
        return Err(());
    }

    let mode = args[1].clone();

    if mode != "--kernel" && mode != "--service" {
        return Err(());
    }

    let Some((setting, value)) = args[2].split_once("=") else {
        return Err(());
    };

    match (setting, value) {
        ("battery_care_limit", value) if ["80", "100"].contains(&value) => {
            Ok((mode, String::from(setting), String::from(value)))
        },
        ("usb_charge", value) if ["0", "1"].contains(&value) => {
            Ok((mode, String::from(setting), String::from(value)))
        },
        ("reader_mode", value) if ["0", "1"].contains(&value) => {
            Ok((mode, String::from(setting), String::from(value)))
        },
        ("fn_lock", value) if ["0", "1"].contains(&value) => {
            Ok((mode, String::from(setting), String::from(value)))
        },
        _ => {
            Err(())
        }
    }
}

fn eprint_usage(app_path: &str) {
    let app_name = Path::new(app_path)
        .file_name()
        .unwrap_or_default()
        .to_string_lossy();

    eprintln!("ERROR: USAGE: {app_name} mode setting=value");
}

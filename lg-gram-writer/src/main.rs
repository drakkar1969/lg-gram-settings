use std::env;
use std::process;
use std::fs;

use nix;

fn main() {
    // Exit if not running as root
    if !nix::unistd::geteuid().is_root() {
        eprintln!("ERROR: App must be run as root");
        process::exit(1);
    }

    // Validate args
    let args: Vec<String> = env::args().collect();

    let Ok((setting, value)) = validate_args(&args) else {
        eprint_usage(&args[0]);
        process::exit(1);
    };

    // Check if settings file exists
    let file = format!("/sys/devices/platform/lg-laptop/{}", setting);

    if fs::metadata(&file).is_err() {
        eprintln!("ERROR: Settings file does not exist");
        process::exit(1);
    }

    // Write to settings file
    let content = format!("{}\n", value);

    if fs::write(file, content).is_err() {
        eprintln!("ERROR: Error writing to settings file");
        process::exit(1);
    }

    println!("Successfully changed {} setting", setting);
}

fn validate_args(args: &[String]) -> Result<(String, String), ()> {
    if args.len() != 2 {
        return Err(());
    }

    let Some((setting, value)) = args[1].split_once("=") else {
        return Err(());
    };

    match (setting, value) {
        ("battery_care_limit", value) if ["80", "100"].contains(&value) => {
            Ok((String::from(setting), String::from(value)))
        },
        ("usb_charge", value) if ["0", "1"].contains(&value) => {
            Ok((String::from(setting), String::from(value)))
        },
        ("reader_mode", value) if ["0", "1"].contains(&value) => {
            Ok((String::from(setting), String::from(value)))
        },
        ("fn_lock", value) if ["0", "1"].contains(&value) => {
            Ok((String::from(setting), String::from(value)))
        },
        _ => {
            Err(())
        }
    }
}

fn eprint_usage(app: &str) {
    eprintln!(
        "\n\
        USAGE:\n\
        \n\
        {} <setting>=<value>\n\
        \n\
        where <setting> is one of:\n\
        \n\
        battery_care_limit: <value>=80/100\n\
        usb_charge: <value>=0/1\n\
        reader_mode: <value>=0/1\n\
        fn_lock: <value>=0/1\n",
        app
    );
}

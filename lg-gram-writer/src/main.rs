use std::env;
use std::process;
use std::fs;
use std::path::Path;

//------------------------------------------------------------------------------
// APP: main
//------------------------------------------------------------------------------
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
        "--system-info" => system_information(),
        "--feature" => set_feature(&setting, &value),
        "--service" => enable_service(&setting, &value),
        _ => unreachable!()
    };

    // Exit if error
    match result {
        Ok(msg) => {
            println!("{msg}");
        },
        Err(error) => {
            eprintln!("{error}");
            process::exit(1);
        }
    }
}

//---------------------------------------
// System information function
//---------------------------------------
fn system_information() -> Result<String, String> {
    let dmidecode = |string: &str| -> Result<String, String> {
        let output = process::Command::new("dmidecode")
            .arg("-s")
            .arg(string)
            .output()
            .map_err(|error| error.to_string())?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).into())
        }

        Ok(String::from_utf8_lossy(&output.stdout).into())
    };

    let manufacturer = dmidecode("system-manufacturer")?;
    let product_name = dmidecode("system-product-name")?;
    let serial_number = dmidecode("system-serial-number")?;

    let output = [
        String::from("Manufacturer\n"),
        manufacturer,
        String::from("Product Name\n"),
        product_name,
        String::from("Serial Number\n"),
        serial_number
    ].join("");

    Ok(output)
}

//---------------------------------------
// Set feature function
//---------------------------------------
fn set_feature(setting: &str, value: &str) -> Result<String, String> {
    // Check if settings file exists
    let file = format!("/sys/devices/platform/lg-laptop/{setting}");

    fs::metadata(&file)
        .map_err(|_| String::from("ERROR: Settings file does not exist"))?;

    // Write to settings file
    let content = format!("{value}\n");

    fs::write(file, content)
        .map_err(|_| String::from("ERROR: Error writing to settings file"))?;

    Ok(format!("Successfully changed {setting} setting"))
}

//---------------------------------------
// Enable service function
//---------------------------------------
fn enable_service(setting: &str, value: &str) -> Result<String, String> {
    // Check if service unit file exists
    let service_name = format!("lg-gram-{}.service", setting.replace('_', "-"));

    let unit_file = format!("/usr/lib/systemd/system/{service_name}");

    fs::metadata(&unit_file)
        .map_err(|_| String::from("ERROR: Service unit file does not exist"))?;

    // Enable/disable service
    let enable = if value == "0" { "disable" } else { "enable" };

    let output = process::Command::new("systemctl")
        .arg(enable)
        .arg(&service_name)
        .output()
        .map_err(|error| error.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into())
    }

    Ok(format!("Successfully {enable}d {service_name}"))
}

//---------------------------------------
// Validate args function
//---------------------------------------
fn validate_args(args: &[String]) -> Result<(String, String, String), ()> {
    let Some(mode) = args.get(1).cloned() else {
        return Err(());
    };

    match mode.as_str() {
        "--system-info" => { Ok((mode, String::new(), String::new())) } 
        "--feature" => {
            let Some((setting, value)) = args.get(2).and_then(|arg| arg.split_once('=')) else {
                return Err(());
            };

            match (setting, value) {
                ("battery_care_limit", value) if ["80", "100"].contains(&value) => {
                    Ok((mode, String::from(setting), String::from(value)))
                },
                ("fn_lock" | "usb_charge" | "reader_mode", value) if ["0", "1"].contains(&value) => {
                    Ok((mode, String::from(setting), String::from(value)))
                },
                _ => {
                    Err(())
                }
            }
        },
        "--service" => {
            let Some((setting, value)) = args.get(2).and_then(|arg| arg.split_once('=')) else {
                return Err(());
            };

            if ["battery_care_limit", "usb_charge", "reader_mode", "fn_lock"].contains(&setting) &&
                ["0", "1"].contains(&value)
            {
                Ok((mode, String::from(setting), String::from(value)))
            } else {
                Err(())
            }
        },
        _ => {
            Err(())
        }
    }
}

//---------------------------------------
// Eprint usage function
//---------------------------------------
fn eprint_usage(app_path: &str) {
    let app_name = Path::new(app_path)
        .file_name()
        .unwrap_or_default()
        .to_string_lossy();

    eprintln!("ERROR: USAGE: {app_name} mode setting=value");
}

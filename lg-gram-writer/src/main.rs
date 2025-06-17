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

    let Ok((mode, setting, value, enable)) = validate_args(&args) else {
        eprint_usage(&args[0]);
        process::exit(1);
    };

    // Check mode
    let result = match mode {
        "--system-info" => system_information(),
        "--feature" => set_feature(setting, value, enable),
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
    let dmi_read = |string: &str| -> Result<String, String> {
        let file = format!("/sys/devices/virtual/dmi/id/{string}");

        fs::read_to_string(file)
            .map_err(|error| error.to_string())
            .map(|value| value.trim().to_owned())
    };

    let product_name = dmi_read("product_name")?;
    let product_serial = dmi_read("product_serial")?;
    let bios_vendor = dmi_read("bios_vendor")?;
    let bios_version = dmi_read("bios_version")?;

    let output = [
        String::from("Product Name"),
        product_name,
        String::from("Serial Number"),
        product_serial,
        String::from("BIOS Vendor"),
        bios_vendor,
        String::from("BIOS Version"),
        bios_version,
    ].join("\n");

    Ok(output)
}

//---------------------------------------
// Set feature function
//---------------------------------------
fn set_feature(setting: &str, value: &str, enable: &str) -> Result<String, String> {
    // Check if settings file exists
    let settings_file = format!("/sys/devices/platform/lg-laptop/{setting}");

    fs::metadata(&settings_file)
        .map_err(|_| String::from("ERROR: Settings file does not exist"))?;

    // Check if service unit file exists
    let service_name = format!("lg-gram-{}.service", setting.replace('_', "-"));

    let unit_file = format!("/usr/lib/systemd/system/{service_name}");

    fs::metadata(&unit_file)
        .map_err(|_| String::from("ERROR: Service unit file does not exist"))?;

    // Write to settings file
    let content = format!("{value}\n");

    fs::write(settings_file, content)
        .map_err(|_| String::from("ERROR: Error writing to settings file"))?;

    // Enable/disable service
    let output = process::Command::new("systemctl")
        .arg(enable)
        .arg(&service_name)
        .output()
        .map_err(|error| error.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into())
    }

    Ok(format!("Successfully changed {setting} setting"))
}

//---------------------------------------
// Validate args function
//---------------------------------------
fn validate_args(args: &[String]) -> Result<(&str, &str, &str, &str), ()> {
    let Some(mode) = args.get(1) else {
        return Err(());
    };

    match mode.as_str() {
        "--system-info" => { Ok((mode, "", "", "")) } 
        "--feature" => {
            let Some((setting, value)) = args.get(2).and_then(|arg| arg.split_once('=')) else {
                return Err(());
            };

            match (setting, value) {
                ("battery_care_limit", value) if ["80", "100"].contains(&value) => {
                    let enable = if value == "100" { "disable" } else { "enable" };

                    Ok((mode, setting, value, enable))
                },
                ("fn_lock" | "usb_charge" | "reader_mode", value) if ["0", "1"].contains(&value) => {
                    let enable = if value == "0" { "disable" } else { "enable" };

                    Ok((mode, setting, value, enable))
                },
                _ => {
                    Err(())
                }
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

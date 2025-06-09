//------------------------------------------------------------------------------
// MODULE: KernelFeatures
//------------------------------------------------------------------------------
pub mod kernel_features {
    use std::fs;
    use std::process::{Command, ExitStatus};

    //---------------------------------------
    // Constants
    //---------------------------------------
    const SETTINGS_PATH: &str = "/sys/devices/platform/lg-laptop";

    const BATTERY_ID: &str = "battery_care_limit";
    const FNLOCK_ID: &str = "fn_lock";
    const READER_ID: &str = "reader_mode";
    const USB_ID: &str = "usb_charge";

    //---------------------------------------
    // Read/write helper functions
    //---------------------------------------
    fn parse_u32_from_file(file: &str) -> Result<u32, String> {
        fs::read_to_string(file)
            .map_err(|error| error.to_string())
            .and_then(|value| {
                value.trim().parse::<u32>()
                    .map_err(|error| error.to_string())
            })
    }

    fn write_u32_to_file(id: &str, value: u32) -> Result<ExitStatus, String> {
        let mut process = Command::new("pkexec")
            .arg("lg-gram-writer")
            .arg(format!("{id}={value}"))
            .spawn()
            .map_err(|error| error.to_string())?;

        process.wait()
            .map_err(|error| error.to_string())
    }

    //---------------------------------------
    // Battery limit functions
    //---------------------------------------
    pub fn battery_limit() -> Result<u32, String> {
        parse_u32_from_file(&format!("{}/{}", SETTINGS_PATH, BATTERY_ID))
    }

    pub fn set_battery_limit(value: u32) -> Result<std::process::ExitStatus, String> {
        write_u32_to_file(BATTERY_ID, value)
    }

    //---------------------------------------
    // USB charge functions
    //---------------------------------------
    pub fn usb_charge() -> Result<u32, String> {
        parse_u32_from_file(&format!("{}/{}", SETTINGS_PATH, USB_ID))
    }

    pub fn set_usb_charge(value: u32) -> Result<std::process::ExitStatus, String> {
        write_u32_to_file(USB_ID, value)
    }

    //---------------------------------------
    // Reader mode functions
    //---------------------------------------
    pub fn reader_mode() -> Result<u32, String> {
        parse_u32_from_file(&format!("{}/{}", SETTINGS_PATH, READER_ID))
    }

    pub fn set_reader_mode(value: u32) -> Result<std::process::ExitStatus, String> {
        write_u32_to_file(READER_ID, value)
    }

    //---------------------------------------
    // Fn lock functions
    //---------------------------------------
    pub fn fn_lock() -> Result<u32, String> {
        parse_u32_from_file(&format!("{}/{}", SETTINGS_PATH, FNLOCK_ID))
    }

    pub fn set_fn_lock(value: u32) -> Result<std::process::ExitStatus, String> {
        write_u32_to_file(FNLOCK_ID, value)
    }
}

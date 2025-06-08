//------------------------------------------------------------------------------
// MODULE: KernelFeatures
//------------------------------------------------------------------------------
pub mod kernel_features {
    use std::fs;
    use std::process::{Command, ExitStatus, Stdio};
    use std::io::Write;

    //---------------------------------------
    // Constants
    //---------------------------------------
    const BATTERY_PATH: &str = "/sys/devices/platform/lg-laptop/battery_care_limit";
    const FNLOCK_PATH: &str = "/sys/devices/platform/lg-laptop/fn_lock";
    const READER_PATH: &str = "/sys/devices/platform/lg-laptop/reader_mode";
    const FAN_PATH: &str = "/sys/devices/platform/lg-laptop/fan_mode";
    const USB_PATH: &str = "/sys/devices/platform/lg-laptop/usb_charge";

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

    fn write_u32_to_file(value: u32, file: &str) -> Result<ExitStatus, String> {
        let mut process = Command::new("pkexec")
            .arg("tee")
            .arg(file)
            .stdin(Stdio::piped())
            .spawn()
            .map_err(|error| error.to_string())?;

        if let Some(mut stdin) = process.stdin.take() {
            let content = format!("{value}\n");

            stdin.write_all(content.as_bytes())
                .map_err(|error| error.to_string())?;
        }

        process.wait()
            .map_err(|error| error.to_string())
    }

    //---------------------------------------
    // Battery limit function
    //---------------------------------------
    pub fn battery_limit() -> Result<u32, String> {
        let battery_limit = parse_u32_from_file(BATTERY_PATH)?;

        Ok(if battery_limit == 100 { 0 } else { 1 })
    }

    //---------------------------------------
    // Fn lock functions
    //---------------------------------------
    pub fn fn_lock() -> Result<bool, String> {
        let fn_lock = parse_u32_from_file(FNLOCK_PATH)?;

        Ok(fn_lock != 0)
    }

    pub fn set_fn_lock(value: u32) -> Result<std::process::ExitStatus, String> {
        write_u32_to_file(value, FNLOCK_PATH)
    }

    //---------------------------------------
    // Reader mode function
    //---------------------------------------
    pub fn reader_mode() -> Result<bool, String> {
        let reader_mode = parse_u32_from_file(READER_PATH)?;

        Ok(reader_mode != 0)
    }

    //---------------------------------------
    // Fan mode function
    //---------------------------------------
    pub fn fan_mode() -> Result<bool, String> {
        let fan_mode = parse_u32_from_file(FAN_PATH)?;

        // Note 0 = silent fan enabled
        Ok(fan_mode == 0)
    }

    //---------------------------------------
    // USB charge function
    //---------------------------------------
    pub fn usb_charge() -> Result<bool, String> {
        let usb_charge = parse_u32_from_file(USB_PATH)?;

        Ok(usb_charge != 0)
    }
}

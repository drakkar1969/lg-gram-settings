//------------------------------------------------------------------------------
// MODULE: Gram
//------------------------------------------------------------------------------
pub mod gram {
    use std::fs;
    use std::process::Command;

    //---------------------------------------
    // Constants
    //---------------------------------------
    const SETTINGS_PATH: &str = "/sys/devices/platform/lg-laptop";

    //---------------------------------------
    // System information function
    //---------------------------------------
    pub fn system_information() -> Result<String, String> {
        let output = Command::new("pkexec")
            .arg("lg-gram-writer")
            .arg("--system-info")
            .output()
            .map_err(|error| error.to_string())?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).into())
        }

        Ok(String::from_utf8_lossy(&output.stdout).into())
    }

    //---------------------------------------
    // Feature function
    //---------------------------------------
    pub fn feature(id: &str) -> Result<String, String> {
        let file = format!("{SETTINGS_PATH}/{id}");

        fs::read_to_string(file)
            .map_err(|error| error.to_string())
            .map(|value| value.trim().to_owned())
    }

    //---------------------------------------
    // Set feature function
    //---------------------------------------
    pub fn set_feature(id: &str, value: &str) -> Result<String, String> {
        let output = Command::new("pkexec")
            .arg("lg-gram-writer")
            .arg("--feature")
            .arg(format!("{id}={value}"))
            .output()
            .map_err(|error| error.to_string())?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).into())
        }

        Ok(String::from_utf8_lossy(&output.stdout).into())
    }

    //---------------------------------------
    // Is service enabled function
    //---------------------------------------
    pub fn is_service_enabled(id: &str) -> Result<bool, String> {
        let unit_file = format!("lg-gram-{}.service", id.replace('_', "-"));

        let status = Command::new("systemctl")
            .arg("--quiet")
            .arg("is-enabled")
            .arg(unit_file)
            .status()
            .map_err(|error| error.to_string())?;

        Ok(status.success())
    }

    //---------------------------------------
    // Enable service function
    //---------------------------------------
    pub fn enable_service(id: &str, value: u32) -> Result<String, String> {
        let output = Command::new("pkexec")
            .arg("lg-gram-writer")
            .arg("--service")
            .arg(format!("{id}={value}"))
            .output()
            .map_err(|error| error.to_string())?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).into())
        }

        Ok(String::from_utf8_lossy(&output.stdout).into())
    }
}

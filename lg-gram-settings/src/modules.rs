//------------------------------------------------------------------------------
// MODULE: KernelFeatures
//------------------------------------------------------------------------------
pub mod kernel_features {
    use std::fs;
    use std::process::Command;

    //---------------------------------------
    // Constants
    //---------------------------------------
    const SETTINGS_PATH: &str = "/sys/devices/platform/lg-laptop";

    //---------------------------------------
    // Feature function
    //---------------------------------------
    pub fn feature(id: &str) -> Result<u32, String> {
        let file = format!("{}/{}", SETTINGS_PATH, id);

        fs::read_to_string(file)
            .map_err(|error| error.to_string())
            .and_then(|value| {
                value.trim().parse::<u32>()
                    .map_err(|error| error.to_string())
            })
    }

    //---------------------------------------
    // Set feature function
    //---------------------------------------
    pub fn set_feature(id: &str, value: u32) -> Result<String, String> {
        let output = Command::new("pkexec")
            .arg("lg-gram-writer")
            .arg("--kernel")
            .arg(format!("{id}={value}"))
            .output()
            .map_err(|error| error.to_string())?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).into())
        }

        Ok(String::from_utf8_lossy(&output.stdout).into())
    }
}

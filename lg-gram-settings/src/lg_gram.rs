//------------------------------------------------------------------------------
// MODULE: Gram
//------------------------------------------------------------------------------
pub mod gram {
    use std::fs;

    use async_process::Command as AsyncCommand;

    //---------------------------------------
    // Constants
    //---------------------------------------
    const WRITER: &str = "/usr/share/lg-gram-settings/lg-gram-writer";
    const SETTINGS_PATH: &str = "/sys/devices/platform/lg-laptop";

    //---------------------------------------
    // System information function
    //---------------------------------------
    pub async fn system_information_async() -> Result<String, String> {
        let output = AsyncCommand::new("pkexec")
            .arg(WRITER)
            .arg("--system-info")
            .output()
            .await
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
    pub async fn set_feature_async(id: &str, value: &str) -> Result<String, String> {
        let output = AsyncCommand::new("pkexec")
            .arg(WRITER)
            .arg("--feature")
            .arg(format!("{id}={value}"))
            .output()
            .await
            .map_err(|error| error.to_string())?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).into())
        }

        Ok(String::from_utf8_lossy(&output.stdout).into())
    }
}

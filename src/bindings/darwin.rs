use std::env;
use std::process::Command;

mod darwin {
    pub fn getID() {
        let hwid = Command::new("ioreg").arg("-rd1").arg("-c").arg("IOExpertPlatformDevice").output().expect("Failed to get macOS HWID");
        return hwid
    }
}
use std::env;
use std::process::Command;
use std::fs::File;
use std::io::Write;

mod darwin {
    pub fn getID() -> std::string::String {
        let fuck = Command::new("ioreg").arg("-rd1").arg("-c").arg("IOExpertPlatformDevice").output().expect("Failed to get macOS HWID");
        let mut hwid = File::create("/tmp/hwid-please-ignore.txt");
        hwid.write_all(&fuck.stdout).unwrap();
        return hwid.read();
    }
}
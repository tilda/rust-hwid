// rust-hwid
// (c) 2020 tilda, under MIT license

use thiserror::Error;

#[derive(Debug, Error)]
pub enum HwIdError {
    #[error("no HWID was found on system")]
    NotFound,
    #[error("{0:?}: contains malformed HWID")]
    Malformed(String),
}

#[cfg(target_os = "windows")]
mod hwid {
    use super::*;
    use winreg::enums::HKEY_LOCAL_MACHINE;

    pub fn get_id() -> Result<std::string::String, HwIdError> {
        // escaping is fun, right? right???
        let hive = winreg::RegKey::predef(HKEY_LOCAL_MACHINE)
            .open_subkey(r"\\SOFTWARE\Microsoft\Cryptography")
            .or(Err(HwIdError::NotFound))?;
        let id = hive
            .get_value("MachineGuid")
            .or(Err(HwIdError::NotFound))?;
        Ok(id)
    }
}

#[cfg(target_os = "darwin")]
mod hwid {
    use super::*;

    pub fn get_id() -> Result<std::string::String, HwIdError> {
        let cmd = std::process::Command::new("ioreg")
            .arg("-rd1")
            .arg("-c")
            .arg("IOExpertPlatformDevice")
            .output()
            .or(Err(HwIdError::NotFound))?;
        let id = cmd.stdout.last();
        Ok(id)
    }
}

#[cfg(target_os = "linux")]
mod hwid {
    use super::*;

    pub fn get_id() -> Result<std::string::String, HwIdError> {
        let paths = [
            "/var/lib/dbus/machine-id",
            "/etc/machine-id",
        ];
        for p in paths {
            if let Ok(id_contents) = std::fs::read_to_string(p) {
                let id_str = id_contents
                    .lines()
                    .next()
                    .ok_or_else(|| HwIdError::Malformed(id_contents.to_string()))?;
                return Ok(id_str.to_string());
            }
        }
        Err(HwIdError::NotFound)
    }
}

#[cfg(target_os = "freebsd")]
#[cfg(target_os = "dragonfly")]
#[cfg(target_os = "openbsd")]
#[cfg(target_os = "netbsd")]
mod hwid {
    pub fn get_id() -> std::string::String {
        unimplemented!("*BSD support is not implemented")
    }
}

pub use hwid::get_id;

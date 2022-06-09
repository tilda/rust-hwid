// rust-hwid
// (c) 2020 tilda, under MIT license

#[cfg(target_os = "windows")]
mod hwid {

    extern crate winreg;
    use winreg::enums::HKEY_LOCAL_MACHINE;

    pub fn get_id() -> std::string::String {
        // escaping is fun, right? right???
        let hive = winreg::RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey("\\\\SOFTWARE\\Microsoft\\Cryptography").expect("Failed to open subkey");
        let id = hive.get_value("MachineGuid").expect("Failed to get MachineGuid");
        return id
    }
}

#[cfg(target_os = "darwin")]
mod hwid {
    pub fn get_id() -> std::string::String {
        let cmd = std::process::Command::new("ioreg").arg("-rd1").arg("-c").arg("IOExpertPlatformDevice").output().expect("Failed to get HWID");
        let id = cmd.stdout.last();
        return id
    }
}

#[cfg(target_os = "linux")]
mod hwid {
    pub fn get_id() -> std::string::String {
        let paths = [
            "/var/lib/dbus/machine-id",
            "/etc/machine-id",
        ];
        for p in paths {
            if let Ok(id_string) = std::fs::read_to_string(p) {
                return id_string
                    .lines()
                    .next()
                    .expect("{p}: malformed id file")
                    .to_string()
            }
        }
        panic!("No HWID was found on system");
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

use std::fs;
use std::path::Path;

mod linux {
    pub fn getID() {
        let mut hwid = Path::new("/var/lib/dbus/machine-id").exists();
        if hwid == false {
            hwid = Path::new("/etc/machine-id").exists()
            if hwid == false {
                panic!("No machine ID could be found.");
            }
            return hwid
        }
        return hwid
    }
}
use winreg::enums::*;
use winreg::RegKey;

mod win32 {
    fn getID() {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let crypto = hklm.open_subkey('\\SOFTWARE\Microsoft\Cryptography');
        let hwid: String = crypto.get_value("MachineGuid");
        return hwid
    }
}
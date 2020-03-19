// rust-hwid
// (c) 2020 tilda, under MIT license

use std::env;
use std::process::Command;
use std::fs;
use std::path::Path;
use winreg::enums::*;
use winreg::RegKey;

mod bsd;
mod darwin;
mod linux;
mod win32;


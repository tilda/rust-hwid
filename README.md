# rust-hwid

A simple library to detect an OS equivalent of an Hardware ID

# Currently supported
- Windows
- macOS
- Linux (dbus, systemd HWIDs)

# TODO
Find a way to support *BSDs or ditch the function for them entirely

# Usage
There is a single function: `get_id()`

An example is provided in the `examples` folder.
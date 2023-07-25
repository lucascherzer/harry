# harry

Simple socket based, password authenticated bind shell for windows and linux systems

# Disclaimer
**This is by no means secure and should not be used in scenarios where something of value is at stake**
I put this together as a quick and easy backdoor for KOTH games/TryHackMe rooms/HackTheBox boxes.

## Building
Before building, change the password in src/main.rs (Line 8). After that, its as easy as running `cargo build --release`

## Connecting

1. Connect to sock (e.g. via `nc`)
2. Send plain text password
3. Send commands to be interpreted

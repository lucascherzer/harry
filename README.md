# harry

Simple socket based, password authenticated bind shell for windows and linux systems

# Disclaimer
**This is by no means secure and should not be used in scenarios where something of value is at stake**
I put this together as a quick and easy backdoor for KOTH games/TryHackMe rooms/HackTheBox boxes.

## Building
Before building, some settings can be adjusted in the source code under the imports in `src/main.rs` 

## Connecting

1. Connect to sock (e.g. via `nc`)
2. Send plain text password
3. Send commands to be interpreted

# AutoUpdater

A simple app that scans your `~/Downloads` folder for `.deb` package files and installs them automatically.

## Usage
Run the app once and it will continue running in the background.  
To enable auto-start on boot, add it to your startup applications.

## Permissions
Requires administrative privileges (`sudo`) to install packages.

## Requirements
- Debian-based Linux distribution
- GNOME desktop environment

## Note
Currently supports only GNOME-based systems.

## Future Plans
- Support for other desktop environments
- User prompts for install confirmation
- Adjustable timeout between scans (now 10 seconds)
- Logging support

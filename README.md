# lumoctl

**lumoctl** is a lightweight command-line tool for automating GPIO-controlled light switches based on daily schedules such as Islamic prayer times. It is built with Rust and designed for use on devices like Raspberry Pi.

## Features

- Automatically toggles lights based on daily prayer times (fajr, syuruq, maghrib).
- Multiple switch support with configurable GPIO pins.
- Supports normally-closed relay logic (GPIO `0` = ON, `1` = OFF).
- Safe boot behavior — turns off all relays on startup.
- Periodic polling every 27 seconds.

## How It Works

1. All configured GPIO outputs are turned OFF at launch.
2. The app reads the current day’s schedule from a MySQL-compatible database.
3. It determines ON/OFF windows for each pin.
4. GPIO pins are toggled based on the current time and logic rules.

## Requirements

- **MySQL Client Library**: `libmysqlclient` (for database connection via Diesel).
- **GPIO CLI Utility**: A tool like `gpio` ([`WiringPi` (Unofficial Fork)](https://github.com/WiringPi/WiringPi)) must be installed and available in the system path.
  - Required to use the `gpio` CLI tool
  - ⚠️ Note: The original WiringPi is deprecated. Use the updated fork linked above.
- Rust toolchain with Diesel CLI (configured for MySQL).

## Example Use Case

- Switch 1 (porch): ON from *maghrib* (sunset) to 10 minutes before *syuruq* (sunrise).
- Switch 2 (backyard): ON from 20 minutes before *maghrib* to 10 minutes after *syuruq*.
- Switch 3 (window): ON from 20 minutes after *maghrib* to 10 minutes after *fajr* (time for the Fajr prayer).

## Status

Currently a CLI application. Future plans include web and mobile interfaces.

## License

MIT
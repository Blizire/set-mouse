# set-mouse

A tiny Windows command-line tool for setting the mouse pointer speed
(the same value controlled by the "Pointer speed" slider in
**Settings → Bluetooth & devices → Mouse → Additional mouse settings**).

This project was made entirely with [Claude](https://claude.com/claude-code).

## Requirements

- Windows 10/11
- [Rust toolchain](https://www.rust-lang.org/tools/install) (`cargo`/`rustc`) to build from source

## Installation

### Option 1: Install script (recommended)

Installs `set-mouse` system-wide so it's available to every user account
on the machine.

1. Clone or download this repository.
2. Open PowerShell **as Administrator** and run:

   ```powershell
   .\install.ps1
   ```

   This builds a release binary, copies it to
   `%ProgramFiles%\set-mouse\set-mouse.exe`, and adds that folder to the
   system-wide `PATH`.

3. Open a new terminal window and run `set-mouse` from anywhere.

### Option 2: Manual build

1. Clone or download this repository.
2. Build a release binary:

   ```
   cargo build --release
   ```

3. The executable will be at `target\release\set-mouse.exe`.
4. Copy it to a folder that's on your `PATH`, for example:

   ```powershell
   Copy-Item target\release\set-mouse.exe "$env:USERPROFILE\bin\set-mouse.exe"
   ```

   (Create `%USERPROFILE%\bin` first if it doesn't exist, and add it to
   your `PATH` via **Settings → System → About → Advanced system
   settings → Environment Variables** if it isn't already.)

## Usage

```
set-mouse <speed>
```

`<speed>` is an integer from **1** (slowest) to **20** (fastest). Windows'
default is **10**.

### Examples

```powershell
set-mouse 5      # slower than default
set-mouse 10     # restore the Windows default
set-mouse 20     # fastest
```

On success you'll see:

```
Mouse pointer speed set to 5.
```

The change takes effect immediately (no reboot or sign-out required) and
is saved, so it persists across restarts — the same as changing it by
hand in Settings.

## Troubleshooting

- **"is not a valid integer"** — make sure you passed a whole number
  (e.g. `set-mouse 5`, not `set-mouse 5.0`).
- **"speed must be between 1 and 20"** — Windows only accepts values in
  that range; pick a value inside it.
- **`set-mouse` isn't recognized** — the folder containing
  `set-mouse.exe` isn't on your `PATH`, or you need to open a new
  terminal window after updating `PATH`.

## How it works

The tool calls the Win32 `SystemParametersInfoW` API with
`SPI_SETMOUSESPEED`, the same mechanism Windows' own Settings app uses,
so behavior matches what you'd get from the UI.

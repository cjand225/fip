# Fast Install Program

FIP is a universal Linux application installer written in rust that acts as a smart, user-friendly wrapper around system-level and portable package managers.

It allows users to install and manage applications from a single, consistent interface — without worrying about whether the app is distributed as an AppImage, Flatpak, Snap, or through traditional system package managers like APT or Pacman.

>“You don’t care how it gets installed. You just want it installed. FIP does that.”

## Key Features

- Unified CLI interface
    - One command to rule them all, install, remove, or search apps -- regardless of the backend.
- Pluggable Providers
    - Ability to support any backend install system via common interface modules. (If its not supported, it will be!)
- Configurable Defaults
    - Override preferred install methods globally or per-install.
- Safe and Transparent
    - FIP doesn't dictate what your package manager does -- It delegates to it.

## Supported Implementations

As they're implemented, supported package implementations will be added here:

- APT
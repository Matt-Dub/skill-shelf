# Tauri for the desktop shell

The widget stays open all day (tray panel, optionally pinned to the desktop) and ships as open source on Windows, macOS and Linux, so we build it with Tauri (Rust backend, TypeScript web UI) rather than Electron or one native app per OS. Tauri gives native tray, frameless/transparent windows and a ~10 MB binary with a small memory footprint; Electron would cost ~150 MB of RAM for a list of skills, and native per-OS apps would triple the codebase.

## Consequences

- Filesystem scanning and watching of the Library live in the Rust side; the UI only renders.
- Pinned mode relies on per-OS window behaviour that Tauri does not abstract. On Linux Wayland it goes through the layer-shell protocol applied to the GTK window (supported by KDE Plasma and wlroots compositors such as Sway or Hyprland, not by GNOME), so it is a Linux-specific code path inside the same app, not a separate build.

# Changelog

All notable changes to **Kiyora** will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-11-14

### Initial Production Release

- **Native Windows 10/11 Architecture**: Built 100% in Rust (`x86_64-pc-windows-msvc`) with Slint UI for ultra-low memory consumption (~15-32 MB RAM at idle).
- **Direct Discord Gateway v9**: Connects directly to Discord's official Gateway v9 and REST APIs with zero telemetry or third-party intermediaries.
- **Hardware-Accelerated Voice & Video**:
  - Pure Rust Opus audio engine with RNNoise suppression and smart speech ducking.
  - Screen capture with NVENC (NVIDIA), AMF (AMD), and DXGI/WGC hardware pipelines.
  - Discord Audio/Video End-to-End Encryption (DAVE) support.
- **Full English Localization & Refined Branding**:
  - English is the default and primary language across all UI elements, system messages, console logs, and documentation.
  - Vibrant Kiyora visual identity (Deep Slate `#141A23` background and Glacier Cyan `#3CD7EB` accents).
  - Prominent multi-resolution Kiyora icon embedded in `kiyora.exe`, system tray, and window taskbar.
- **Clean Windows Logging**:
  - Safe log isolation to `%LOCALAPPDATA%\Kiyora\logs\kiyora.log` (and crash dump to `panic.log`).
- **Secure Authentication**:
  - Windows DPAPI token encryption in `%APPDATA%\Kiyora\`.
  - Discord Remote Auth QR Code scanning support.

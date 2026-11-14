# Kiyora

<p align="center">
  <img src="assets/app_icon.png" alt="Kiyora Logo" width="128" height="128"/>
</p>

<p align="center">
  <strong>Ultra-lightweight native Discord client for Windows 10/11, built in Rust + Slint.</strong>
</p>

<p align="center">
  <a href="https://github.com/ciclo9/kiyora/releases"><img src="https://img.shields.io/github/v/release/ciclo9/kiyora?style=flat-square&color=3CD7EB" alt="Latest Release"/></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-3CD7EB?style=flat-square" alt="MIT License"/></a>
  <img src="https://img.shields.io/badge/platform-Windows%2010%2F11-3CD7EB?style=flat-square" alt="Windows 10/11"/>
  <img src="https://img.shields.io/badge/rust-stable-3CD7EB?style=flat-square" alt="Rust"/>
</p>

---

## What is Kiyora?

**Kiyora** is a native, ultra-lightweight Discord client for **Windows 10 and Windows 11**, written entirely in [Rust](https://www.rust-lang.org/) with a [Slint](https://slint.dev/) UI. It connects directly to Discord's Gateway v9 API without an Electron shell, consuming as little as **~15 MB of RAM** at idle — a fraction of the official client.

> **Windows-only by design.** Kiyora is built exclusively for `x86_64-pc-windows-msvc` and uses native Win32 APIs: WASAPI for audio, DXGI/MediaFoundation for video capture, DPAPI for secure token storage, and Direct3D 11 for hardware acceleration.

---

## Features

- 🚀 **~15 MB RAM** at idle — no Electron, no Chromium
- 🎙️ **Voice chat** with Opus encoding, noise suppression (RNNoise), speech ducking
- 🖥️ **Screen sharing** with NVENC (NVIDIA), AMF (AMD), and WMF (software) H.264 encoding
- 🔐 **Secure token storage** via Windows DPAPI
- 🎨 **Native Windows UI** with Slint — smooth animations, system tray, global keybinds
- 🌐 **Multi-language**: English (default), Portuguese, Spanish, German, French, Russian, Japanese
- ⚡ **Direct Gateway v9 connection** — no proxy, no telemetry, no intermediaries
- 🔄 **Auto-updater** via GitHub Releases

---

## Download

Download the latest `kiyora.exe` from the [**Releases**](https://github.com/ciclo9/kiyora/releases) page.

No installer required — simply run `kiyora.exe`.

---

## Building from Source

### Prerequisites

- **Windows 10 or 11** (`x86_64`)
- [Rust](https://rustup.rs/) (stable, MSVC toolchain): `rustup install stable-x86_64-pc-windows-msvc`
- [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) with C++ and Windows 10/11 SDK
- [Git](https://git-scm.com/)

### Build

```powershell
git clone https://github.com/ciclo9/kiyora.git
cd kiyora

# Debug build (fast compilation)
cargo build

# Release build (optimized — kiyora.exe in target/release/)
cargo build --release --locked
```

The compiled binary will be at `target/release/kiyora.exe`.

---

## Configuration

Kiyora stores all configuration in `%APPDATA%\Kiyora\` and `%LOCALAPPDATA%\Kiyora\`.

| File | Purpose |
|------|---------|
| `.kiyora_language.json` | UI language preference |
| `.kiyora_updater_config.json` | Update ignore list |
| `kiyora_app.log` | Application log |

---

## Architecture

```
kiyora/
├── src/
│   ├── main.rs              # Entry point, event loop
│   ├── gateway/             # Discord Gateway v9 WebSocket client
│   ├── audio/               # WASAPI voice engine + Opus + RNNoise
│   ├── screen_capture/      # DXGI/WGC capture + NVENC/AMF/WMF encoder
│   ├── auth/                # DPAPI token vault + QR remote auth
│   ├── encoder/             # H.264 hardware encoders
│   ├── ui/                  # Slint UI helpers and data bindings
│   └── utils/               # i18n, keybinds, tray, updater, logger
├── ui/
│   ├── appwindow.slint      # Main application window
│   ├── theme.slint          # Kiyora color palette & typography
│   └── components/          # Reusable UI components
└── assets/                  # Icons, fonts, SVGs
```

---

## Security

- All connections go **exclusively** to Discord's official endpoints: `discord.com`, `gateway.discord.gg`, `cdn.discordapp.com`
- Version update checks query `api.github.com/repos/ciclo9/kiyora/releases/latest` only
- **No telemetry, no analytics, no third-party servers**
- Discord token is stored encrypted via Windows DPAPI in `%APPDATA%\Kiyora\`

See [SECURITY.md](SECURITY.md) for the full security policy.

---

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) first.

- **Language**: All code, comments, and documentation must be written in **English**
- **Platform**: Windows 10/11 (`x86_64-pc-windows-msvc`) only

---

## License

[MIT License](LICENSE) © ciclo9

---

<p align="center">
  <a href="https://github.com/ciclo9/kiyora">github.com/ciclo9/kiyora</a>
</p>

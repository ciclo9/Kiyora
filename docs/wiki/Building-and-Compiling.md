# 🛠️ Building & Compiling kiyora

This guide explains how to compile and build production-ready kiyora binaries from source code.

---

## 1. Prerequisites

### Windows (10 / 11 x64)
- **Rust Toolchain**: `rustup default stable-x86_64-pc-windows-msvc`
- **Visual Studio C++ Build Tools**: MSVC compiler and Windows 10/11 SDK.
- **Inno Setup (Optional)**: For building the official `kiyora-Setup-x64.exe` installer.

### Linux (Debian / Ubuntu / Arch / Fedora)
- Required system packages:
  ```bash
  # Debian/Ubuntu:
  sudo apt-get install -y build-essential libasound2-dev libssl-dev libx11-dev libfontconfig1-dev
  
  # Arch Linux:
  sudo pacman -S --needed base-devel alsa-lib openssl libx11 fontconfig
  ```

---

## 2. Build Commands

### Development Mode (Fast compile)
```bash
cargo run --bin kiyora
```

### Production Release (Fully optimized)
```bash
cargo run --bin kiyora --release
```

### Statically Linked Windows Binary
```bash
RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --bin kiyora
```

### Universal AnyLinux AppImage (quick-sharun)
kiyora packages all dynamic libraries into a distro-agnostic AppImage:
```bash
# 1. Download quick-sharun
wget -q https://raw.githubusercontent.com/pkgforge-dev/Anylinux-AppImages/refs/heads/main/useful-tools/quick-sharun.sh -O ./quick-sharun
chmod +x ./quick-sharun

# 2. Package binary and bundled dependencies
export OUTPATH=.
export DESKTOP=assets/kiyora.desktop
export ICON=assets/kiyora.png
export DEPLOY_GTK=1

./quick-sharun target/release/kiyora
./quick-sharun --make-appimage
mv *.AppImage kiyora-x86_64.AppImage
```

---

## 3. Cargo Optimization Settings

`Cargo.toml` includes tuned release profile settings:
```toml
[profile.release]
opt-level = 3
lto = "thin"
codegen-units = 1
panic = "abort"
strip = true
```
This reduces the final standalone executable down to **~8 MB** with instant sub-150ms startup times.

use std::path::PathBuf;
use tokio::sync::mpsc;
use log::info;
use serde::{Deserialize, Serialize};

const REPO_OWNER: &str = "ciclo9";
const REPO_NAME: &str = "kiyora";
const CONFIG_FILE: &str = ".kiyora_updater_config.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseInfo {
    pub tag_name: String,
    pub version: String,
    pub download_url: String,
    pub release_name: String,
    pub release_body: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct UpdaterConfig {
    ignored_version: Option<String>,
}

fn get_config_path() -> PathBuf {
    PathBuf::from(CONFIG_FILE)
}

pub fn is_version_ignored(tag: &str) -> bool {
    let path = get_config_path();
    if path.exists() {
        if let Ok(data) = std::fs::read_to_string(&path) {
            if let Ok(cfg) = serde_json::from_str::<UpdaterConfig>(&data) {
                if let Some(ref ignored) = cfg.ignored_version {
                    return ignored == tag;
                }
            }
        }
    }
    false
}

pub fn save_ignored_version(tag: &str) {
    let cfg = UpdaterConfig {
        ignored_version: Some(tag.to_string()),
    };
    if let Ok(data) = serde_json::to_string_pretty(&cfg) {
        let _ = std::fs::write(get_config_path(), data);
        info!("🏷️ Version {} marked as ignored in {}", tag, CONFIG_FILE);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedVersionInfo {
    pub status: VersionComparisonStatus,
    pub local_version: String,
    pub local_hash: String,
    pub is_dirty: bool,
    pub remote_version: String,
    pub remote_tag: String,
    pub release_name: String,
    pub release_body: String,
    pub download_url: String,
    pub formatted_status: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VersionComparisonStatus {
    UpToDate,
    Outdated,
    AheadOfRelease,
    Modified,
}

/// Returns the formatted local version string (e.g. "v1.3.1-beta (0a21d14)" or "v1.3.1-beta (0a21d14-modified)")
pub fn get_local_version_string() -> String {
    let base_ver = env!("CARGO_PKG_VERSION");
    let git_hash = env!("KIYORA_GIT_HASH");
    let is_dirty = env!("KIYORA_GIT_DIRTY") == "true";

    if is_dirty {
        format!("v{} ({}-modified)", base_ver, git_hash)
    } else {
        format!("v{} ({})", base_ver, git_hash)
    }
}

/// Compares version strings (e.g., "0.2.2" > "0.2.1"). Only the numeric parts count:
/// "1.3.1-beta" is treated as 1.3.1.
pub fn is_newer_version(remote_ver: &str, current_ver: &str) -> bool {
    let parse_ver = |v: &str| -> Vec<u64> {
        v.trim_start_matches('v')
            .split('.')
            .map(|p| {
                let digits = p.find(|c: char| !c.is_ascii_digit()).map_or(p, |end| &p[..end]);
                digits.parse::<u64>().unwrap_or(0)
            })
            .collect()
    };

    let remote_parts = parse_ver(remote_ver);
    let current_parts = parse_ver(current_ver);

    let max_len = remote_parts.len().max(current_parts.len());
    for i in 0..max_len {
        let r = remote_parts.get(i).copied().unwrap_or(0);
        let c = current_parts.get(i).copied().unwrap_or(0);
        if r > c {
            return true;
        } else if r < c {
            return false;
        }
    }
    false
}

/// Checks GitHub API for the latest release (respecting ignored versions)
pub async fn check_for_updates() -> Option<ReleaseInfo> {
    let det = check_version_status_detailed(true).await.ok()?;
    if det.status == VersionComparisonStatus::Outdated {
        Some(ReleaseInfo {
            tag_name: det.remote_tag,
            version: det.remote_version,
            download_url: det.download_url,
            release_name: det.release_name,
            release_body: det.release_body,
        })
    } else {
        None
    }
}

/// Manually checks GitHub API for the latest release and returns complete comparison status
pub async fn check_for_updates_manual() -> Result<DetailedVersionInfo, String> {
    check_version_status_detailed(false).await
}

pub async fn check_version_status_detailed(respect_ignored: bool) -> Result<DetailedVersionInfo, String> {
    let local_version = env!("CARGO_PKG_VERSION").to_string();
    let local_hash = env!("KIYORA_GIT_HASH").to_string();
    let is_dirty = env!("KIYORA_GIT_DIRTY") == "true";

    let url = format!("https://api.github.com/repos/{}/{}/releases/latest", REPO_OWNER, REPO_NAME);
    info!("🔍 Comparing local version ({}) with the latest GitHub release...", get_local_version_string());

    let client = reqwest::Client::builder()
        .user_agent("Kiyora-Updater/1.0")
        .build()
        .map_err(|e| e.to_string())?;

    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !resp.status().is_success() {
        return Err(format!("GitHub API returned status {}", resp.status()));
    }

    let json: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;

    let remote_tag = json["tag_name"].as_str().unwrap_or("").to_string();
    let remote_version = remote_tag.trim_start_matches('v').to_string();
    let release_name = json["name"].as_str().unwrap_or("").to_string();
    let release_body = json["body"].as_str().unwrap_or("").to_string();

    // Find the kiyora.exe download URL
    let download_url = json["assets"]
        .as_array()
        .and_then(|assets| {
            assets.iter().find(|a| {
                a["name"].as_str().map(|n| n.ends_with(".exe")).unwrap_or(false)
            })
        })
        .and_then(|a| a["browser_download_url"].as_str())
        .unwrap_or("")
        .to_string();

    if respect_ignored && is_version_ignored(&remote_tag) {
        return Ok(DetailedVersionInfo {
            status: VersionComparisonStatus::UpToDate,
            local_version,
            local_hash,
            is_dirty,
            remote_version,
            remote_tag,
            release_name,
            release_body,
            download_url,
            formatted_status: "Up to date (update ignored)".to_string(),
        });
    }

    let status = if is_dirty {
        VersionComparisonStatus::Modified
    } else if is_newer_version(&remote_version, &local_version) {
        VersionComparisonStatus::Outdated
    } else if is_newer_version(&local_version, &remote_version) {
        VersionComparisonStatus::AheadOfRelease
    } else {
        VersionComparisonStatus::UpToDate
    };

    let formatted_status = match status {
        VersionComparisonStatus::UpToDate => "✅ Up to date".to_string(),
        VersionComparisonStatus::Outdated => format!("⬇️ Update available: v{}", remote_version),
        VersionComparisonStatus::AheadOfRelease => "🚀 Ahead of public release".to_string(),
        VersionComparisonStatus::Modified => format!("🔧 Locally modified ({})", local_hash),
    };

    Ok(DetailedVersionInfo {
        status,
        local_version,
        local_hash,
        is_dirty,
        remote_version,
        remote_tag,
        release_name,
        release_body,
        download_url,
        formatted_status,
    })
}

fn get_secure_update_dir() -> PathBuf {
    let path = std::env::temp_dir().join("kiyora_update");
    let _ = std::fs::create_dir_all(&path);
    path
}

/// Downloads the release file and launches the updater/installer
pub async fn download_and_install_update(
    download_url: String,
    progress_tx: mpsc::Sender<f32>,
) -> Result<(), String> {
    info!("Starting update download: {}", download_url);
    if !download_url.starts_with("https://github.com/ciclo9/kiyora/")
        && !download_url.starts_with("https://objects.githubusercontent.com/")
        && !download_url.starts_with("https://github-releases.githubusercontent.com/")
    {
        return Err("Update URL rejected for security: unauthorized domain.".to_string());
    }

    let client = reqwest::Client::builder()
        .user_agent("Kiyora-App-Updater/1.0")
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {:?}", e))?;

    let mut response = client.get(&download_url).send().await
        .map_err(|e| format!("Failed to connect to download URL: {:?}", e))?;

    if !response.status().is_success() {
        return Err(format!("Download failed with HTTP status {}", response.status()));
    }

    let total_size = response.content_length().unwrap_or(0);
    let temp_dir = get_secure_update_dir();

    let is_installer = download_url.to_lowercase().contains("setup") || download_url.to_lowercase().contains("installer");
    let target_filename = if is_installer {
        "Kiyora-Update-Setup.exe"
    } else {
        "kiyora_new.exe"
    };

    let target_path = temp_dir.join(target_filename);
    let mut file = tokio::fs::File::create(&target_path).await
        .map_err(|e| format!("Failed to create temporary file: {:?}", e))?;

    let mut downloaded: u64 = 0;

    while let Some(chunk) = response.chunk().await.map_err(|e| format!("Error while downloading: {:?}", e))? {
        tokio::io::AsyncWriteExt::write_all(&mut file, &chunk).await
            .map_err(|e| format!("Error writing downloaded data: {:?}", e))?;

        downloaded += chunk.len() as u64;
        if total_size > 0 {
            let progress = (downloaded as f32 / total_size as f32).clamp(0.0, 1.0);
            let _ = progress_tx.try_send(progress);
        }
    }

    tokio::io::AsyncWriteExt::flush(&mut file).await
        .map_err(|e| format!("Failed to flush file: {:?}", e))?;
    drop(file);

    if total_size > 0 && downloaded != total_size {
        let _ = tokio::fs::remove_file(&target_path).await;
        return Err(format!("Incomplete download (received {} of {} bytes).", downloaded, total_size));
    }
    if downloaded < 50_000 {
        let _ = tokio::fs::remove_file(&target_path).await;
        return Err(format!("Download truncated or corrupted (only {} bytes received).", downloaded));
    }

    let _ = progress_tx.try_send(1.0);
    info!("Download complete and verified ({} bytes)! Target: {:?}", downloaded, target_path);

    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        if is_installer {
            let installer_str = target_path.to_string_lossy().to_string();
            let res = Command::new("cmd")
                .args([
                    "/c",
                    "timeout",
                    "/t",
                    "1",
                    "/nobreak",
                    ">nul",
                    "&",
                    "start",
                    "",
                    &installer_str,
                    "/SP-",
                    "/SILENT",
                    "/FORCECLOSEAPPLICATIONS",
                    "/RESTARTAPPLICATIONS",
                ])
                .spawn();

            match res {
                Ok(_) => {
                    info!("Installer scheduled successfully. Exiting Kiyora process...");
                    unsafe {
                        windows_sys::Win32::System::Threading::TerminateProcess(
                            windows_sys::Win32::System::Threading::GetCurrentProcess(),
                            0,
                        );
                    }
                    std::process::exit(0);
                }
                Err(_e) => {
                    let _ = Command::new(&target_path)
                        .args(["/SP-", "/FORCECLOSEAPPLICATIONS", "/RESTARTAPPLICATIONS"])
                        .spawn();
                    std::process::exit(0);
                }
            }
        } else {
            // Standalone executable: replace running binary via background PowerShell command
            if let Ok(current_exe) = std::env::current_exe() {
                let script = format!(
                    "Start-Sleep -Seconds 1; Copy-Item -Force '{}' '{}'; Start-Process '{}'",
                    target_path.display(),
                    current_exe.display(),
                    current_exe.display()
                );
                let _ = Command::new("powershell")
                    .args(["-WindowStyle", "Hidden", "-Command", &script])
                    .spawn();

                info!("Kiyora replacement scheduled. Exiting...");
                unsafe {
                    windows_sys::Win32::System::Threading::TerminateProcess(
                        windows_sys::Win32::System::Threading::GetCurrentProcess(),
                        0,
                    );
                }
                std::process::exit(0);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::is_newer_version;

    #[test]
    fn a_new_last_number_with_a_suffix_is_newer() {
        assert!(is_newer_version("1.3.1-beta", "1.3.0-beta"));
        assert!(is_newer_version("v1.3.1-beta", "1.3.0-beta"));
        assert!(is_newer_version("1.4.0-beta", "1.3.1-beta"));
        assert!(!is_newer_version("1.3.1-beta", "1.3.1-beta"));
        assert!(!is_newer_version("1.3.0-beta", "1.3.1-beta"));
        assert!(!is_newer_version("1.3.1", "1.3.1-beta"));
    }
}

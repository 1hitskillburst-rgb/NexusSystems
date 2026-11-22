use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use crate::theme::ThemeMode;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppConfig {
    pub theme: ThemeMode,
    pub refresh_interval_ms: u64,
    pub window_width: f32,
    pub window_height: f32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: ThemeMode::Vintage,
            refresh_interval_ms: 1000,
            window_width: 1200.0,
            window_height: 580.0,
        }
    }
}

impl AppConfig {
    pub const CONFIG_FILE: &'static str = "nexus_settings.json";

    pub fn config_path() -> PathBuf {
        PathBuf::from(Self::CONFIG_FILE)
    }

    pub fn load_from(path: &Path) -> Option<Self> {
        if path.exists() {
            if let Ok(contents) = fs::read_to_string(path) {
                if let Ok(config) = serde_json::from_str::<AppConfig>(&contents) {
                    return Some(config);
                }
            }
        }
        None
    }

    pub fn load() -> Self {
        Self::load_from(&Self::config_path()).unwrap_or_default()
    }

    pub fn save_to(&self, target_path: &Path) -> std::io::Result<()> {
        let serialized = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        if let Some(parent) = target_path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }

        let tmp_path = match target_path.parent() {
            Some(parent) if !parent.as_os_str().is_empty() => {
                parent.join("nexus_settings.json.tmp")
            }
            _ => PathBuf::from("nexus_settings.json.tmp"),
        };

        let write_result = (|| -> std::io::Result<()> {
            let mut file = File::create(&tmp_path)?;
            file.write_all(serialized.as_bytes())?;
            file.sync_all()?;
            drop(file);
            fs::rename(&tmp_path, target_path)?;
            Ok(())
        })();

        if let Err(err) = write_result {
            eprintln!("Failed to save configuration atomically: {err}");
            let _ = fs::remove_file(&tmp_path);
            return Err(err);
        }

        Ok(())
    }

    pub fn save(&self) {
        if let Err(err) = self.save_to(&Self::config_path()) {
            eprintln!("Failed to save configuration: {err}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use std::time::{SystemTime, UNIX_EPOCH};

    static CWD_CONFIG_LOCK: Mutex<()> = Mutex::new(());

    fn unique_temp_dir(prefix: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("{}_{}_{}", prefix, std::process::id(), nanos));
        let _ = fs::create_dir_all(&dir);
        dir
    }

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.theme, ThemeMode::Vintage);
        assert_eq!(config.refresh_interval_ms, 1000);
        assert_eq!(config.window_width, 1200.0);
        assert_eq!(config.window_height, 580.0);
    }

    #[test]
    fn test_save_to_creates_valid_file_and_cleans_tmp() {
        let temp_dir = unique_temp_dir("nexus_test_save");
        let target_path = temp_dir.join("nexus_settings.json");
        let tmp_path = temp_dir.join("nexus_settings.json.tmp");

        let config = AppConfig {
            theme: ThemeMode::Forge,
            refresh_interval_ms: 500,
            window_width: 1400.0,
            window_height: 700.0,
        };

        let result = config.save_to(&target_path);
        assert!(result.is_ok(), "save_to should succeed: {:?}", result.err());

        assert!(target_path.exists(), "Target configuration file must exist");
        assert!(
            !tmp_path.exists(),
            "nexus_settings.json.tmp must not linger after save"
        );

        let loaded = AppConfig::load_from(&target_path);
        assert_eq!(loaded, Some(config));

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_save_to_overwrites_existing_file_atomically() {
        let temp_dir = unique_temp_dir("nexus_test_overwrite");
        let target_path = temp_dir.join("nexus_settings.json");
        let tmp_path = temp_dir.join("nexus_settings.json.tmp");

        let initial_config = AppConfig {
            theme: ThemeMode::Clearspace,
            refresh_interval_ms: 2000,
            window_width: 1000.0,
            window_height: 600.0,
        };
        initial_config.save_to(&target_path).unwrap();
        assert_eq!(AppConfig::load_from(&target_path), Some(initial_config));

        let updated_config = AppConfig {
            theme: ThemeMode::Terminal,
            refresh_interval_ms: 250,
            window_width: 1600.0,
            window_height: 900.0,
        };
        updated_config.save_to(&target_path).unwrap();

        assert_eq!(AppConfig::load_from(&target_path), Some(updated_config));
        assert!(
            !tmp_path.exists(),
            "Temporary file must not linger after overwrite"
        );

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_save_to_cleans_up_tmp_file_on_error() {
        let temp_dir = unique_temp_dir("nexus_test_err_cleanup");
        let target_path = temp_dir.join("nexus_settings.json");
        let tmp_path = temp_dir.join("nexus_settings.json.tmp");

        // Create target_path as a directory so that fs::rename(&tmp_path, target_path) fails
        fs::create_dir_all(&target_path).unwrap();

        let config = AppConfig::default();
        let result = config.save_to(&target_path);

        assert!(result.is_err(), "save_to should fail when rename target is a directory");
        assert!(
            !tmp_path.exists(),
            "Temporary file nexus_settings.json.tmp must be cleaned up on write error"
        );

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_save_method_persists_and_no_tmp_lingers() {
        let _guard = CWD_CONFIG_LOCK.lock().unwrap();

        let config_path = AppConfig::config_path();
        let tmp_path = match config_path.parent() {
            Some(p) if !p.as_os_str().is_empty() => p.join("nexus_settings.json.tmp"),
            _ => PathBuf::from("nexus_settings.json.tmp"),
        };

        // Backup existing config file if present
        let original_contents = if config_path.exists() {
            fs::read_to_string(&config_path).ok()
        } else {
            None
        };

        let test_config = AppConfig {
            theme: ThemeMode::Nebula,
            refresh_interval_ms: 750,
            window_width: 1350.0,
            window_height: 650.0,
        };

        test_config.save();

        assert!(config_path.exists(), "Saved config file must exist");
        assert!(
            !tmp_path.exists(),
            "Temporary file nexus_settings.json.tmp must not linger after save()"
        );

        let loaded = AppConfig::load();
        assert_eq!(loaded, test_config);

        // Restore original contents or clean up
        if let Some(orig) = original_contents {
            let _ = fs::write(&config_path, orig);
        } else {
            let _ = fs::remove_file(&config_path);
        }
        let _ = fs::remove_file(&tmp_path);
    }

    #[test]
    fn test_load_nonexistent_returns_default() {
        let temp_dir = unique_temp_dir("nexus_test_nonexistent");
        let target_path = temp_dir.join("non_existent_config.json");

        assert_eq!(AppConfig::load_from(&target_path), None);
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_load_corrupted_json_returns_none() {
        let temp_dir = unique_temp_dir("nexus_test_corrupt");
        let target_path = temp_dir.join("nexus_settings.json");

        fs::write(&target_path, b"{ not valid json ...").unwrap();

        assert_eq!(AppConfig::load_from(&target_path), None);
        let _ = fs::remove_dir_all(&temp_dir);
    }
}

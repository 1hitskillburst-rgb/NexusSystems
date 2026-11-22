use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use crate::theme::ThemeMode;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub theme: ThemeMode,
    pub refresh_interval_ms: u64,
    pub window_width: f32,
    pub window_height: f32,
    pub show_cooling: bool,
    pub show_storage: bool,
    pub show_network: bool,
    pub show_processes: bool,
    pub auto_hide_empty_cards: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: ThemeMode::Vintage,
            refresh_interval_ms: 1000,
            window_width: 1200.0,
            window_height: 580.0,
            show_cooling: true,
            show_storage: true,
            show_network: true,
            show_processes: true,
            auto_hide_empty_cards: true,
        }
    }
}

impl AppConfig {
    pub const CONFIG_FILE: &'static str = "nexus_settings.json";

    pub fn config_dir() -> PathBuf {
        match std::env::var("APPDATA") {
            Ok(appdata) if !appdata.trim().is_empty() => {
                PathBuf::from(appdata).join("NexusSystems")
            }
            _ => PathBuf::from("."),
        }
    }

    pub fn config_path() -> PathBuf {
        Self::config_dir().join(Self::CONFIG_FILE)
    }

    pub fn reset_to_defaults(&mut self) {
        *self = Self::default();
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
        let appdata_path = Self::config_path();
        if appdata_path.exists() {
            if let Some(config) = Self::load_from(&appdata_path) {
                return config;
            }
            return Self::default();
        }

        // Backward compatibility fallback: check CWD if %APPDATA% file does not exist
        let cwd_path = Path::new(Self::CONFIG_FILE);
        if cwd_path.exists() {
            if let Some(config) = Self::load_from(cwd_path) {
                return config;
            }
        }

        Self::default()
    }

    pub fn save_to(&self, target_path: &Path) -> std::io::Result<()> {
        let serialized = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        if let Some(parent) = target_path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }

        let tmp_file_name = format!(
            "{}.tmp",
            target_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(Self::CONFIG_FILE)
        );

        let tmp_path = match target_path.parent() {
            Some(parent) if !parent.as_os_str().is_empty() => {
                parent.join(&tmp_file_name)
            }
            _ => PathBuf::from(&tmp_file_name),
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

    static CONFIG_TEST_MUTEX: Mutex<()> = Mutex::new(());

    struct EnvGuard {
        key: &'static str,
        original: Option<String>,
    }

    impl EnvGuard {
        fn set(key: &'static str, value: impl AsRef<std::ffi::OsStr>) -> Self {
            let original = std::env::var(key).ok();
            std::env::set_var(key, value);
            Self { key, original }
        }

        fn remove(key: &'static str) -> Self {
            let original = std::env::var(key).ok();
            std::env::remove_var(key);
            Self { key, original }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            if let Some(ref orig) = self.original {
                std::env::set_var(self.key, orig);
            } else {
                std::env::remove_var(self.key);
            }
        }
    }

    struct CwdConfigBackupGuard {
        backup_path: PathBuf,
        had_file: bool,
    }

    impl CwdConfigBackupGuard {
        fn hide() -> Self {
            let cwd_file = Path::new(AppConfig::CONFIG_FILE);
            let backup_path = PathBuf::from(format!("{}.test_bak_{}", AppConfig::CONFIG_FILE, std::process::id()));
            let had_file = cwd_file.exists();
            if had_file {
                let _ = fs::rename(cwd_file, &backup_path);
            }
            Self { backup_path, had_file }
        }
    }

    impl Drop for CwdConfigBackupGuard {
        fn drop(&mut self) {
            let cwd_file = Path::new(AppConfig::CONFIG_FILE);
            if self.had_file {
                let _ = fs::rename(&self.backup_path, cwd_file);
            } else if cwd_file.exists() {
                let _ = fs::remove_file(cwd_file);
            }
            let _ = fs::remove_file(&self.backup_path);
        }
    }

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
        assert!(config.show_cooling);
        assert!(config.show_storage);
        assert!(config.show_network);
        assert!(config.show_processes);
    }

    #[test]
    fn test_reset_to_defaults() {
        let mut config = AppConfig {
            theme: ThemeMode::Vintage,
            refresh_interval_ms: 500,
            window_width: 800.0,
            window_height: 400.0,
            show_cooling: false,
            show_storage: false,
            show_network: false,
            show_processes: false,
            auto_hide_empty_cards: false,
        };
        config.reset_to_defaults();
        assert_eq!(config, AppConfig::default());
    }

    #[test]
    fn test_backward_compatibility_loading_legacy_configs() {
        // Legacy JSON format from previous versions with old theme variants and no secondary visibility fields
        let legacy_json = r#"{
            "theme": "Nebula",
            "refresh_interval_ms": 2000,
            "window_width": 1400.0,
            "window_height": 700.0
        }"#;

        let parsed: AppConfig = serde_json::from_str(legacy_json).expect("Should deserialize legacy JSON");
        assert_eq!(parsed.theme, ThemeMode::Vintage, "Legacy theme must default to Vintage");
        assert_eq!(parsed.refresh_interval_ms, 2000);
        assert_eq!(parsed.window_width, 1400.0);
        assert_eq!(parsed.window_height, 700.0);
        assert!(parsed.show_cooling, "Missing secondary fields must default to true");
        assert!(parsed.show_storage, "Missing secondary fields must default to true");
        assert!(parsed.show_network, "Missing secondary fields must default to true");
        assert!(parsed.show_processes, "Missing secondary fields must default to true");
    }

    #[test]
    fn test_backward_compatibility_partial_and_empty_json() {
        let empty_json = "{}";
        let parsed: AppConfig = serde_json::from_str(empty_json).expect("Empty JSON must default all fields");
        assert_eq!(parsed, AppConfig::default());

        let partial_json = r#"{"refresh_interval_ms": 500, "show_cooling": false}"#;
        let parsed_partial: AppConfig = serde_json::from_str(partial_json).expect("Partial JSON must deserialize");
        assert_eq!(parsed_partial.refresh_interval_ms, 500);
        assert!(!parsed_partial.show_cooling);
        assert!(parsed_partial.show_storage);
        assert!(parsed_partial.show_network);
        assert!(parsed_partial.show_processes);
        assert_eq!(parsed_partial.theme, ThemeMode::Vintage);
    }

    #[test]
    fn test_config_dir_and_path_resolution() {
        let _lock = CONFIG_TEST_MUTEX.lock().unwrap();
        let temp_dir = unique_temp_dir("nexus_test_appdata_res");
        let _env_guard = EnvGuard::set("APPDATA", &temp_dir);

        let expected_dir = temp_dir.join("NexusSystems");
        let expected_path = expected_dir.join("nexus_settings.json");

        assert_eq!(AppConfig::config_dir(), expected_dir);
        assert_eq!(AppConfig::config_path(), expected_path);

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_config_dir_fallback_when_appdata_unset() {
        let _lock = CONFIG_TEST_MUTEX.lock().unwrap();
        let _env_guard = EnvGuard::remove("APPDATA");

        assert_eq!(AppConfig::config_dir(), PathBuf::from("."));
        assert_eq!(AppConfig::config_path(), PathBuf::from(".").join("nexus_settings.json"));
    }

    #[test]
    fn test_directory_auto_creation_and_atomic_save_in_appdata() {
        let _lock = CONFIG_TEST_MUTEX.lock().unwrap();
        let temp_dir = unique_temp_dir("nexus_test_autocreate");
        let _env_guard = EnvGuard::set("APPDATA", &temp_dir);

        let expected_dir = AppConfig::config_dir();
        assert!(!expected_dir.exists(), "Target NexusSystems dir should not exist before save");

        let test_config = AppConfig {
            theme: ThemeMode::Vintage,
            refresh_interval_ms: 750,
            window_width: 1350.0,
            window_height: 650.0,
            show_cooling: false,
            show_storage: true,
            show_network: false,
            show_processes: true,
            auto_hide_empty_cards: true,
        };

        test_config.save();

        assert!(expected_dir.exists(), "NexusSystems dir should have been auto-created");
        assert!(AppConfig::config_path().exists(), "Configuration file should exist in %APPDATA%");
        assert!(
            !expected_dir.join("nexus_settings.json.tmp").exists(),
            "Temporary file must not linger after save()"
        );

        let loaded = AppConfig::load();
        assert_eq!(loaded, test_config);

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_load_from_appdata_precedence_over_cwd() {
        let _lock = CONFIG_TEST_MUTEX.lock().unwrap();
        let _cwd_guard = CwdConfigBackupGuard::hide();

        let temp_dir = unique_temp_dir("nexus_test_precedence");
        let _env_guard = EnvGuard::set("APPDATA", &temp_dir);

        // Config in %APPDATA%
        let appdata_config = AppConfig {
            theme: ThemeMode::Vintage,
            refresh_interval_ms: 500,
            window_width: 1500.0,
            window_height: 750.0,
            show_cooling: false,
            show_storage: true,
            show_network: true,
            show_processes: true,
            auto_hide_empty_cards: true,
        };
        appdata_config.save();

        // Config in CWD
        let cwd_config = AppConfig {
            theme: ThemeMode::Vintage,
            refresh_interval_ms: 2000,
            window_width: 800.0,
            window_height: 500.0,
            show_cooling: true,
            show_storage: false,
            show_network: false,
            show_processes: false,
            auto_hide_empty_cards: true,
        };
        let cwd_path = Path::new(AppConfig::CONFIG_FILE);
        cwd_config.save_to(cwd_path).unwrap();

        // load() must prioritize %APPDATA%
        let loaded = AppConfig::load();
        assert_eq!(loaded, appdata_config);

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_backward_compatibility_fallback_to_cwd() {
        let _lock = CONFIG_TEST_MUTEX.lock().unwrap();
        let _cwd_guard = CwdConfigBackupGuard::hide();

        let temp_dir = unique_temp_dir("nexus_test_backward_compat");
        let _env_guard = EnvGuard::set("APPDATA", &temp_dir);

        // Ensure %APPDATA% file does not exist
        assert!(!AppConfig::config_path().exists());

        // Put configuration in CWD
        let cwd_config = AppConfig {
            theme: ThemeMode::Vintage,
            refresh_interval_ms: 1200,
            window_width: 1100.0,
            window_height: 550.0,
            show_cooling: true,
            show_storage: true,
            show_network: false,
            show_processes: true,
            auto_hide_empty_cards: true,
        };
        let cwd_path = Path::new(AppConfig::CONFIG_FILE);
        cwd_config.save_to(cwd_path).unwrap();

        // load() should fall back to CWD configuration
        let loaded = AppConfig::load();
        assert_eq!(loaded, cwd_config);

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_save_and_load_fallback_when_appdata_unset() {
        let _lock = CONFIG_TEST_MUTEX.lock().unwrap();
        let _cwd_guard = CwdConfigBackupGuard::hide();
        let _env_guard = EnvGuard::remove("APPDATA");

        let test_config = AppConfig {
            theme: ThemeMode::Vintage,
            refresh_interval_ms: 1500,
            window_width: 1280.0,
            window_height: 720.0,
            show_cooling: true,
            show_storage: false,
            show_network: true,
            show_processes: false,
            auto_hide_empty_cards: true,
        };

        test_config.save();

        let cwd_file = Path::new(AppConfig::CONFIG_FILE);
        assert!(cwd_file.exists(), "Should save to CWD when APPDATA is unset");

        let loaded = AppConfig::load();
        assert_eq!(loaded, test_config);
    }

    #[test]
    fn test_load_default_when_both_missing() {
        let _lock = CONFIG_TEST_MUTEX.lock().unwrap();
        let _cwd_guard = CwdConfigBackupGuard::hide();

        let temp_dir = unique_temp_dir("nexus_test_missing");
        let _env_guard = EnvGuard::set("APPDATA", &temp_dir);

        assert!(!AppConfig::config_path().exists());
        assert!(!Path::new(AppConfig::CONFIG_FILE).exists());

        assert_eq!(AppConfig::load(), AppConfig::default());

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_load_corrupted_appdata_returns_default() {
        let _lock = CONFIG_TEST_MUTEX.lock().unwrap();
        let _cwd_guard = CwdConfigBackupGuard::hide();

        let temp_dir = unique_temp_dir("nexus_test_corrupt_appdata");
        let _env_guard = EnvGuard::set("APPDATA", &temp_dir);

        let config_path = AppConfig::config_path();
        if let Some(parent) = config_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        fs::write(&config_path, b"{ broken json ...").unwrap();

        assert_eq!(AppConfig::load(), AppConfig::default());

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_save_to_creates_valid_file_and_cleans_tmp() {
        let temp_dir = unique_temp_dir("nexus_test_save");
        let target_path = temp_dir.join("nexus_settings.json");
        let tmp_path = temp_dir.join("nexus_settings.json.tmp");

        let config = AppConfig {
            theme: ThemeMode::Vintage,
            refresh_interval_ms: 500,
            window_width: 1400.0,
            window_height: 700.0,
            show_cooling: false,
            show_storage: true,
            show_network: true,
            show_processes: false,
            auto_hide_empty_cards: true,
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
            theme: ThemeMode::Vintage,
            refresh_interval_ms: 2000,
            window_width: 1000.0,
            window_height: 600.0,
            show_cooling: true,
            show_storage: true,
            show_network: true,
            show_processes: true,
            auto_hide_empty_cards: true,
        };
        initial_config.save_to(&target_path).unwrap();
        assert_eq!(AppConfig::load_from(&target_path), Some(initial_config));

        let updated_config = AppConfig {
            theme: ThemeMode::Vintage,
            refresh_interval_ms: 250,
            window_width: 1600.0,
            window_height: 900.0,
            show_cooling: false,
            show_storage: false,
            show_network: false,
            show_processes: false,
            auto_hide_empty_cards: false,
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
    fn test_load_from_nonexistent_returns_none() {
        let temp_dir = unique_temp_dir("nexus_test_nonexistent");
        let target_path = temp_dir.join("non_existent_config.json");

        assert_eq!(AppConfig::load_from(&target_path), None);
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_load_from_corrupted_json_returns_none() {
        let temp_dir = unique_temp_dir("nexus_test_corrupt");
        let target_path = temp_dir.join("nexus_settings.json");

        fs::write(&target_path, b"{ not valid json ...").unwrap();

        assert_eq!(AppConfig::load_from(&target_path), None);
        let _ = fs::remove_dir_all(&temp_dir);
    }
}

use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::theme::ThemeMode;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    const CONFIG_FILE: &'static str = "nexus_settings.json";

    pub fn load() -> Self {
        if Path::new(Self::CONFIG_FILE).exists() {
            if let Ok(contents) = fs::read_to_string(Self::CONFIG_FILE) {
                if let Ok(config) = serde_json::from_str::<AppConfig>(&contents) {
                    return config;
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) {
        if let Ok(serialized) = serde_json::to_string_pretty(self) {
            let _ = fs::write(Self::CONFIG_FILE, serialized);
        }
    }
}

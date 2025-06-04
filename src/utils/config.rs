use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserConfig {
    pub search_engine: SearchEngineConfig,
    pub privacy: PrivacyConfig,
    pub ui: UiConfig,
    pub storage: StorageConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchEngineConfig {
    pub default_provider: String,
    pub fourget_instance: String,
    pub fallback_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyConfig {
    pub auto_clear_cookies_days: u32,
    pub tracking_protection: bool,
    pub ad_blocking: bool,
    pub https_upgrade: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    pub font_family: String,
    pub theme: String,
    pub show_bookmarks_bar: bool,
    pub tab_position: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub data_dir: PathBuf,
    pub max_history_days: u32,
    pub max_cache_size_mb: u32,
}

impl Default for BrowserConfig {
    fn default() -> Self {
        let data_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("bagel-browser");

        Self {
            search_engine: SearchEngineConfig {
                default_provider: "4get".to_string(),
                fourget_instance: "https://4get.ca".to_string(),
                fallback_enabled: true,
            },
            privacy: PrivacyConfig {
                auto_clear_cookies_days: 30,
                tracking_protection: true,
                ad_blocking: true,
                https_upgrade: true,
            },
            ui: UiConfig {
                font_family: "Ubuntu".to_string(),
                theme: "light".to_string(),
                show_bookmarks_bar: true,
                tab_position: "top".to_string(),
            },
            storage: StorageConfig {
                data_dir,
                max_history_days: 90,
                max_cache_size_mb: 500,
            },
        }
    }
}

impl BrowserConfig {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        
        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: BrowserConfig = serde_json::from_str(&content)?;
            Ok(config)
        } else {
            let config = Self::default();
            config.save()?;
            Ok(config)
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&config_path, content)?;
        Ok(())
    }

    fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;
        Ok(config_dir.join("bagel-browser").join("config.json"))
    }

    pub fn data_dir(&self) -> &PathBuf {
        &self.storage.data_dir
    }

    pub fn ensure_directories(&self) -> Result<()> {
        std::fs::create_dir_all(&self.storage.data_dir)?;
        std::fs::create_dir_all(self.storage.data_dir.join("userscripts"))?;
        std::fs::create_dir_all(self.storage.data_dir.join("userstyles"))?;
        Ok(())
    }
}
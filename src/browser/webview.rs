use crate::browser::{SecurityManager, TabManager};
use crate::utils::{log_navigation, log_security_event, BrowserConfig};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationRequest {
    pub url: String,
    pub tab_id: Option<Uuid>,
    pub is_new_tab: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationEvent {
    pub tab_id: Uuid,
    pub url: String,
    pub title: String,
    pub can_go_back: bool,
    pub can_go_forward: bool,
}

pub struct WebViewManager {
    tab_manager: Arc<Mutex<TabManager>>,
    security_manager: SecurityManager,
    config: BrowserConfig,
}

impl WebViewManager {
    pub fn new(config: BrowserConfig) -> Self {
        let security_manager = SecurityManager::new(config.clone());

        Self {
            tab_manager: Arc::new(Mutex::new(TabManager::new())),
            security_manager,
            config,
        }
    }

    pub fn navigate(&self, request: NavigationRequest) -> Result<Uuid> {
        let url = self.process_url(&request.url)?;

        // Security check
        if self.security_manager.should_block_request(&url) {
            log_security_event("NAVIGATION_BLOCKED", &url);
            return Err(anyhow::anyhow!("Navigation blocked by security policy"));
        }

        // Suggest HTTPS upgrade if available
        // just to warn users that if they navigate to an http site it may be insecure
        // so we can try to redirect them to https
        let final_url = self
            .security_manager
            .suggest_https_upgrade(&url)
            .unwrap_or(url);

        log_navigation(&final_url);

        let tab_id = if request.is_new_tab || request.tab_id.is_none() {
            let mut tab_manager = self.tab_manager.lock().unwrap();
            tab_manager.create_tab(final_url.clone(), None)
        } else {
            let tab_id = request.tab_id.unwrap();
            let mut tab_manager = self.tab_manager.lock().unwrap();
            tab_manager.update_tab_url(tab_id, final_url.clone());
            tab_manager.set_tab_loading(tab_id, true);
            tab_id
        };

        Ok(tab_id)
    }

    fn process_url(&self, input: &str) -> Result<String> {
        let trimmed = input.trim();

        // Check if it's already a valid URL
        if url::Url::parse(trimmed).is_ok() {
            return Ok(trimmed.to_string());
        }

        // Check if it looks like a domain
        if trimmed.contains('.') && !trimmed.contains(' ') {
            let url = if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
                trimmed.to_string()
            } else {
                format!("https://{}", trimmed)
            };

            if url::Url::parse(&url).is_ok() {
                return Ok(url);
            }
        }

        // Treat as search query
        self.create_search_url(trimmed)
    }

    fn create_search_url(&self, query: &str) -> Result<String> {
        let encoded_query = urlencoding::encode(query);
        let search_url = format!(
            "{}/web?s={}",
            self.config.search_engine.fourget_instance, encoded_query
        );
        Ok(search_url)
    }

    pub fn go_back(&self, tab_id: Uuid) -> Result<()> {
        let tab_manager = self.tab_manager.lock().unwrap();
        if let Some(tab) = tab_manager.get_tab(tab_id) {
            if tab.can_go_back {
                // erm this is just a dummy implementation
                // gotta figure out navigation hehe
                log::info!("Navigate back for tab: {}", tab_id);
            }
        }
        Ok(())
    }

    pub fn go_forward(&self, tab_id: Uuid) -> Result<()> {
        let tab_manager = self.tab_manager.lock().unwrap();
        if let Some(tab) = tab_manager.get_tab(tab_id) {
            if tab.can_go_forward {
                // see comment above
                log::info!("Navigate forward for tab: {}", tab_id);
            }
        }
        Ok(())
    }

    pub fn reload(&self, tab_id: Uuid) -> Result<()> {
        // reloader i barely know her.
        log::info!("Reload tab: {}", tab_id);
        Ok(())
    }

    pub fn create_new_tab(&self, url: Option<String>) -> Result<Uuid> {
        let mut tab_manager = self.tab_manager.lock().unwrap();
        let default_url = url.unwrap_or_else(|| "bagel://home".to_string());
        Ok(tab_manager.create_tab(default_url, None))
    }

    pub fn close_tab(&self, tab_id: Uuid) -> Result<bool> {
        let mut tab_manager = self.tab_manager.lock().unwrap();
        Ok(tab_manager.close_tab(tab_id))
    }

    pub fn set_active_tab(&self, tab_id: Uuid) -> Result<bool> {
        let mut tab_manager = self.tab_manager.lock().unwrap();
        Ok(tab_manager.set_active_tab(tab_id))
    }

    pub fn get_tab_manager(&self) -> Arc<Mutex<TabManager>> {
        Arc::clone(&self.tab_manager)
    }

    pub fn update_tab_info(
        &self,
        tab_id: Uuid,
        title: String,
        url: String,
        can_go_back: bool,
        can_go_forward: bool,
    ) -> Result<()> {
        let mut tab_manager = self.tab_manager.lock().unwrap();
        tab_manager.update_tab_title(tab_id, title);
        tab_manager.update_tab_url(tab_id, url);
        tab_manager.set_tab_navigation_state(tab_id, can_go_back, can_go_forward);
        tab_manager.set_tab_loading(tab_id, false);
        Ok(())
    }
}

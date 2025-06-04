// most of the code is like "dead code" but you can ignore it just cuz i might change it later
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tab {
    pub id: Uuid,
    pub title: String,
    pub url: String,
    pub favicon: Option<String>,
    pub is_loading: bool,
    pub can_go_back: bool,
    pub can_go_forward: bool,
    pub created_at: DateTime<Utc>,
    pub last_accessed: DateTime<Utc>,
    pub is_pinned: bool,
    pub is_muted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabManager {
    tabs: HashMap<Uuid, Tab>,
    active_tab_id: Option<Uuid>,
    tab_order: Vec<Uuid>,
}

impl Default for TabManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TabManager {
    pub fn new() -> Self {
        Self {
            tabs: HashMap::new(),
            active_tab_id: None,
            tab_order: Vec::new(),
        }
    }

    pub fn create_tab(&mut self, url: String, title: Option<String>) -> Uuid {
        let tab_id = Uuid::new_v4();
        let now = Utc::now();

        let tab = Tab {
            id: tab_id,
            title: title.unwrap_or_else(|| "New Tab".to_string()),
            url,
            favicon: None,
            is_loading: false,
            can_go_back: false,
            can_go_forward: false,
            created_at: now,
            last_accessed: now,
            is_pinned: false,
            is_muted: false,
        };

        self.tabs.insert(tab_id, tab);
        self.tab_order.push(tab_id);

        if self.active_tab_id.is_none() {
            self.active_tab_id = Some(tab_id);
        }

        tab_id
    }

    pub fn close_tab(&mut self, tab_id: Uuid) -> bool {
        if let Some(index) = self.tab_order.iter().position(|&id| id == tab_id) {
            self.tab_order.remove(index);
            self.tabs.remove(&tab_id);

            if self.active_tab_id == Some(tab_id) {
                self.active_tab_id = if index < self.tab_order.len() {
                    Some(self.tab_order[index])
                } else if !self.tab_order.is_empty() {
                    Some(self.tab_order[index - 1])
                } else {
                    None
                };
            }
            true
        } else {
            false
        }
    }

    pub fn set_active_tab(&mut self, tab_id: Uuid) -> bool {
        if self.tabs.contains_key(&tab_id) {
            self.active_tab_id = Some(tab_id);
            if let Some(tab) = self.tabs.get_mut(&tab_id) {
                tab.last_accessed = Utc::now();
            }
            true
        } else {
            false
        }
    }

    pub fn get_active_tab(&self) -> Option<&Tab> {
        self.active_tab_id.and_then(|id| self.tabs.get(&id))
    }

    pub fn get_active_tab_mut(&mut self) -> Option<&mut Tab> {
        self.active_tab_id.and_then(|id| self.tabs.get_mut(&id))
    }

    pub fn get_tab(&self, tab_id: Uuid) -> Option<&Tab> {
        self.tabs.get(&tab_id)
    }

    pub fn get_tab_mut(&mut self, tab_id: Uuid) -> Option<&mut Tab> {
        self.tabs.get_mut(&tab_id)
    }

    pub fn get_all_tabs(&self) -> Vec<&Tab> {
        self.tab_order
            .iter()
            .filter_map(|id| self.tabs.get(id))
            .collect()
    }

    pub fn update_tab_url(&mut self, tab_id: Uuid, url: String) {
        if let Some(tab) = self.tabs.get_mut(&tab_id) {
            tab.url = url;
            tab.last_accessed = Utc::now();
        }
    }

    pub fn update_tab_title(&mut self, tab_id: Uuid, title: String) {
        if let Some(tab) = self.tabs.get_mut(&tab_id) {
            tab.title = title;
        }
    }

    pub fn set_tab_loading(&mut self, tab_id: Uuid, is_loading: bool) {
        if let Some(tab) = self.tabs.get_mut(&tab_id) {
            tab.is_loading = is_loading;
        }
    }

    pub fn set_tab_navigation_state(
        &mut self,
        tab_id: Uuid,
        can_go_back: bool,
        can_go_forward: bool,
    ) {
        if let Some(tab) = self.tabs.get_mut(&tab_id) {
            tab.can_go_back = can_go_back;
            tab.can_go_forward = can_go_forward;
        }
    }

    pub fn pin_tab(&mut self, tab_id: Uuid) {
        if let Some(tab) = self.tabs.get_mut(&tab_id) {
            tab.is_pinned = true;
        }
    }

    pub fn unpin_tab(&mut self, tab_id: Uuid) {
        if let Some(tab) = self.tabs.get_mut(&tab_id) {
            tab.is_pinned = false;
        }
    }

    pub fn move_tab(&mut self, from_index: usize, to_index: usize) -> bool {
        if from_index < self.tab_order.len() && to_index < self.tab_order.len() {
            let tab_id = self.tab_order.remove(from_index);
            self.tab_order.insert(to_index, tab_id);
            true
        } else {
            false
        }
    }

    pub fn tab_count(&self) -> usize {
        self.tabs.len()
    }

    pub fn has_tabs(&self) -> bool {
        !self.tabs.is_empty()
    }

    pub fn get_recently_closed(&self) -> Vec<Tab> {
        Vec::new()
    }
}

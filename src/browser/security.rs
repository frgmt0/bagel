use crate::utils::{log_security_event, BrowserConfig};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityLevel {
    pub is_secure: bool,
    pub certificate_valid: bool,
    pub mixed_content: bool,
    pub tracking_blocked: bool,
    pub ads_blocked: u32,
}

#[derive(Debug, Clone)]
pub struct SecurityManager {
    config: BrowserConfig,
    blocked_domains: Vec<String>,
    tracking_patterns: Vec<String>,
}

impl SecurityManager {
    pub fn new(config: BrowserConfig) -> Self {
        Self {
            config,
            blocked_domains: Self::load_blocked_domains(),
            tracking_patterns: Self::load_tracking_patterns(),
        }
    }

    fn load_blocked_domains() -> Vec<String> {
        vec![
            "doubleclick.net".to_string(),
            "googleadservices.com".to_string(),
            "googlesyndication.com".to_string(),
            "google-analytics.com".to_string(),
            "facebook.com".to_string(),
            "connect.facebook.net".to_string(),
            "scorecardresearch.com".to_string(),
            "outbrain.com".to_string(),
            "taboola.com".to_string(),
        ]
    }

    fn load_tracking_patterns() -> Vec<String> {
        vec![
            r".*\.ads\..*".to_string(),
            r".*\.analytics\..*".to_string(),
            r".*\.tracker\..*".to_string(),
            r".*\.telemetry\..*".to_string(),
        ]
    }

    pub fn check_url_security(&self, url: &str) -> Result<SecurityLevel> {
        let parsed_url = Url::parse(url)?;

        let is_secure = parsed_url.scheme() == "https";
        let mixed_content = false; // dummy mixed content detection; will do later

        if !is_secure && self.config.privacy.https_upgrade {
            log_security_event("HTTPS_UPGRADE_SUGGESTED", url);
        }

        let security_level = SecurityLevel {
            is_secure,
            certificate_valid: true, // dummy certificate validation
            mixed_content,
            tracking_blocked: false,
            ads_blocked: 0,
        };

        Ok(security_level)
    }

    pub fn should_block_request(&self, url: &str) -> bool {
        if !self.config.privacy.ad_blocking && !self.config.privacy.tracking_protection {
            return false;
        }

        if let Ok(parsed_url) = Url::parse(url) {
            if let Some(domain) = parsed_url.domain() {
                // Check against blocked domains
                if self.config.privacy.ad_blocking && self.is_domain_blocked(domain) {
                    log_security_event("AD_BLOCKED", url);
                    return true;
                }

                // Check against tracking patterns
                if self.config.privacy.tracking_protection && self.is_tracking_request(url) {
                    log_security_event("TRACKER_BLOCKED", url);
                    return true;
                }
            }
        }

        false
    }

    fn is_domain_blocked(&self, domain: &str) -> bool {
        self.blocked_domains
            .iter()
            .any(|blocked| domain == blocked || domain.ends_with(&format!(".{}", blocked)))
    }

    fn is_tracking_request(&self, url: &str) -> bool {
        self.tracking_patterns.iter().any(|_pattern| {
            // literally need to do regex technically
            // but this is good enough for now
            url.contains("analytics")
                || url.contains("tracker")
                || url.contains("telemetry")
                || url.contains("/ads/")
        })
    }

    pub fn suggest_https_upgrade(&self, url: &str) -> Option<String> {
        if self.config.privacy.https_upgrade {
            if let Ok(mut parsed_url) = Url::parse(url) {
                if parsed_url.scheme() == "http" {
                    parsed_url.set_scheme("https").ok()?;
                    return Some(parsed_url.to_string());
                }
            }
        }
        None
    }

    pub fn validate_certificate(&self, _url: &str) -> Result<bool> {
        // Implement proper certificate validation later
        // This would involve checking the certificate chain, validity dates, etc.
        Ok(true)
    }

    pub fn check_malicious_site(&self, _url: &str) -> Result<bool> {
        // Implement malicious site checking later
        // maybe theres an API for this?
        // or just use Google Safe Browsing API or similar idk
        Ok(false)
    }

    pub fn get_content_security_policy(&self) -> String {
        "default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline';"
            .to_string()
    }
}

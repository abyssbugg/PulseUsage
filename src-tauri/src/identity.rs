//! Application identity helpers.
//!
//! Centralizes product-name and external-link strings so a future rename
//! (e.g. PulseUsage -> PulseBar) only requires touching `tauri.conf.json`
//! plus this module. Menu and UI code consume these helpers instead of
//! hardcoding the product name.

// PR-A1 intentionally introduces a broader identity API before every call site
// consumes it. Later PulseBar rename PRs will migrate additional surfaces onto
// these constants without changing bundle IDs or data paths.
#![allow(dead_code)]

use tauri::AppHandle;

pub const PRODUCT_DISPLAY_NAME: &str = "PulseBar";
pub const INTERNAL_PROJECT_NAME: &str = "pulseusage";
pub const MARKETING_NAME: &str = "PulseBar";

pub const GITHUB_OWNER: &str = "abyssbugg";
pub const GITHUB_REPO: &str = "PulseUsage";
pub const GITHUB_REPOSITORY: &str = "abyssbugg/PulseUsage";
pub const REPOSITORY_URL: &str = "https://github.com/abyssbugg/PulseUsage";
pub const ISSUES_URL: &str = "https://github.com/abyssbugg/PulseUsage/issues";

/// GitHub releases page used by the "Download Latest Release…" menu item.
/// Update this constant on migration or fork.
pub const RELEASES_URL: &str = "https://github.com/abyssbugg/PulseUsage/releases";
pub const DOCUMENTATION_URL: &str = "https://github.com/abyssbugg/PulseUsage#readme";
pub const SUPPORT_URL: &str = ISSUES_URL;

/// Canonical app display name, sourced from `tauri.conf.json` `productName`.
///
/// Borrows from the `AppHandle`; the returned `&str` is valid as long as the
/// handle is borrowed. Centralized so a future product rename only needs
/// `tauri.conf.json` plus this module.
pub fn app_display_name(app: &AppHandle) -> &str {
    &app.package_info().name
}

pub fn release_api_url(tag: &str) -> String {
    format!("https://api.github.com/repos/{GITHUB_REPOSITORY}/releases/tags/{tag}")
}

pub fn pull_request_url(number: u64) -> String {
    format!("{REPOSITORY_URL}/pull/{number}")
}

pub fn commit_url(hash: &str) -> String {
    format!("{REPOSITORY_URL}/commit/{hash}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn releases_url_points_at_github_releases() {
        assert!(
            RELEASES_URL.starts_with("https://github.com/"),
            "expected HTTPS GitHub URL, got {RELEASES_URL}"
        );
        assert!(
            RELEASES_URL.ends_with("/releases"),
            "expected URL to end with /releases, got {RELEASES_URL}"
        );
    }

    #[test]
    fn display_identity_is_pulsebar_while_repository_identity_remains_pulseusage() {
        assert_eq!(PRODUCT_DISPLAY_NAME, "PulseBar");
        assert_eq!(INTERNAL_PROJECT_NAME, "pulseusage");
        assert_eq!(MARKETING_NAME, "PulseBar");
        assert_eq!(GITHUB_OWNER, "abyssbugg");
        assert_eq!(GITHUB_REPO, "PulseUsage");
        assert_eq!(GITHUB_REPOSITORY, "abyssbugg/PulseUsage");
        assert_eq!(REPOSITORY_URL, "https://github.com/abyssbugg/PulseUsage");
        assert_eq!(ISSUES_URL, "https://github.com/abyssbugg/PulseUsage/issues");
        assert_eq!(RELEASES_URL, "https://github.com/abyssbugg/PulseUsage/releases");
        assert_eq!(DOCUMENTATION_URL, "https://github.com/abyssbugg/PulseUsage#readme");
        assert_eq!(SUPPORT_URL, ISSUES_URL);
        assert_eq!(release_api_url("v0.7.0-rc.1"), "https://api.github.com/repos/abyssbugg/PulseUsage/releases/tags/v0.7.0-rc.1");
        assert_eq!(pull_request_url(52), "https://github.com/abyssbugg/PulseUsage/pull/52");
        assert_eq!(commit_url("abc123"), "https://github.com/abyssbugg/PulseUsage/commit/abc123");
    }
}

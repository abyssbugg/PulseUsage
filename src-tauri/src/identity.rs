//! Application identity helpers.
//!
//! Centralizes product-name and external-link strings so a future rename
//! (e.g. PulseUsage -> PulseBar) only requires touching `tauri.conf.json`
//! plus this module. Menu and UI code consume these helpers instead of
//! hardcoding the product name.

use tauri::AppHandle;

/// Canonical app display name, sourced from `tauri.conf.json` `productName`.
///
/// Borrows from the `AppHandle`; the returned `&str` is valid as long as the
/// handle is borrowed. Centralized so a future product rename only needs
/// `tauri.conf.json` plus this module.
pub fn app_display_name(app: &AppHandle) -> &str {
    &app.package_info().name
}

/// GitHub releases page used by the "Download Latest Release…" menu item.
/// Update this constant on migration or fork.
pub const RELEASES_URL: &str = "https://github.com/abyssbugg/PulseUsage/releases";

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
}

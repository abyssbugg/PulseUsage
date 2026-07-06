//! Host API capability enforcement model.
//!
//! Each plugin declares which `ctx.host.*` functions it needs via the
//! `hostCapabilities` array in `plugin.json` (schema v2). At injection time,
//! `inject_host_api_with_deadline` checks the `HostCapabilitySet` and only
//! injects the modules the plugin is authorized to use.
//!
//! Schema v1 plugins (no `hostCapabilities` field) fall back to the
//! compatibility map (`infer_v1_capabilities`), which grants each legacy
//! plugin exactly the capabilities it currently uses — preserving backward
//! compatibility with zero behavior changes.
//!
//! See: Program 2 Design Review (Program Transition document).

use std::collections::HashSet;

/// A single host API capability. Each corresponds to one or more
/// `ctx.host.*` JS functions that a plugin may call.
///
/// `Log` is intentionally absent — logging is always available to all
/// plugins (no capability needed).
///
/// NOTE: Items in this module are not yet wired into the orchestrator.
/// PR2 of Program 2 will integrate `HostCapabilitySet` into
/// `inject_host_api_with_deadline`. The `allow(dead_code)` suppressions
/// are removed at that point.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(clippy::enum_variant_names)]
pub enum HostCapability {
    // Filesystem
    FsRead,
    FsWrite,
    FsListDir,

    // Keychain
    KeychainRead,
    KeychainWrite,
    KeychainDelete,

    // Network
    HttpRequest,
    /// Explicit opt-in for `dangerouslyIgnoreTls` on localhost.
    /// `HttpRequest` alone does NOT grant TLS bypass.
    HttpDangerousLocalhostTls,

    // Data
    SqliteQuery,
    SqliteExec,
    PlistRead,

    // Subprocess
    CcusageQuery,
    LsDiscover,

    // Crypto
    CryptoAes,
    CryptoSha,

    // Environment
    EnvRead,
}

impl HostCapability {
    /// The JSON string used in `plugin.json` `hostCapabilities` arrays.
    #[allow(dead_code)]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::FsRead => "fsRead",
            Self::FsWrite => "fsWrite",
            Self::FsListDir => "fsListDir",
            Self::KeychainRead => "keychainRead",
            Self::KeychainWrite => "keychainWrite",
            Self::KeychainDelete => "keychainDelete",
            Self::HttpRequest => "httpRequest",
            Self::HttpDangerousLocalhostTls => "httpDangerousLocalhostTls",
            Self::SqliteQuery => "sqliteQuery",
            Self::SqliteExec => "sqliteExec",
            Self::PlistRead => "plistRead",
            Self::CcusageQuery => "ccusageQuery",
            Self::LsDiscover => "lsDiscover",
            Self::CryptoAes => "cryptoAes",
            Self::CryptoSha => "cryptoSha",
            Self::EnvRead => "envRead",
        }
    }
}

/// A validated set of host API capabilities for a single plugin.
///
/// Built from the `hostCapabilities` array in `plugin.json` (schema v2)
/// or inferred from the plugin ID via the v1 compatibility map.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct HostCapabilitySet {
    capabilities: HashSet<HostCapability>,
}

#[allow(dead_code)]
impl HostCapabilitySet {
    /// Build a `HostCapabilitySet` from a list of capability strings
    /// (as they appear in `plugin.json` `hostCapabilities`).
    ///
    /// Unknown strings are silently dropped (fail-safe — a typo in a
    /// capability name means the capability is not granted, not that
    /// the plugin fails to load).
    pub fn from_strings(strings: &[String]) -> Self {
        let capabilities = strings
            .iter()
            .filter_map(|s| Self::parse_capability(s))
            .collect();
        Self { capabilities }
    }

    /// Returns `true` if the set contains the given capability.
    pub fn contains(&self, cap: HostCapability) -> bool {
        self.capabilities.contains(&cap)
    }

    /// Returns `true` if the set is empty (plugin has no host API access
    /// beyond `log` and `utils`).
    pub fn is_empty(&self) -> bool {
        self.capabilities.is_empty()
    }

    /// Number of capabilities declared.
    pub fn len(&self) -> usize {
        self.capabilities.len()
    }

    /// Returns a set containing every capability. For test fixtures that
    /// need to exercise the full host API surface regardless of plugin ID.
    #[cfg(test)]
    pub fn all() -> Self {
        use HostCapability::*;
        let capabilities = [
            FsRead, FsWrite, FsListDir,
            PlistRead,
            CryptoAes, CryptoSha,
            EnvRead,
            HttpRequest,
            KeychainRead, KeychainWrite, KeychainDelete,
            SqliteQuery, SqliteExec,
            LsDiscover,
            CcusageQuery,
        ]
        .into_iter()
        .collect();
        Self { capabilities }
    }

    /// Parse a single capability string. Returns `None` for unknown strings.
    fn parse_capability(s: &str) -> Option<HostCapability> {
        let trimmed = s.trim();
        const ALL: &[HostCapability] = &[
            HostCapability::FsRead,
            HostCapability::FsWrite,
            HostCapability::FsListDir,
            HostCapability::KeychainRead,
            HostCapability::KeychainWrite,
            HostCapability::KeychainDelete,
            HostCapability::HttpRequest,
            HostCapability::HttpDangerousLocalhostTls,
            HostCapability::SqliteQuery,
            HostCapability::SqliteExec,
            HostCapability::PlistRead,
            HostCapability::CcusageQuery,
            HostCapability::LsDiscover,
            HostCapability::CryptoAes,
            HostCapability::CryptoSha,
            HostCapability::EnvRead,
        ];
        ALL.iter().copied().find(|cap| cap.as_str() == trimmed)
    }
}

/// Infer capabilities for a schema v1 plugin (no `hostCapabilities` field)
/// based on the plugin ID. This preserves backward compatibility — each
/// legacy plugin receives exactly the capabilities it currently uses.
///
/// Unknown plugin IDs receive an empty set (log + utils only).
#[allow(dead_code)]
pub fn infer_v1_capabilities(plugin_id: &str) -> HostCapabilitySet {
    let caps = match plugin_id {
        "amp" => vec![HostCapability::FsRead],
        "antigravity" => vec![
            HostCapability::FsRead,
            HostCapability::FsWrite,
            HostCapability::HttpRequest,
            HostCapability::KeychainRead,
            HostCapability::LsDiscover,
            HostCapability::SqliteQuery,
        ],
        "claude" => vec![
            HostCapability::CcusageQuery,
            HostCapability::CryptoSha,
            HostCapability::EnvRead,
            HostCapability::FsRead,
            HostCapability::FsWrite,
            HostCapability::KeychainRead,
            HostCapability::KeychainWrite,
        ],
        "codex" => vec![
            HostCapability::CcusageQuery,
            HostCapability::EnvRead,
            HostCapability::FsRead,
            HostCapability::FsWrite,
            HostCapability::KeychainRead,
            HostCapability::KeychainWrite,
        ],
        "copilot" => vec![
            HostCapability::FsRead,
            HostCapability::FsWrite,
            HostCapability::KeychainRead,
            HostCapability::KeychainWrite,
            HostCapability::KeychainDelete,
        ],
        "cursor" => vec![
            HostCapability::KeychainRead,
            HostCapability::KeychainWrite,
            HostCapability::SqliteQuery,
            HostCapability::SqliteExec,
        ],
        "devin" => vec![
            HostCapability::FsRead,
            HostCapability::HttpRequest,
            HostCapability::SqliteQuery,
        ],
        "factory" => vec![
            HostCapability::CryptoAes,
            HostCapability::FsRead,
            HostCapability::FsWrite,
            HostCapability::KeychainRead,
            HostCapability::KeychainWrite,
        ],
        "grok" => vec![HostCapability::FsRead, HostCapability::FsWrite],
        "jetbrains-ai-assistant" => vec![
            HostCapability::FsRead,
            HostCapability::FsListDir,
        ],
        "kimi" => vec![HostCapability::FsRead, HostCapability::FsWrite],
        "kiro" => vec![
            HostCapability::FsRead,
            HostCapability::FsListDir,
            HostCapability::FsWrite,
            HostCapability::SqliteQuery,
        ],
        "minimax" => vec![HostCapability::EnvRead],
        "opencode-go" => vec![HostCapability::FsRead, HostCapability::SqliteQuery],
        "perplexity" => vec![HostCapability::FsRead, HostCapability::SqliteQuery],
        "synthetic" => vec![HostCapability::EnvRead, HostCapability::FsRead],
        "warp" => vec![
            HostCapability::FsRead,
            HostCapability::PlistRead,
            HostCapability::SqliteQuery,
        ],
        "zai" => vec![HostCapability::EnvRead],
        // Unknown plugins: no host API access (log + utils only)
        _ => vec![],
    };
    HostCapabilitySet {
        capabilities: caps.into_iter().collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_strings_parses_known_capabilities() {
        let set = HostCapabilitySet::from_strings(&[
            "fsRead".to_string(),
            "keychainWrite".to_string(),
            "sqliteExec".to_string(),
        ]);
        assert!(set.contains(HostCapability::FsRead));
        assert!(set.contains(HostCapability::KeychainWrite));
        assert!(set.contains(HostCapability::SqliteExec));
        assert!(!set.contains(HostCapability::FsWrite));
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn from_strings_drops_unknown_capabilities() {
        let set = HostCapabilitySet::from_strings(&[
            "fsRead".to_string(),
            "unknownCapability".to_string(),
            "  sqliteQuery  ".to_string(), // trimmed
        ]);
        assert!(set.contains(HostCapability::FsRead));
        assert!(set.contains(HostCapability::SqliteQuery));
        assert!(!set.contains(HostCapability::FsWrite));
        assert_eq!(set.len(), 2, "unknown capability should be dropped");
    }

    #[test]
    fn empty_set_has_no_access() {
        let set = HostCapabilitySet::default();
        assert!(set.is_empty());
        assert!(!set.contains(HostCapability::FsRead));
        assert!(!set.contains(HostCapability::KeychainRead));
    }

    #[test]
    fn all_capabilities_have_distinct_strings() {
        const ALL: &[HostCapability] = &[
            HostCapability::FsRead,
            HostCapability::FsWrite,
            HostCapability::FsListDir,
            HostCapability::KeychainRead,
            HostCapability::KeychainWrite,
            HostCapability::KeychainDelete,
            HostCapability::HttpRequest,
            HostCapability::HttpDangerousLocalhostTls,
            HostCapability::SqliteQuery,
            HostCapability::SqliteExec,
            HostCapability::PlistRead,
            HostCapability::CcusageQuery,
            HostCapability::LsDiscover,
            HostCapability::CryptoAes,
            HostCapability::CryptoSha,
            HostCapability::EnvRead,
        ];
        let mut seen = std::collections::HashSet::new();
        for cap in ALL {
            let s = cap.as_str();
            assert!(
                seen.insert(s),
                "duplicate capability string: {s}",
            );
        }
        assert_eq!(ALL.len(), 16, "expected 16 capabilities");
    }

    #[test]
    fn parse_capability_is_case_sensitive() {
        assert_eq!(
            HostCapabilitySet::parse_capability("fsRead"),
            Some(HostCapability::FsRead)
        );
        assert_eq!(
            HostCapabilitySet::parse_capability("FSREAD"),
            None,
            "capability strings are case-sensitive"
        );
        assert_eq!(HostCapabilitySet::parse_capability("fsread"), None);
    }

    #[test]
    fn infer_v1_capabilities_cursor_grants_sqlite_exec() {
        // Cursor is the only plugin that uses sqlite.exec (write capability)
        let set = infer_v1_capabilities("cursor");
        assert!(set.contains(HostCapability::SqliteExec));
        assert!(set.contains(HostCapability::SqliteQuery));
        assert!(set.contains(HostCapability::KeychainRead));
        assert!(set.contains(HostCapability::KeychainWrite));
    }

    #[test]
    fn infer_v1_capabilities_warp_grants_plist_read() {
        // Warp is one of the few plugins that uses plist.read
        let set = infer_v1_capabilities("warp");
        assert!(set.contains(HostCapability::PlistRead));
        assert!(set.contains(HostCapability::FsRead));
        assert!(set.contains(HostCapability::SqliteQuery));
        assert!(!set.contains(HostCapability::FsWrite));
    }

    #[test]
    fn infer_v1_capabilities_copilot_grants_keychain_delete() {
        // Copilot is the only plugin that calls deleteGenericPassword
        let set = infer_v1_capabilities("copilot");
        assert!(set.contains(HostCapability::KeychainDelete));
        assert!(set.contains(HostCapability::KeychainRead));
        assert!(set.contains(HostCapability::KeychainWrite));
    }

    #[test]
    fn infer_v1_capabilities_unknown_plugin_gets_nothing() {
        let set = infer_v1_capabilities("unknown-plugin");
        assert!(set.is_empty(), "unknown plugins get log + utils only");
    }

    #[test]
    fn infer_v1_capabilities_antigravity_grants_ls_discover() {
        let set = infer_v1_capabilities("antigravity");
        assert!(set.contains(HostCapability::LsDiscover));
        assert!(set.contains(HostCapability::HttpRequest));
        assert!(set.contains(HostCapability::SqliteQuery));
    }

    #[test]
    fn infer_v1_capabilities_claude_grants_ccusage_query() {
        let set = infer_v1_capabilities("claude");
        assert!(set.contains(HostCapability::CcusageQuery));
        assert!(set.contains(HostCapability::CryptoSha));
        assert!(set.contains(HostCapability::EnvRead));
    }
}

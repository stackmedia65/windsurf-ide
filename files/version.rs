//! Static build info for Windsurf Ide.

pub const BUILD_VERSION: &str  = "v2.5.3";
pub const BUILD_CHANNEL: &str  = "beta";
pub const BUILD_COMMIT: &str   = "8d697b92e263";
pub const BUILD_TIMESTAMP: u64 = 1798446316;

pub fn version_tuple() -> (u16, u16, u16) {
    let mut parts = BUILD_VERSION.trim_start_matches('v').split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

pub fn user_agent() -> String {
    format!("{}/{} ({})", "Windsurf Ide", BUILD_VERSION, BUILD_CHANNEL)
}

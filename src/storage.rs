use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

use crate::profile::Profile;

const APP_NAME: &str = "simple-profiles-manager";
const PROFILES_FILE: &str = "profiles.json";
const SELECTED_FILE: &str = "selected-profile";

static APP_ID: OnceLock<String> = OnceLock::new();

/// Sanitize the app_id to remove invalid path characters
/// This ensures the path stays within the simple-profiles-manager directory
fn sanitize_app_id(app_id: &str) -> String {
    app_id
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' | '\0' => '_',
            '.' if app_id.starts_with('.') => '_', // Prevent hidden directories
            _ => c,
        })
        .collect::<String>()
        .trim_matches(|c| c == ' ' || c == '.')
        .to_string()
}

/// Set the app_id for storage (must be called before any storage operations)
pub fn set_app_id(app_id: &str) {
    let sanitized = sanitize_app_id(app_id);
    APP_ID.set(sanitized).expect("app_id can only be set once");
}

fn get_app_id() -> &'static str {
    APP_ID.get().expect("app_id must be set before storage operations")
}

pub fn get_config_dir() -> Option<PathBuf> {
    dirs::config_dir().map(|p| p.join(APP_NAME).join(get_app_id()))
}

pub fn ensure_config_dir() -> Option<PathBuf> {
    let config_dir = get_config_dir()?;
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).ok()?;
    }
    Some(config_dir)
}

pub fn load_profiles() -> Vec<Profile> {
    let Some(config_dir) = get_config_dir() else {
        return Vec::new();
    };

    let profiles_path = config_dir.join(PROFILES_FILE);
    if !profiles_path.exists() {
        return Vec::new();
    }

    let Ok(content) = fs::read_to_string(&profiles_path) else {
        return Vec::new();
    };

    serde_json::from_str(&content).unwrap_or_default()
}

pub fn save_profiles(profiles: &[Profile]) -> bool {
    let Some(config_dir) = ensure_config_dir() else {
        return false;
    };

    let profiles_path = config_dir.join(PROFILES_FILE);
    let Ok(content) = serde_json::to_string_pretty(profiles) else {
        return false;
    };

    fs::write(profiles_path, content).is_ok()
}

pub fn load_selected_profile() -> Option<String> {
    let config_dir = get_config_dir()?;
    let selected_path = config_dir.join(SELECTED_FILE);

    if !selected_path.exists() {
        return None;
    }

    fs::read_to_string(selected_path).ok().map(|s| s.trim().to_string())
}

pub fn save_selected_profile(name: &str) -> bool {
    let Some(config_dir) = ensure_config_dir() else {
        return false;
    };

    let selected_path = config_dir.join(SELECTED_FILE);
    fs::write(selected_path, name).is_ok()
}

use std::str::FromStr;

use anyhow::{Context, Result};
use global_hotkey::{hotkey::HotKey, GlobalHotKeyManager};

use crate::config::AppVars;

#[allow(dead_code)]
pub struct Shortcuts {
    hotkeys_manager: GlobalHotKeyManager,
    pub mute_shortcut: HotKey,
}

impl Shortcuts {
    pub fn new(app_vars: &AppVars) -> Result<Self> {
        let hotkeys_manager = GlobalHotKeyManager::new().unwrap();
        let mute_shortcut = HotKey::from_str(&app_vars.shortcut)?;
        hotkeys_manager
            .register(mute_shortcut)
            .context("Failed to register hotkey")?;
        let shortcuts = Self {
            hotkeys_manager,
            mute_shortcut,
        };
        Ok(shortcuts)
    }
}

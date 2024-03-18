use anyhow::{Context, Result};
use global_hotkey::{
    hotkey::{Code, HotKey},
    GlobalHotKeyManager,
};

#[allow(dead_code)]
pub struct Shortcuts {
    hotkeys_manager: GlobalHotKeyManager,
    pub shift_meta_a: HotKey,
}

impl Shortcuts {
    pub fn new() -> Result<Self> {
        let hotkeys_manager = GlobalHotKeyManager::new().unwrap();
        let shift_meta_a = HotKey::new(None, Code::F5);
        hotkeys_manager
            .register(shift_meta_a)
            .context("Failed to register hotkey")?;
        let shortcuts = Self {
            hotkeys_manager,
            shift_meta_a,
        };
        Ok(shortcuts)
    }
}

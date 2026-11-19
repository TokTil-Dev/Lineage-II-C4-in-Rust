use std::collections::HashMap;

use killer_ui::color::Argb;

use super::{NameCtrlType, Rect, TextAlign};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Overlay {
    pub shown: HashMap<String, bool>,
    pub texts: HashMap<String, String>,
    pub text_colors: HashMap<String, Argb>,
    pub textures: HashMap<String, String>,
    pub alphas: HashMap<String, i32>,
    pub names: HashMap<String, (String, NameCtrlType, TextAlign, Option<Argb>)>,
    pub points: HashMap<String, (i32, i32)>,
    pub checks: HashMap<String, bool>,
    pub disabled: HashMap<String, bool>,
    pub tabs: HashMap<String, i32>,
    pub moves: HashMap<String, (i32, i32)>,
    pub sizes: HashMap<String, (i32, i32)>,
    pub anchors: HashMap<String, (String, String, String, i32, i32)>,
    pub cleared_anchors: Vec<String>,
    pub timers: Vec<(String, i32, i32)>,
    pub rects: HashMap<String, Rect>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EventRegistry {
    pub listeners: HashMap<i32, Vec<String>>,
}

impl EventRegistry {
    pub fn register(&mut self, window: &str, event_id: i32) {
        let listeners = self.listeners.entry(event_id).or_default();
        if !listeners.iter().any(|known| known == window) {
            listeners.push(window.to_string());
        }
    }

    pub fn listeners(&self, event_id: i32) -> &[String] {
        self.listeners.get(&event_id).map_or(&[], Vec::as_slice)
    }
}

pub fn leaf(key: &str) -> &str {
    key.rsplit('.').next().unwrap_or(key)
}

impl Overlay {
    pub fn lookup<'a, T>(map: &'a HashMap<String, T>, key: &str) -> Option<&'a T> {
        map.get(key).or_else(|| map.get(leaf(key)))
    }

    pub fn hidden(&self, key: &str) -> Option<bool> {
        Self::lookup(&self.shown, key).map(|shown| !shown)
    }

    pub fn text(&self, key: &str) -> Option<&str> {
        Self::lookup(&self.texts, key).map(String::as_str)
    }

    pub fn texture(&self, key: &str) -> Option<&str> {
        Self::lookup(&self.textures, key).map(String::as_str)
    }

    pub fn alpha(&self, key: &str) -> Option<i32> {
        Self::lookup(&self.alphas, key).copied()
    }

    pub fn point(&self, key: &str) -> Option<(i32, i32)> {
        Self::lookup(&self.points, key).copied()
    }

    pub fn checked(&self, key: &str) -> Option<bool> {
        Self::lookup(&self.checks, key).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlays_resolve_by_full_path_then_by_leaf_and_events_keep_registration_order() {
        let mut overlay = Overlay::default();
        overlay.texts.insert("StatusWnd.LevelTextBox".into(), "52".into());
        overlay.shown.insert("SystemMsgWnd".into(), false);
        assert_eq!(overlay.text("StatusWnd.LevelTextBox"), Some("52"));
        assert_eq!(overlay.text("LevelTextBox"), None);
        overlay.texts.insert("LevelTextBox".into(), "53".into());
        assert_eq!(overlay.text("Other.LevelTextBox"), Some("53"));
        assert_eq!(overlay.hidden("SystemMsgWnd"), Some(true));
        assert_eq!(overlay.hidden("ChatWnd"), None);
        let mut events = EventRegistry::default();
        events.register("StatusWnd", 70);
        events.register("DetailStatusWnd", 70);
        events.register("StatusWnd", 70);
        assert_eq!(events.listeners(70), ["StatusWnd", "DetailStatusWnd"]);
        assert!(events.listeners(71).is_empty());
    }
}

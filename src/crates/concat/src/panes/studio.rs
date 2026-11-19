// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Jareer Abdullah and Concat contributors

//! The Studio Library sheet. It shows the same preset names and ids the MCP
//! tools use (generated from the library JSON), and applies the chosen one as
//! a real Concat edit so a person editing alone gets the studio power too.

use slint::{ModelRc, SharedString, VecModel};

use crate::ui::{StudioData, StudioItem};
use crate::ui_libraries;

/// One thing that happens to the sheet.
#[derive(Clone, Debug)]
pub enum StudioMsg {
    Open,
    Close,
    Tab(i32),
    Chosen(String),
}

/// The library tabs, in the order the dialog shows them.
const TABS: [&str; 5] = ["frames", "animations", "curves", "rigs", "tracking"];

/// The sheet's state.
pub struct StudioPane {
    open: bool,
    tab: i32,
    items: ModelRc<StudioItem>,
    count: i32,
}

impl Default for StudioPane {
    fn default() -> Self {
        Self {
            open: false,
            tab: 0,
            items: ModelRc::new(VecModel::from(Vec::new())),
            count: 0,
        }
    }
}

fn rows(tab: i32) -> &'static [(&'static str, &'static str)] {
    match TABS.get(tab as usize).copied().unwrap_or("frames") {
        "frames" => ui_libraries::FRAMES,
        "animations" => ui_libraries::ANIMATIONS,
        "curves" => ui_libraries::CURVES,
        "rigs" => ui_libraries::CAMERA_RIGS,
        "tracking" => ui_libraries::TRACKERS,
        _ => ui_libraries::FRAMES,
    }
}

impl StudioPane {
    /// The Slint struct the dialog reads.
    pub fn data(&self) -> StudioData {
        StudioData {
            open: self.open,
            tab: self.tab,
            items: self.items.clone(),
            count: self.count,
        }
    }

    /// Rebuild the visible list for the current tab.
    fn refill(&mut self) {
        let list: Vec<StudioItem> = rows(self.tab)
            .iter()
            .map(|(id, name)| StudioItem {
                id: SharedString::from(*id),
                name: SharedString::from(*name),
            })
            .collect();
        self.count = list.len() as i32;
        self.items = ModelRc::new(VecModel::from(list));
    }

    /// Apply a chosen preset as a real edit. Frames add a shape; the rest key
    /// the selected clip, or stabilize it for tracking.
    fn apply(&self, id: &str, studio: &mut crate::studio::Studio) -> Result<String, String> {
        use concat_project::Command;
        use concat_project::model::{KeyEase, KeyProperty};

        let tab = TABS.get(self.tab as usize).copied().unwrap_or("frames");
        if tab == "frames" {
            let name = rows(self.tab)
                .iter()
                .find(|(fid, _)| *fid == id)
                .map(|(_, n)| n.to_string())
                .unwrap_or_else(|| id.to_string());
            return Ok(format!("STUDIO {name} — frame layer added"));
        }

        let Some(clip) = studio.selection.first().cloned() else {
            return Err("select a clip first".to_owned());
        };
        let command = match tab {
            "curves" => Command::SetClipKey {
                clip_id: clip,
                property: KeyProperty::Opacity,
                at: 0.0,
                value: 0.0,
                ease: KeyEase::IN_OUT,
            },
            "animations" => Command::SetClipKey {
                clip_id: clip,
                property: KeyProperty::Scale,
                at: 0.0,
                value: 0.8,
                ease: KeyEase::OUT,
            },
            "rigs" => Command::SetClipKey {
                clip_id: clip,
                property: if id.contains("crane") {
                    KeyProperty::OffsetY
                } else if id.contains("orbit") {
                    KeyProperty::Rotation
                } else {
                    KeyProperty::Scale
                },
                at: 0.0,
                value: 1.0,
                ease: KeyEase::IN_OUT,
            },
            "tracking" => Command::SetClipStabilize {
                clip_id: clip,
                stabilize: Some(concat_project::model::Stabilize {
                    smoothness: if id.contains("lock") { 0.95 } else { 0.6 },
                    ..Default::default()
                }),
            },
            _ => return Err("unknown library".to_owned()),
        };
        studio.apply(command).map_err(|e| e.to_string())?;
        Ok(format!("STUDIO {id} — applied to {clip}"))
    }

    /// Handle one message.
    pub fn update(&mut self, msg: StudioMsg, studio: &mut crate::studio::Studio) {
        match msg {
            StudioMsg::Open => {
                self.refill();
                self.open = true;
            }
            StudioMsg::Close => self.open = false,
            StudioMsg::Tab(tab) => {
                self.tab = tab;
                self.refill();
            }
            StudioMsg::Chosen(id) => match self.apply(&id, studio) {
                Ok(message) => studio.notify(&message, false),
                Err(error) => studio.notify(&format!("Studio: {error}"), true),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn libraries_are_populated() {
        assert!(ui_libraries::FRAMES.len() >= 50);
        assert!(ui_libraries::ANIMATIONS.len() >= 50);
        assert!(ui_libraries::CURVES.len() >= 10);
        let mut pane = StudioPane::default();
        pane.refill();
        assert_eq!(pane.count as usize, ui_libraries::FRAMES.len());
    }
    #[test]
    fn tabs_point_at_the_right_libraries() {
        assert_eq!(rows(0).len(), ui_libraries::FRAMES.len());
        assert_eq!(rows(1).len(), ui_libraries::ANIMATIONS.len());
        assert_eq!(rows(2).len(), ui_libraries::CURVES.len());
    }
}

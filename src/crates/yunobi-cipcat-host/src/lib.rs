// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 YunobiAirdrop

//! What the editor window needs that is not the edit itself.
//!
//! The edit lives in `yunobi-cipcat-project`, pixels in `yunobi-cipcat-render` and
//! `yunobi-cipcat-export`, files in `yunobi-cipcat-media`. This crate is the layer the
//! window talks to: it opens a project folder as a [`session::Session`],
//! keeps the recents list, caches waveforms and filmstrips beside the
//! project, composites the paused monitor's true frame, plays the audible
//! clips, finds the masks behind cutouts, packs templates, and holds the
//! one-at-a-time slots for long jobs.
//!
//! The window calls these functions in-process, and the doctrine is that
//! no editing decision is made here. If a function starts deciding what an
//! edit means, it belongs in `yunobi-cipcat-project`.
//!
//! Nothing here knows about a window. Long work reports through callbacks
//! and cancels through flags, and the caller decides which thread it runs on.

pub mod brush;
pub mod cards;
pub mod crash;
pub mod cutout;
pub mod dirs;
pub mod enhance;
pub mod export;
pub mod jobs;
pub mod logs;
pub mod machine;
pub mod media;
pub mod memory;
pub mod models;
pub mod playback;
pub mod preview;
pub mod projects;
pub mod proxy;
pub mod record;
pub mod reverse;
pub mod session;
pub mod templates;
pub mod titles;
pub mod updates;

pub use brush::{Brushes, RegionRequest};
pub use cutout::{AnalyseRequest, Cutouts};
pub use dirs::AppDirs;
pub use enhance::{EnhanceRequest, Enhancers};
pub use jobs::{Job, SingleFlight};
pub use projects::ProjectInfo;
pub use reverse::{ReverseRequest, Reversers};
pub use session::{EditorView, Session, SettingsView};
pub use titles::{TitleClip, Titles};

/// The one scheduler every decode for the screen goes through: the
/// monitor's frames and the frames ahead of the playhead first, then
/// filmstrips, the bin's artwork and proxies in turn, on a few threads
/// with one always kept clear of the background work. Owns the reader
/// pool the monitors read; see `yunobi_cipcat_media::Prefetcher`.
pub fn scheduler() -> &'static std::sync::Arc<yunobi_cipcat_media::Prefetcher> {
    static SCHEDULER: std::sync::OnceLock<std::sync::Arc<yunobi_cipcat_media::Prefetcher>> =
        std::sync::OnceLock::new();
    SCHEDULER.get_or_init(|| {
        std::sync::Arc::new(yunobi_cipcat_media::Prefetcher::with_defaults(
            // The frame cache is a share of the machine's memory, not one
            // size for a small laptop and a workstation alike.
            std::sync::Arc::new(yunobi_cipcat_media::ReaderPool::sized_for(memory::total())),
        ))
    })
}

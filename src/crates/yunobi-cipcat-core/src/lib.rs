// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 YunobiAirdrop

//! Core vocabulary of the Yunobi Cipcat engine.
//!
//! This crate holds the types that every other crate needs to agree on, and
//! nothing else. It has no dependencies and does no IO.
//!
//! - [`time`] - exact rational timestamps. Every time value in Yunobi Cipcat is a
//!   [`time::Rational`] number of seconds. Never `f64`.
//! - [`arena`] - generational arenas and `Copy` handles, the way Yunobi Cipcat models
//!   graphs.
//! - [`frame`] - a decoded RGBA8 image buffer.
//! - [`timeline`] - projects, tracks and clips.
//!
//! If you are about to add a dependency to this crate, the thing you are adding
//! probably belongs in `yunobi-cipcat-media` or `yunobi-cipcat-render` instead.

pub mod animate;
pub mod arena;
pub mod frame;
pub mod retime;
pub mod shader;
pub mod time;
pub mod timeline;

pub use frame::Frame;
pub use retime::SpeedCurve;
pub use shader::{Lut, RevealMap, ShaderPass, Stage, TransitionPass};
pub use time::{FrameRate, Rational, TimeRange};
pub use timeline::{Blend, Clip, ClipId, Project, Timeline, Track, TrackId, TrackKind};

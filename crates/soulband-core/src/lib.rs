//! Soulband game logic.
//!
//! Everything here is independent of the frontend: no windowing, input, or rendering.
//! The `soulband` binary reads input, calls into this crate, and draws the resulting state.

pub mod entity;
pub mod game_state;
pub mod level;
pub mod player;
pub mod pos;
pub mod sprite;

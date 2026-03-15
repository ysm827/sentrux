//! Sentrux binary library — allows sentrux-pro to reuse the entire CLI/GUI.
//!
//! Architecture: sentrux-pro depends on sentrux_bin and calls `sentrux_bin::run()`.
//! The only difference: sentrux-pro calls `license::set_tier(Pro)` before `run()`.

mod main_impl;
pub use main_impl::run;

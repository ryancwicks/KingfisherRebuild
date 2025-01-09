#![cfg_attr(not(feature = "std"), no_std)]

pub mod microcontroller_types;

#[cfg(feature = "std")]
pub mod dds_topics;

pub const DEFAULT_DOMAIN: i32 = 50;
pub const DEFAULT_ID: &str = "Kingfisher";
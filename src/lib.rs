#![doc(
    html_logo_url = "https://raw.githubusercontent.com/opensass/i18n-rs/refs/heads/main/assets/logo.webp",
    html_favicon_url = "https://github.com/opensass/i18n-rs/blob/main/assets/favicon.ico"
)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

#[cfg(feature = "yew")]
pub mod yew;

pub mod config;

pub use config::{I18n, I18nConfig, StorageType};

//! Oxide design system.
//!
//! This crate ships only `.slint` sources. Apps register [`SLINT_DIR`] as a Slint
//! library path in their `build.rs` and import components with
//! `import { OxideButton } from "@oxide/oxide.slint";`.
//!
//! ```ignore
//! // build.rs
//! let config = slint_build::CompilerConfiguration::new()
//!     .with_style("material".into())
//!     .with_library_paths(std::collections::HashMap::from([(
//!         "oxide".to_string(),
//!         std::path::PathBuf::from(oxide_ui::SLINT_DIR),
//!     )]));
//! slint_build::compile_with_config("ui/app.slint", config).unwrap();
//! ```

/// Absolute path of the directory holding the Oxide `.slint` sources.
pub const SLINT_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/ui");

/// Name under which [`SLINT_DIR`] should be registered (`@oxide/...`).
pub const LIBRARY_NAME: &str = "oxide";

/// Entry point file inside [`SLINT_DIR`].
pub const ENTRY: &str = "oxide.slint";

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn entry_file_exists() {
        assert!(Path::new(SLINT_DIR).join(ENTRY).is_file());
    }
}

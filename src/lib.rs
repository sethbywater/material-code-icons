//! This crate uses the icons from the
//! [VS Code Material Icon Theme](https://marketplace.visualstudio.com/items?itemName=Equinusocio.vsc-material-theme)
//! and makes them available in your rust code.
//!
//! The main point of interaction in this crate is [`code_icon()`](code_icon).
//! It takes a file name, folder name, or file extension and returns an SVG as a
//! `&[u8]`. If you know which icon you want ahead of time, you can also refer
//! to it specifically.
//!
//! ```rust
//! # use material_code_icons::code_icon;
//! let icon_from_extension = code_icon("rs");
//! let icon_from_file_name = code_icon(".gitignore");
//! let icon_from_file_name_extensions = code_icon("lib.rs");
//! let icon_from_folder_name = code_icon("src");
//! let icon = material_code_icons::RUST;
//! ```
//!
//! Currently, this crate does not provide any other return types, and you need
//! to deserialize the SVG with another crate, but if there is a popular SVG or
//! image type in Rust then it is possible to add support for it if you open an
//! issue.

use std::collections::HashMap;

mod icons;
pub use icons::*;

lazy_static::lazy_static! {
    static ref MAP: HashMap<String, usize> = {
        let mut map = HashMap::new();

        for (name, idx) in NAMES {
            map.insert(name.to_owned(), idx);
        }

        map
    };
}

/// Returns an SVG image (as `&'static [u8]`). `name` can be a file name, folder
/// name, or file extenstion.
/// ```rust
/// # use material_code_icons::code_icon;
/// let icon_from_file_name_extension = code_icon("lib.rs");
/// let icon_from_extension = code_icon("rs");
/// let icon_from_file_name = code_icon(".gitignore");
/// let icon_from_folder_name = code_icon("src");
///
/// ```
pub fn code_icon(name: &str) -> Option<&'static [u8]> {
    let name = &name.to_string();
    println!("{name}");
    MAP.get(name)
        .or(name.split('.').last().map_or(None, |name| MAP.get(name)))
        .map(|idx| ICONS[*idx])
}

#[cfg(test)]
mod test {
    use crate::code_icon;

    #[test]
    fn lang_name() {
        let rust_icon = include_bytes!("../vscode-material-icon-theme/icons/rust.svg");

        assert_eq!(code_icon("rust").expect("No icon for \"rust\""), rust_icon)
    }

    #[test]
    fn lang_extension() {
        let rust_icon = include_bytes!("../vscode-material-icon-theme/icons/rust.svg");

        assert_eq!(code_icon("rs").expect("No icon for \"rs\""), rust_icon)
    }

    #[test]
    fn file_name() {
        let git_icon = include_bytes!("../vscode-material-icon-theme/icons/git.svg");

        assert_eq!(
            code_icon(".gitignore").expect("No icon for \".gitignore\""),
            git_icon
        );
    }

    #[test]
    fn folder() {
        let test_icon = include_bytes!("../vscode-material-icon-theme/icons/folder-test.svg");

        assert_eq!(code_icon("test").expect("No icon for \"test\""), test_icon);
    }

    #[test]
    fn three_d() {
        let three_d_icon = include_bytes!("../vscode-material-icon-theme/icons/3d.svg");

        assert_eq!(code_icon("stl").expect("No icon for \"3d\""), three_d_icon);
    }
}

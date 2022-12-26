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

#![allow(unused)]

use material_code_icons::code_icon;

fn main() {
    let icon_from_extension = code_icon("rs");
    let icon_from_file_name = code_icon(".gitignore");
    let icon_from_file_name_extensions = code_icon("lib.rs");
    let icon_from_folder_name = code_icon("src");
    let icon = material_code_icons::RUST;
}

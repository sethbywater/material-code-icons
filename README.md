# material-code-icons

This crate uses the icons from the 
[VS Code Material Icon Theme](https://marketplace.visualstudio.com/items?itemName=Equinusocio.vsc-material-theme)
and makes them available in your rust code.

The main point of interaction in this crate is `code_icon()`. It takes a file
name, folder name, or file extension and returns an SVG as a `&[u8]`.

```rust
let icon_from_file_name_extension = code_icon("lib.rs");
let icon_from_extension = code_icon("rs");
let icon_from_file_name = code_icon(".gitignore");
let icon_from_folder_name = code_icon("src");
let icon = material_code_icons::RUST;
```

Currently, this crate does not provide any other return types, and you need to
deserialize the SVG with another crate, but if there is a popular SVG or image
type in Rust then it is possible to add support for it if you open an issue.
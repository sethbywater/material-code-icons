#![allow(unused)]

use glob::glob;
use std::{collections::HashMap, fmt::format};

fn main() {
    let mut icon_idxs = HashMap::new();
    let mut bare_icons = String::new();
    let mut icon_arr = String::new();
    let mut names = String::new();
    let mut icon_count: usize = 0;
    let mut names_count: usize = 0;

    for entry in glob("./vscode-material-icon-theme/icons/*.svg")
        .expect("Failed to read glob patter")
        .filter_map(|r| r.ok())
        .filter(|entry| entry.is_file())
    {
        let name = entry.file_stem().unwrap().to_str().unwrap();
        eprintln!("{name}");
        let id = name.replace('-', "_").replace('3', "THREE_").to_uppercase();

        icon_idxs.insert(name.to_owned(), icon_count);
        bare_icons.push_str(format!("pub const {id}: &'static [u8] = include_bytes!(\"../vscode-material-icon-theme/icons/{name}.svg\");\n").as_str());
        icon_arr.push_str(format!("\t{id},\n").as_str());

        icon_count += 1;
    }

    let file_icons_json = include_str!("src/file_icons.json");
    let folder_icons_json = include_str!("src/folder_icons.json");
    let language_icons_json = include_str!("src/language_icons.json");

    let file_icons_values: serde_json::Value =
        serde_json::from_str(file_icons_json).expect("Could not deserialze file_icons.json");
    let folder_icons_values: serde_json::Value =
        serde_json::from_str(folder_icons_json).expect("Could not deserialze folder_icons.json");
    let language_icons_values: serde_json::Value = serde_json::from_str(language_icons_json)
        .expect("Could not deserialze language_icons.json");

    for lang in language_icons_values
        .as_array()
        .unwrap()
        .iter()
        .map(|val| val.as_object().unwrap())
    {
        let name = lang
            .get("icon")
            .unwrap()
            .get("name")
            .unwrap()
            .as_str()
            .unwrap();

        let idx = icon_idxs
            .get(name)
            .expect(format!("No icon found for {name}").as_str());

        names.push_str(format!("\t(\"{name}\",{idx}),\n").as_str());
        names_count += 1;
    }

    // TODO: Use default icon
    for file_id in file_icons_values
        .get("icons")
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .map(|val| val.as_object().unwrap())
    {
        let name = file_id.get("name").unwrap().as_str().unwrap();
        let idx = icon_idxs
            .get(name)
            .expect(format!("No icon found for {name}").as_str());

        for id in ["fileNames", "fileExtensions"]
            .iter()
            .filter_map(|id| file_id.get(*id))
            .map(|val| val.as_array().unwrap().iter())
            .flatten()
            .map(|val| val.as_str().unwrap())
        {
            names.push_str(format!("\t(\"{id}\",{idx}),\n").as_str());
            names_count += 1;
        }
    }

    for folder_id in folder_icons_values.as_array().unwrap()[0]
        .get("icons")
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .map(|val| val.as_object().unwrap())
    {
        let name = folder_id.get("name").unwrap().as_str().unwrap();
        let idx = icon_idxs
            .get(name)
            .expect(format!("No icon found for {name}").as_str());

        for folder_name in folder_id
            .get("folderNames")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .map(|val| val.as_str().unwrap())
        {
            names.push_str(format!("\t(\"{folder_name}\",{idx}),\n").as_str());
            names_count += 1;
        }
    }

    let contents = format!(
        "
{bare_icons}
pub(crate) const ICONS: [&[u8]; {icon_count}] = [
{icon_arr}
];
		
pub(crate) const NAMES: [(&str, usize); {names_count}] = [
{names}
];
"
    );
    std::fs::write("src/icons.rs", contents).expect("Could not write icons.rs to disk");
}

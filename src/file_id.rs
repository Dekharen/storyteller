use std::path::Path;

use bevy::ecs::resource::Resource;

use crate::FileTypeSelection;

#[derive(Default, Debug, Resource, Clone)]
pub enum FileTypeSuggestion {
    Directory,
    Executable,
    Text,
    Readable,
    #[default]
    Unknown,
}

pub fn suggestion(path_buf: &Path) -> FileTypeSuggestion {
    if path_buf.is_file() {
        let ext = match path_buf.extension() {
            Some(ext) => ext.to_str().unwrap_or("Unknown extension"),
            None => return FileTypeSuggestion::Unknown,
        };
        match ext {
            "exe" | "bat" | "cmd" | "com" | "sh" | "ps1" | "vbs" | "wsf" | "js" | "jse" | "vbe"
            | "wsh" | "hta" | "cpl" | "scr" | "pif" | "lnk" => FileTypeSuggestion::Executable,
            "txt" => FileTypeSuggestion::Text,
            ".md" | "json" | "toml" | "yaml" | "json5" | "jsonc" | "yml" | "yamlc" | "ymlc" => {
                FileTypeSuggestion::Readable
            }
            _ => FileTypeSuggestion::Unknown,
        }
    } else {
        FileTypeSuggestion::Directory
    }
    // if mode & 0o111 != 0 {
    //     println!("binary...");
    // }
}

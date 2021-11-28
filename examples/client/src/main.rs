use libdmd::utils::config::builder::*;
use libdmd::utils::editor::editor::{Editor, EditorApp};
use serde::{Serialize, Deserialize};
use anyhow::Result;

#[derive(Serialize, Deserialize, Debug, Clone, Default, Eq, PartialEq)]
pub struct AppOptions {
    pub host: String,
    pub owner: String,
    pub editor: Editor,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Eq, PartialEq)]
pub struct Log {
    pub message: String,
}

fn main() -> Result<()> {
    let options = AppOptions {
        host: "GitHub".to_string(),
        owner: "edfloreshz".to_string(),
        editor: Editor::new(EditorApp::Vim)
    };
    let mut log = Log {
        message: "Logsd".to_string()
    };

    let config = ConfigBuilder::new()
        .project("devmode")
        .dir(
            DirectoryBuilder::new()
                .name("config")
                .file(
                    FileBuilder::new()
                        .name("config")
                        .format(FileFormat::TOML)
                        .data(&options),
                ),
        )
        .dir(
            DirectoryBuilder::new().name("logs")
                .file(
                    FileBuilder::new()
                        .name("logs")
                        .format(FileFormat::TOML)
                        .data(&log),
                )
        )
        .dir(DirectoryBuilder::new().name("paths"))
        .build()
        .unwrap();
    log.message = "Updated!".to_string();
    config.update()
}

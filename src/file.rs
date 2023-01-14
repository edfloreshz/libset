use std::{io::Write, path::PathBuf};

use anyhow::Result;
use serde::Serialize;

use crate::format::FileFormat;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct File {
    /// Represents a file or directory name.
    pub name: String,
    pub path: PathBuf,
    /// Represents a file's format.
    pub format: FileFormat,
    pub content: String,
}

impl File {
    /// Returns a new instance of `File`, the default format is TOML.
    /// If you wish to change that, use the `set_format` method.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            path: PathBuf::default(),
            format: FileFormat::TOML,
            content: String::new(),
        }
    }

    /// Set a file's format.
    /// ```
    /// use libset::file::File;
    /// use libset::format::FileFormat;
    /// use libset::new_file;
    ///
    /// let settings = new_file!("settings").set_format(FileFormat::JSON);
    /// ```
    pub fn set_format(mut self, format: FileFormat) -> Self {
        self.format = format;
        self
    }

    /// Set an element's path.
    /// ```
    /// use libset::file::File;
    /// use libset::new_file;
    /// use std::path::PathBuf;
    ///
    /// let settings: File = new_file!("settings")
    ///     .set_path(&PathBuf::from("/some/path"));
    /// ```
    pub fn set_path(mut self, path: &PathBuf) -> Self {
        self.path = path.join(&self.name);
        self
    }

    /// Write to a file.
    /// ```
    /// use libset::project::Project;
    /// use libset::format::FileFormat;
    /// use libset::new_file;
    /// use serde::{Serialize, Deserialize};
    /// use anyhow::{Result, Context};
    ///
    /// #[derive(Serialize, Deserialize)]
    /// struct Settings {
    ///     dark_mode_enabled: bool
    /// }
    ///
    /// fn main() -> Result<()> {
    ///     Project::new("com", "organization", "App").add_files(&[new_file!("settings").set_format(FileFormat::TOML)]);
    ///
    ///     let settings = Settings { dark_mode_enabled: true };
    ///     let project = Project::open("com", "organization", "App")?;
    ///     let mut files = project.find("settings")?;
    ///     if let Some(file) = files.get_mut(0) {
    ///         file.set_content(settings)?;
    ///         file.write()?;
    ///     }        
    ///     Ok(())
    /// }
    /// ```
    pub fn set_content<T: Serialize>(&mut self, content: T) -> Result<Self> {
        match self.format {
            FileFormat::Plain => (),
            FileFormat::TOML => self.content = toml::to_string(&content)?,
            FileFormat::JSON => self.content = serde_json::to_string(&content)?,
        };
        Ok(self.clone())
    }

    pub fn set_text(&mut self, content: &str) -> Result<Self> {
        match self.format {
            FileFormat::Plain => self.content = content.to_string(),
            FileFormat::TOML | FileFormat::JSON => (),
        };
        Ok(self.clone())
    }

    /// Write contents into the file.
    pub fn write(&self) -> Result<()> {
        std::fs::File::create(&self.path)?.write_all(self.content.as_bytes())?;
        Ok(())
    }
}

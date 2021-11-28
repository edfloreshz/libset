use std::{fs::{DirBuilder, File}, io::Write, path::{Path, PathBuf}};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::utils::constants::messages::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConfigBuilder {
    project: String,
    path: PathBuf,
    dirs: Vec<DirectoryBuilder>,
    files: Vec<FileBuilder>,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn builder(self) -> Self {
        self
    }
    pub fn project(mut self, project: &str) -> Self {
        self.project = project.to_string();
        let path = crate::data().join(&self.project);
        self.path = path.clone();
        self.dir(DirectoryBuilder::new().builder().path(path));
        self
    }
    pub fn dir(&mut self, mut dir: DirectoryBuilder) -> Self {
        dir.parent(self.path.clone());
        self.dirs.push(dir);
        self.clone()
    }
    pub fn file(&mut self, mut file: FileBuilder) -> Self {
        file.parent(self.path.clone());
        self.files.push(file);
        self.clone()
    }
    pub fn build(&self) -> Result<()> {
        for dir in &self.dirs {
            dir.build()?;
        }
        for file in &self.files {
            file.build()?;
        }
        self.write(self)
    }
    fn write<T: Serialize>(&self, data: T) -> Result<()> {
        File::create(crate::data().join(format!("{}{}", self.project.to_lowercase(), "/config/config.toml")))
            .with_context(|| failed_to("open", "config.toml"))?
            .write_all(
                toml::to_string(&data)?.as_bytes(),
            )
            .with_context(|| FAILED_TO_WRITE_CONFIG)?;
        println!("{}", SETTINGS_UPDATED);
        Ok(())
    }
    pub fn validate() -> Result<()> {
        Ok(())
    }
}


#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DirectoryBuilder {
    name: String,
    path: PathBuf,
    recursive: bool,
    dirs: Vec<DirectoryBuilder>,
    files: Vec<FileBuilder>,
}

impl DirectoryBuilder {
    pub fn new() -> Self {
        DirectoryBuilder::default()
    }
    pub fn builder(self) -> Self {
        self
    }
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
    pub fn path(mut self, path: PathBuf) -> Self {
        self.path = path;
        self
    }
    pub fn parent(&mut self, path: PathBuf) -> Self {
        let mut path = path;
        path.push(Path::new(self.name.as_str()));
        self.path = path.to_path_buf();
        self.clone()
    }
    pub fn recursive(mut self, recursive: bool) -> Self {
        self.recursive = recursive;
        self
    }
    pub fn dir(mut self, dir: DirectoryBuilder) -> Self {
        self.dirs.push(dir);
        self
    }
    pub fn file(&mut self, mut file: FileBuilder) -> Self {
        file.parent(self.path.clone());
        self.files.push(file);
        self.clone()
    }
    pub fn build(&self) -> Result<()> {
        DirBuilder::new().recursive(self.recursive).create(&self.path)?;
        for dir in &self.dirs {
            dir.build()?;
        }
        for file in &self.files {
            file.build()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileBuilder {
    name: String,
    path: PathBuf,
    format: FileFormat
}

impl FileBuilder {
    pub fn new() -> Self {
        FileBuilder::default()
    }
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
    pub fn format(mut self, format: FileFormat) -> Self {
        self.format = format;
        self
    }
    pub fn parent(&mut self, path: PathBuf) -> Self {
        let mut path = path;
        path.push(Path::new(&self.format.ext(self.name.as_str())));
        self.path = path.to_path_buf();
        self.clone()
    }
    pub fn path(mut self, path: &mut PathBuf) -> Self {
        let path = path;
        path.push(Path::new(&self.format.ext(self.name.as_str())));
        self.path = path.to_path_buf();
        self
    }
    pub fn builder(self) -> Self {
        self
    }
    pub fn write<T: Serialize>(&self, data: T) -> Result<()> {
        File::create(&self.path)
            .with_context(|| failed_to("open", "config.toml"))?
            .write_all(
                toml::to_string(&data)
                    .with_context(|| FAILED_TO_PARSE)?
                    .as_bytes(),
            )
            .with_context(|| FAILED_TO_WRITE_CONFIG)?;
        println!("{}", SETTINGS_UPDATED);
        Ok(())
    }
    pub fn build(&self) -> Result<()> {
        File::create(&self.path).with_context(|| "Failed to create file.")?;
        Ok(())
    }
}


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FileFormat {
    TOML,
    JSON
}

impl FileFormat {
    fn ext(&self, file: &str) -> String {
        match self {
            FileFormat::TOML => format!("{}{}", file, ".toml"),
            FileFormat::JSON => format!("{}{}", file, ".json"),
        }
    }
}

impl Default for FileFormat {
    fn default() -> Self {
        Self::TOML
    }
}
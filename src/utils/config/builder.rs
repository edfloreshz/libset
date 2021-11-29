use std::{
    fs::{DirBuilder, File},
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use core::default::Default;
use std::fs::read_to_string;
use crate::data;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConfigBuilder  {
    project: String,
    root: DirectoryBuilder,
    dirs: Vec<DirectoryBuilder>,
    files: Vec<FileBuilder>
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn current<'a, T: Serialize + Deserialize<'a> + Default>(&self, path: &str) -> Option<String> {
        let path = data().join("app.json");
        if path.exists() {
            return read_to_string(&path).ok();
        }
        None
    }
    fn parse<'a, T: Serialize + Deserialize<'a> + Default>(content: &'a str, file: FileBuilder) -> Option<T> {
        match file.format {
            FileFormat::TOML => toml::from_str(content).ok(),
            FileFormat::JSON => serde_json::from_str(content).ok()
        }
    }
    pub fn project(mut self, project: &str) -> Self {
        self.project = project.to_string();
        let root = DirectoryBuilder::new().path(crate::data().join(project));
        self.root = root.clone();
        self.dir(root.clone());
        self
    }
    pub fn dir(&mut self, mut dir: DirectoryBuilder) -> Self {
        dir.parent(self.root.path.clone().into());
        for file in &mut dir.files {
            file.parent(&mut dir.path);
        }
        self.dirs.push(dir);
        self.clone()
    }
    pub fn file(&mut self, mut file: FileBuilder) -> Self {
        file.parent(&mut self.root.path.clone().into());
        self.files.push(file);
        self.clone()
    }
    pub fn build(&mut self) -> Result<Self> {
        let config = FileBuilder::new()
            .name("app")
            .format(FileFormat::JSON)
            .data(self)?;
        self.file(config);
        for dir in &self.dirs {
            dir.build()?;
        }
        for file in &self.files {
            file.build()?;
        }
        Ok(self.clone())
    }
    pub fn update(mut self) -> Result<()> {
        self.build()?;
        println!("Update successful");
        Ok(())
    }
    pub fn validate() -> Result<()> {
        //TODO: Validate structure.
        Ok(())
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DirectoryBuilder {
    name: String,
    path: PathBuf,
    recursive: bool,
    dirs: Vec<DirectoryBuilder>,
    files: Vec<FileBuilder>
}

impl DirectoryBuilder {
    pub fn new() -> Self {
        DirectoryBuilder::default()
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
    /// Add a file to DirectoryBuilder.
    pub fn file(&mut self, mut file: FileBuilder) -> Self {
        file.parent(&mut self.path.clone());
        self.files.push(file);
        self.clone()
    }
    pub fn build(&self) -> Result<()> {
        if !self.path.exists() {
            DirBuilder::new()
                .recursive(self.recursive)
                .create(&self.path)?;
        }
        for dir in &self.dirs {
            if !dir.path.exists() {
                dir.build()?;
            }
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
    data: Option<String>,
    format: FileFormat,
}

impl FileBuilder {
    pub fn new() -> Self {
        FileBuilder::default()
    }
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
    pub fn get_name(&self) -> String {
        self.format.ext(&*self.name)
    }
    pub fn current(&self) -> Option<String> {
        std::fs::read_to_string(self.path.clone()).ok()
    }
    pub fn format(mut self, format: FileFormat) -> Self {
        self.format = format;
        self
    }
    /// Set parent to FileBuilder.
    pub fn parent(&mut self, path: &mut PathBuf) -> Self {
        let path = path.join(self.get_name().as_str()); //TODO: No se modifica
        self.path = path.clone();
        self.clone()
    }
    pub fn path(mut self, path: &mut PathBuf) -> Self {
        let path = path;
        path.push(Path::new(&self.format.ext(self.name.as_str())));
        self.path = path.to_path_buf();
        self
    }
    pub fn data<T: Serialize + Default + Clone>(mut self, data: &T) -> Result<Self> {
        let data = match self.format {
            FileFormat::TOML => toml::to_string(&data)?,
            FileFormat::JSON => serde_json::to_string(&data)?
        };
        self.data = Some(data);
        Ok(self)
    }
    pub fn write(&self) -> Result<()> {
        if self.data.is_none() {
            return Ok(());
        }
        File::create(&self.path)
            .with_context(|| "Failed to create file.")?
            .write_all(self.data.as_ref().unwrap().as_bytes())
            .with_context(|| "Failed to write to file.")?;
        println!("Wrote to {}", self.path.display());
        Ok(())
    }
    pub fn build(&self) -> Result<()> {
        File::create(&self.path).with_context(|| "Failed to create file.")?;
        if self.data.is_some() {
            self.write()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FileFormat {
    TOML,
    JSON,
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

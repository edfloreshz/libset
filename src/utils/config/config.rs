use anyhow::{Result};
use serde::{Deserialize, Serialize};
use core::default::Default;
use std::fs::read_to_string;
use std::io::{BufReader, Read};
use serde::de::DeserializeOwned;
use crate::data;
use crate::utils::config::{
    file::*,
    format::*,
    directory::*
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    project: String,
    root: Directory,
    dirs: Vec<Directory>,
    files: Vec<File>
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn current(&self) -> Option<Self> {
        let path = data().join(format!("{}/app.json", self.project));
        let contents = if path.exists() {
            read_to_string(&path).ok()?
        } else {
            String::new()
        };
        serde_json::from_str(&contents).ok()
    }
    pub fn get<'a, T: Serialize + DeserializeOwned>(path: &str, format: FileFormat) -> Option<T> {
        let path = data().join(path);
        if path.exists() {
            let file = std::fs::File::open(path).ok()?;
            let mut reader = BufReader::new(file);
            let mut buffer = Vec::new();
            reader.read_to_end(&mut buffer).ok()?;
            match format {
                FileFormat::TOML => toml::from_slice(buffer.as_slice()).ok(),
                FileFormat::JSON => serde_json::from_reader(reader).ok()
            }
        } else {
            None
        }
    }
    pub fn project(mut self, project: &str) -> Self {
        self.project = project.to_string();
        let root = Directory::new().path(crate::data().join(project));
        self.root = root.clone();
        self.dir(root.clone());
        self
    }
    pub fn dir(&mut self, mut dir: Directory) -> Self {
        dir.parent(self.root.path.clone().into());
        for file in &mut dir.files {
            file.parent(&mut dir.path);
        }
        self.dirs.push(dir);
        self.clone()
    }
    pub fn file(&mut self, mut file: File) -> Self {
        file.parent(&mut self.root.path.clone().into());
        self.files.push(file);
        self.clone()
    }
    pub fn build(&mut self) -> Result<Self> {
        let config = File::new()
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






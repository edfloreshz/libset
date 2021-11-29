use anyhow::{Result};
use serde::{Deserialize, Serialize};
use core::default::Default;
use std::fs::read_to_string;
use crate::data;
use crate::utils::config::{
    file::*,
    format::*,
    directory::*
};

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
    pub fn current<'a, T: Serialize + Deserialize<'a> + Default>(&self) -> Option<String> {
        let path = data().join("app.json");
        if path.exists() {
            return read_to_string(&path).ok();
        }
        None
    }
    fn _parse<'a, T: Serialize + Deserialize<'a> + Default>(content: &'a str, file: FileBuilder) -> Option<T> {
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






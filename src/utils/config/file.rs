use std::{
    fs::{File as SysFile},
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use core::default::Default;
use crate::utils::config::{
    format::*,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct File {
    name: String,
    path: PathBuf,
    data: Option<String>,
    pub format: FileFormat,
}

impl File {
    pub fn new() -> Self {
        File::default()
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
        SysFile::create(&self.path)
            .with_context(|| "Failed to create file.")?
            .write_all(self.data.as_ref().unwrap().as_bytes())
            .with_context(|| "Failed to write to file.")?;
        println!("File located at {} written.", self.path.display());
        Ok(())
    }
    pub fn build(&self) -> Result<()> {
        SysFile::create(&self.path).with_context(|| "Failed to create file.")?;
        if self.data.is_some() {
            self.write()?;
        }
        Ok(())
    }
}
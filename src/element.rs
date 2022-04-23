use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs::{DirBuilder, File};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ElementFormat {
    Directory,
    File,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Element {
    pub(crate) name: String,
    pub(crate) path: PathBuf,
    format: ElementFormat,
    pub(crate) children: Vec<Element>,
}

impl Element {
    pub fn new(name: &str) -> Self {
        Element {
            name: name.to_string(),
            path: Default::default(),
            format: ElementFormat::Directory,
            children: vec![],
        }
    }
    pub fn format(mut self, format: ElementFormat) -> Self {
        self.format = format;
        self
    }
    pub(crate) fn set_path(&mut self, path: PathBuf) -> Self {
        self.path = path.join(&self.name);
        self.clone()
    }
    pub fn child(mut self, element: Element) -> Self {
        self.children.push(element);
        self
    }
    pub fn write(&self) -> Result<Self> {
        match &self.format {
            ElementFormat::Directory => {
                if !&self.path.exists() {
                    DirBuilder::new().recursive(false).create(&self.path)?;
                }
            }
            ElementFormat::File => {
                File::create(&self.path).with_context(|| "Failed to create file.")?;
            }
        }
        Ok(self.clone())
    }
}

pub mod directory;
pub mod file;
pub mod format;

use crate::data;
use crate::utils::config::format::FileFormat;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use std::fs::{DirBuilder, File};
use std::io::{BufReader, Read, Write};
use std::path::PathBuf;
use crate::utils::config::format::FileFormat::{TOML};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    name: String,
    author: String,
    version: String,
    about: String,
    elements: Vec<Element>,
}

impl Config {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self.add(Element::new("").child(Element::new("app.toml").format(Format::File)));
        self
    }
    pub fn author(mut self, author: &str) -> Self {
        self.author = author.to_string();
        self
    }
    pub fn version(mut self, version: &str) -> Self {
        self.version = version.to_string();
        self
    }
    pub fn about(mut self, about: &str) -> Self {
        self.about = about.to_string();
        self
    }
    pub fn path(&self) -> PathBuf {
        data().join(self.name.to_string())
    }
    pub fn add(&mut self, mut element: Element) -> Self {
        if element.name.is_empty(){
            element.path(data().join(self.name.to_string()));
            element.name = self.name.to_string()
        } else {
            element.path(self.path());
        }
        for child in &mut element.children {
            child.path(element.path.clone());
            Config::fill_paths(child);
        }
        self.elements.push(element);
        self.clone()
    }
    fn fill_paths(element: &mut Element) {
        for child in &mut element.children {
            child.path(element.path.clone());
            if !child.children.is_empty() {
                Config::fill_paths(child)
            } else {
                continue;
            }
        }
    }
    pub fn write(self) -> Result<Self> {
        for child in &self.elements {
            Config::write_recursive(child)?;
        }
        Config::set(format!("{}/app.toml", self.name).as_str(), self.clone(), TOML)?;
        Ok(self)
    }
    fn write_recursive(element: &Element) -> Result<()> {
        element.write()?;
        for child in &element.children {
            child.write()?;
            if !child.children.is_empty() {
                Config::write_recursive(child)?;
            } else {
                continue;
            }
        }
        Ok(())
    }

    pub fn current() -> Option<Self> {
        Config::get::<Config>("devmode/app.toml", FileFormat::TOML)
    }
    pub fn get<T: Serialize + DeserializeOwned>(path: &str, format: FileFormat) -> Option<T> {
        let path = data().join(path);
        if path.exists() {
            let file = std::fs::File::open(path).ok()?;
            let mut reader = BufReader::new(file);
            let mut buffer = Vec::new();
            reader.read_to_end(&mut buffer).ok()?;
            match format {
                FileFormat::TOML => {
                    let res = toml::from_slice(buffer.as_slice());
                    if let Err(ref e) = res {
                        println!("TOML: {}", e);
                    }
                    res.ok()
                },
                FileFormat::JSON => {
                    let res = serde_json::from_reader(reader);
                    if let Err(ref e) = res {
                        println!("JSON: {}", e);
                    }
                    res.ok()
                },
            }
        } else {
            None
        }
    }
    pub fn set<T: Serialize + DeserializeOwned>(
        path: &str,
        content: T,
        format: FileFormat,
    ) -> Result<()> {
        let path = data().join(path);
        let mut file = std::fs::File::create(path)?;
        let content = match format {
            FileFormat::TOML => toml::to_string(&content)?,
            FileFormat::JSON => serde_json::to_string(&content)?,
        };
        file.write_all(content.as_bytes())?;
        println!("Settings updated.");
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Element {
    name: String,
    path: PathBuf,
    format: Format,
    children: Vec<Element>,
}

impl Element {
    pub fn new(name: &str) -> Self {
        Element {
            name: name.to_string(),
            path: Default::default(),
            format: Format::Directory,
            children: vec![],
        }
    }
    pub fn format(mut self, format: Format) -> Self {
        self.format = format;
        self
    }
    fn path(&mut self, path: PathBuf) -> PathBuf {
        self.path = path.join(self.name.to_string());
        path
    }
    pub fn child(mut self, element: Element) -> Self {
        self.children.push(element);
        self
    }
    pub fn write(&self) -> Result<Self> {
        match &self.format {
            Format::Directory => {
                if !&self.path.exists() {
                    DirBuilder::new().recursive(false).create(&self.path)?;
                    println!("Directory `{}` was written.", self.name)
                }
            }
            Format::File => {
                File::create(&self.path).with_context(|| "Failed to create file.")?;
                println!("File `{}` was written.", self.name)
            }
        }
        Ok(self.clone())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Format {
    Directory,
    File,
}

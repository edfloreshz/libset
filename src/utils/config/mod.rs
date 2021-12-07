pub mod format;
pub mod macros;

use crate::{data, dir, fi};
use crate::utils::config::format::FileFormat;
use crate::utils::config::format::FileFormat::TOML;
use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fs::{DirBuilder, File};
use std::io::{BufReader, Read, Write};
use std::path::PathBuf;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    name: String,
    author: String,
    version: String,
    about: String,
    elements: Vec<Element>,
}

impl Config {
    /// Initializes a new configuration for the app.
    ///
    /// It sets the name and adds the base directory and `app.toml`.
    /// `app.toml` is the file where the file structure is saved.
    ///
    /// Example:
    /// ```rust
    /// use libdmd::utils::config::Config;
    /// use anyhow::Result;
    ///
    /// fn main() -> Result<()> {
    ///     let config = Config::new("app").write()?;
    /// }
    /// ```
    /// Should generate the following structure.
    /// ```text
    /// Config {
    ///     name: "app",
    ///     author: "",
    ///     version: "",
    ///     about: "",
    ///     elements: [
    ///         Element {
    ///             name: "",
    ///             path: "/home/eduardo/.local/share/app/",
    ///             format: Directory,
    ///             children: [],
    ///         },
    ///         Element {
    ///             name: "app.toml",
    ///             path: "/home/eduardo/.local/share/app/app.toml",
    ///             format: File,
    ///             children: [],
    ///         },
    ///     ],
    /// }
    /// ```
    pub fn new(name: &str) -> Self {
        let mut base = Self {
            name: name.to_string(),
            author: "".to_string(),
            version: "".to_string(),
            about: "".to_string(),
            elements: vec![]
        };
        base.add(dir!("").child(fi!("app.toml")))
    }
    /// Sets the author of the program.
    pub fn author(mut self, author: &str) -> Self {
        self.author = author.to_string();
        self
    }
    /// Sets the version of the program.
    pub fn version(mut self, version: &str) -> Self {
        self.version = version.to_string();
        self
    }
    /// Sets the information about the program.
    pub fn about(mut self, about: &str) -> Self {
        self.about = about.to_string();
        self
    }
    /// Returns the base path with the name property joined.
    pub fn path(&self) -> PathBuf {
        data().join(self.name.to_string())
    }
    /// Adds an element to the child
    pub fn add(&mut self, mut element: Element) -> Self {
        // Set path for element
        element.set_path(self.path());
        // Set path for child elements.
        for child in &mut element.children {
            child.set_path(element.path.clone());
            Config::fill_paths(child);
        }
        // Base directory
        if self.elements.get(0).is_some() {
            self.elements.get_mut(0).unwrap().children.push(element.clone());
        }
        if self.elements.is_empty() {
            element.name = self.clone().name;
            self.elements.push(element)
        }
        self.clone()
    }
    fn fill_paths(element: &mut Element) {
        for child in &mut element.children {
            child.set_path(element.path.clone());
            if !child.children.is_empty() {
                Config::fill_paths(child)
            }
        }
    }
    pub fn write(self) -> Result<Self> {
        for child in &self.elements {
            Config::write_recursive(child)?;
        }
        Config::set(
            format!("{}/app.toml", self.name).as_str(),
            self.clone(),
            TOML,
        )?;
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
        let full_path = data();
        let full_path = full_path.join(path);
        if full_path.exists() {
            let file = std::fs::File::open(full_path).ok()?;
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
                }
                FileFormat::JSON => {
                    let res = serde_json::from_reader(reader);
                    if let Err(ref e) = res {
                        println!("JSON: {}", e);
                    }
                    res.ok()
                }
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
        let full_path = data();
        let full_path = full_path.join(path);
        let mut file = std::fs::File::create(full_path)?;
        let content = match format {
            FileFormat::TOML => toml::to_string(&content)?,
            FileFormat::JSON => serde_json::to_string(&content)?,
        };
        file.write_all(content.as_bytes())?;
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
    fn set_path(&mut self, path: PathBuf) -> Self {
        self.path = path.join(self.name.to_string());
        self.clone()
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


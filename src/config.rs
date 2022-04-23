use crate::element::Element;
use crate::format::FileType;
use crate::routes::data;
use crate::{directory, fi};
use anyhow::Result;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
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

/// Config is a data structure that encapsulates the main info about your app.
impl Config {
    /// Initializes a new configuration for the app.
    ///
    /// Sets the name for the app, includes the base directory and `app.toml` to Config's children.
    ///
    /// The `app.toml` file contains the Config structure represented as TOML.
    ///
    /// Example:
    /// ```rust
    /// use libdmd::config::Config;
    /// use anyhow::Result;
    ///
    /// fn main() -> Result<()> {
    ///     let config = Config::new("app");
    ///     Ok(())
    /// }
    /// ```
    pub fn new(name: &str) -> Self {
        let mut base = Self {
            name: name.to_string(),
            author: "".to_string(),
            version: "".to_string(),
            about: "".to_string(),
            elements: vec![],
        };
        base.add(directory!("").child(fi!("app.toml")))
    }
    /// Sets the author of the program.
    ///
    /// ```rust
    /// use libdmd::config::Config;
    ///
    /// let config = Config::new("app").author("Your Name");
    /// ```
    pub fn author(mut self, author: &str) -> Self {
        self.author = author.to_string();
        self
    }
    /// Sets the version of the program.
    ///
    /// ```rust
    /// use libdmd::config::Config;
    ///
    /// let config = Config::new("app").version("0.1.1");
    /// ```
    pub fn version(mut self, version: &str) -> Self {
        self.version = version.to_string();
        self
    }
    /// Sets the information about the program.
    ///
    /// ```rust
    /// use libdmd::config::Config;
    ///
    /// let config = Config::new("app").about("This app is just for demonstration.");
    /// ```
    pub fn about(mut self, about: &str) -> Self {
        self.about = about.to_string();
        self
    }
    /// Returns the base path with the name property joined.
    fn path(&self) -> PathBuf {
        data().join(&self.name)
    }
    /// Adds an element to the child
    ///
    /// ```rust
    /// use libdmd::config::Config;
    /// use libdmd::directory;
    ///
    /// let config = Config::new("app").add(directory!("config"));
    /// ```
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
            self.elements
                .get_mut(0)
                .unwrap()
                .children
                .push(element.clone());
        }
        if self.elements.is_empty() {
            element.name = self.clone().name;
            self.elements.push(element)
        }
        self.clone()
    }
    /// Fills the path for all the children inside an Element.
    fn fill_paths(element: &mut Element) {
        for child in &mut element.children {
            child.set_path(element.path.clone());
            if !child.children.is_empty() {
                Config::fill_paths(child)
            }
        }
    }
    /// Writes the current layout to the filesystem.
    ///
    /// ```rust
    /// use libdmd::config::Config;
    /// use libdmd::directory;
    ///
    /// fn main() -> anyhow::Result<()> {
    ///     let config = Config::new("app").add(directory!("config")).write()?;
    ///     Ok(())
    /// }
    /// ```
    pub fn write(self) -> Result<Self> {
        for child in &self.elements {
            Config::write_recursive(child)?;
        }
        Config::set(
            format!("{}/app.toml", self.name).as_str(),
            self.clone(),
            FileType::TOML,
        )?;
        Ok(self)
    }
    /// Recursively writes every children inside the structure to the filesystem.
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

    pub fn is_written(&self) -> bool {
        Config::get::<Config>(
            format!("{}/app.toml", &self.name).as_str(),
            FileType::TOML,
        )
        .is_some()
    }

    pub fn current() -> Option<Self> {
        Config::get::<Config>("devmode/app.toml", FileType::TOML)
    }
    pub fn get<T: Serialize + DeserializeOwned>(path: &str, format: FileType) -> Option<T> {
        let full_path = data();
        let full_path = full_path.join(path);
        if full_path.exists() {
            let file = std::fs::File::open(full_path).ok()?;
            let mut reader = BufReader::new(file);
            let mut buffer = Vec::new();
            reader.read_to_end(&mut buffer).ok()?;
            match format {
                FileType::TOML => {
                    let res = toml::from_slice(buffer.as_slice());
                    res.ok()
                }
                FileType::JSON => {
                    let res = serde_json::from_reader(reader);
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
        format: FileType,
    ) -> Result<()> {
        let full_path = data();
        let full_path = full_path.join(path);
        let mut file = std::fs::File::create(full_path)?;
        let content = match format {
            FileType::TOML => toml::to_string(&content)?,
            FileType::JSON => serde_json::to_string(&content)?,
        };
        file.write_all(content.as_bytes())?;
        Ok(())
    }
}

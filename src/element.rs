use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs::{DirBuilder, File};
use std::path::PathBuf;

/// Enum used to indicate an Element's type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ElementType {
    /// Represents a directory.
    Directory,
    /// Represents a file.
    File,
}

/// Structure that represents a file or directory.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Element {
    /// Represents a file or directory name.
    pub name: String,
    /// Represents a file or directory path.
    pub path: PathBuf,
    /// Represents an element's type.
    pub format: ElementType,
    /// Children vector for directory types.
    pub children: Vec<Element>,
}

impl Element {
    /// Create a new element.
    /// ```
    /// use libset::element::Element;
    ///
    /// let dir: Element = Element::new("dir");
    /// ```
    pub fn new(name: &str) -> Self {
        Element {
            name: name.to_string(),
            path: Default::default(),
            format: ElementType::Directory,
            children: vec![],
        }
    }
    /// Set an element's type.
    /// ```
    /// use libset::element::Element;
    /// use libset::element::ElementType;
    ///
    /// let settings: Element = Element::new("settings.toml").set_type(ElementType::File);
    /// ```
    pub fn set_type(mut self, format: ElementType) -> Self {
        self.format = format;
        self
    }
    /// Set an element's path.
    /// ```
    /// use libset::element::Element;
    /// use libset::element::ElementType;
    /// use std::path::PathBuf;
    ///
    /// let settings: Element = Element::new("settings.toml")
    ///     .set_type(ElementType::File)
    ///     .set_path(PathBuf::from("/some/path"));
    /// ```
    pub fn set_path(&mut self, path: PathBuf) -> Self {
        self.path = path.join(&self.name);
        self.clone()
    }
    /// Set a child for an element.
    /// ```
    /// use libset::element::Element;
    /// use libset::element::ElementType;
    /// use std::path::PathBuf;
    /// use libset::fi;
    ///
    /// let settings: Element = Element::new("settings")
    ///     .set_type(ElementType::Directory)
    ///     .set_path(PathBuf::from("/some/path"));
    /// settings.add_child(fi!("settings.toml"));
    /// ```
    pub fn add_child(mut self, element: Element) -> Self {
        self.children.push(element);
        self
    }
    pub(crate) fn write(&self) -> Result<Self> {
        match &self.format {
            ElementType::Directory => {
                if !&self.path.exists() {
                    DirBuilder::new().recursive(false).create(&self.path)?;
                }
            }
            ElementType::File => {
                File::create(&self.path).with_context(|| "Failed to create file.")?;
            }
        }
        Ok(self.clone())
    }
}

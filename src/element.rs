use crate::config::Config;
use crate::format::FileFormat;
use anyhow::{Context, Result};
use dyn_clone::DynClone;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
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

/// Trait that should be implemented by user's data types.
pub trait Content: DynClone + erased_serde::Serialize + Debug {}
dyn_clone::clone_trait_object!(Content);
erased_serde::serialize_trait_object!(Content);

/// Structure that represents a file or directory.
#[derive(Debug, Serialize, Clone)]
pub struct Element {
    /// Represents a file or directory name.
    pub name: String,
    /// Represents a file or directory path.
    pub path: PathBuf,
    /// Represents an element's type.
    pub element_type: ElementType,
    /// Represents a file's format.
    // #[serde(skip_serializing_if = Option::is_none)]
    pub element_format: Option<FileFormat>,
    /// Content for the element.
    // #[serde(skip_serializing_if = Option::is_none)]
    pub content: Option<Box<dyn Content>>,
    /// Children vector for directory types.
    // #[serde(skip_serializing_if = Option::is_none)]
    pub children: Option<Vec<Element>>,
}

impl Element {
    /// Create a new element.
    /// ```
    /// use libset::element::{Element, ElementType};
    ///
    /// let element = Element::new("file");
    /// ```
    pub fn new(name: &str) -> Self {
        Element {
            name: name.to_string(),
            path: Default::default(),
            element_type: ElementType::File,
            element_format: None,
            content: None,
            children: None,
        }
    }
    /// Set an element's type.
    /// ```
    /// use libset::element::Element;
    /// use libset::element::ElementType;
    ///
    /// let settings: Element = Element::new("settings.toml").set_type(ElementType::File);
    /// ```
    pub fn set_type(&mut self, element_type: ElementType) -> Self {
        self.element_type = element_type;
        self.clone()
    }
    /// Set an element's format.
    /// ```
    /// use libset::element::Element;
    /// use libset::format::FileFormat;
    ///
    /// let settings = Element::new("settings.toml").set_format(FileFormat::TOML);
    /// ```
    pub fn set_format(&mut self, format: FileFormat) -> Self {
        self.element_format = Some(format);
        self.clone()
    }
    /// Set a file's format.
    pub fn set_content(&mut self, content: Box<dyn Content>) -> Self {
        self.content = Some(content);
        self.clone()
    }
    /// Set an element's path.
    /// ```
    /// use libset::element::Element;
    /// use libset::element::ElementType;
    /// use std::path::PathBuf;
    ///
    /// let settings: Element = Element::new("settings.toml")
    ///     .set_type(ElementType::File)
    ///     .set_path(&PathBuf::from("/some/path"));
    /// ```
    pub fn set_path(&mut self, path: &PathBuf) -> Self {
        self.path = path.join(&self.name);
        self.clone()
    }
    /// Set a child for an element.
    /// ```
    /// use libset::element::Element;
    /// use libset::element::ElementType;
    /// use std::path::PathBuf;
    /// use libset::new_file;
    ///
    /// let mut  settings: Element = Element::new("settings")
    ///     .set_type(ElementType::Directory)
    ///     .set_path(&PathBuf::from("/some/path"));
    /// settings.add_child(new_file!("settings.toml"));
    /// ```
    pub fn add_child(&mut self, element: Element) -> Self {
        if let Some(children) = &mut self.children {
            children.push(element)
        }
        self.clone()
    }
    pub fn write(&self) -> Result<Self> {
        match &self.element_type {
            ElementType::Directory => {
                if !&self.path.exists() {
                    DirBuilder::new().recursive(true).create(&self.path)?;
                }
            }
            ElementType::File => {
                File::create(&self.path).with_context(|| "Failed to create file.")?;
                if self.content.is_some() && self.element_format.is_some() {
                    Config::set(
                        self.path.to_str().unwrap(),
                        self.content.clone().unwrap(),
                        self.element_format.unwrap(),
                    )?;
                }
            }
        }
        Ok(self.clone())
    }
}

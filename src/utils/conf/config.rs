use std::fs::{DirBuilder, File};
use std::path::PathBuf;
use anyhow::{Context, Result};
use crate::data;

#[derive(Debug, Clone)]
pub struct Config<'a> {
    name: &'a str,
    author: &'a str,
    version: &'a str,
    about: &'a str,
    elements: Vec<Element<'a>>,
}

impl<'a> Config<'a> {
    pub fn new() -> Self {
        Self {
            name: "",
            author: "",
            version: "",
            about: "",
            elements: vec![],
        }
    }
    pub fn name(mut self, name: &'a str) -> Self {
        self.name = name;
        self.add(Element::new(""));
        self
    }
    pub fn author(mut self, author: &'a str) -> Self {
        self.author = author;
        self
    }
    pub fn version(mut self, version: &'a str) -> Self {
        self.version = version;
        self
    }
    pub fn about(mut self, about: &'a str) -> Self {
        self.about = about;
        self
    }
    pub fn path(&self) -> PathBuf {
        data().join(self.name)
    }
    pub fn add(&mut self, mut element: Element<'a>) -> Self {
        if element.name == "" {
            element.path(data().join(self.name));
            element.name = self.name
        } else {
            element.path(self.path());
        }
        for mut child in &mut element.children {
            child.path(element.path.clone());
            Config::fill_paths(&mut child);
        }
        self.elements.push(element);
        self.clone()
    }

    fn fill_paths(element: &mut Element) {
        for mut child in &mut element.children {
            child.path(element.path.clone());
            if !child.children.is_empty() {
                Config::fill_paths(&mut child)
            } else {
                continue;
            }
        }
    }
    pub fn write(self) -> Result<Self> {
        for child in &self.elements {
            Config::write_recursive(child)?;
        }
        Ok(self.clone())
    }
    fn write_recursive(element: &Element) -> Result<()> {
        element.write()?;
        for child in &element.children {
            child.write()?;
            if !child.children.is_empty() {
                Config::write_recursive(&child)?;
            } else {
                continue;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Element<'a> {
    name: &'a str,
    path: PathBuf,
    format: Format,
    children: Vec<Element<'a>>,
}

impl<'a> Element<'a> {
    pub fn new(name: &'a str) -> Self {
        Element {
            name,
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
        self.path = path.join(self.name);
        path
    }
    pub fn child(mut self, element: Element<'a>) -> Self {
        self.children.push(element);
        self
    }
    pub fn write(&self) -> Result<Self> {
        match &self.format {
            Format::Directory => {
                if !&self.path.exists() {
                    DirBuilder::new()
                        .recursive(false)
                        .create(&self.path)?;
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

#[derive(Debug, Clone)]
pub enum Format {
    Directory,
    File,
}

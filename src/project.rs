use crate::file::File;
use crate::format::FileFormat;
use anyhow::{anyhow, Context, Ok, Result};
use directories::{ProjectDirs, ProjectDirsExt};
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    path::PathBuf,
};

/// Stores information about the app and the file structure.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Project {
    /// The app's qualifier
    pub qualifier: String,
    /// The app's organization
    pub organization: String,
    /// Application name.
    pub application: String,
    /// Application author.
    pub author: String,
    /// Application version.
    pub version: String,
    /// Application information.
    pub about: String,
}

/// Project is a data structure that encapsulates the most important information about your application.
impl Project {
    /// Initializes a new configuration for the app.
    ///
    /// Sets the name for the app, includes the base directory and `app.toml` to Project's children.
    ///
    /// The `app.toml` file contains the Project structure represented as TOML.
    ///
    /// Example:
    /// ```rust
    /// use libset::project::Project;
    /// use anyhow::Result;
    ///
    /// fn main() -> Result<()> {
    ///     let config: Project = Project::new("com", "organization", "App");
    ///     Ok(())
    /// }
    /// ```
    pub fn new(qualifier: &str, organization: &str, application: &str) -> Self {
        let project = Self {
            qualifier: qualifier.to_string(),
            organization: organization.to_string(),
            application: application.to_string(),
            author: "".to_string(),
            version: "".to_string(),
            about: "".to_string(),
        };

        if let Some(project_dir) = ProjectDirs::from(qualifier, organization, application) {
            std::fs::create_dir_all(project_dir.data_dir())
                .expect("Failed to create the projects directory");
            Project::update(&project).unwrap();
        }
        project
    }

    fn update(project: &Project) -> Result<()> {
        let project_dir = ProjectDirs::from(
            &project.qualifier,
            &project.organization,
            &project.application,
        )
        .context("Project directory doesn't exist")?;
        let path = project_dir.data_dir().join(format!(
            "{}.{}.{}.toml",
            project.qualifier, project.organization, project.application
        ));
        let mut file = std::fs::File::create(path).expect("Failed to create app file");
        let content = toml::to_string(&project).expect("Failed to serialize project.");
        file.write_all(content.as_bytes())
            .expect("Failed to write to the app file.");
        Ok(())
    }

    /// Returns a new instance of the configuration for the app.
    ///
    /// Sets the name for the app, includes the base directory and `app.toml` to Project's children.
    ///
    /// The `app.toml` file contains the Project structure represented as TOML.
    ///
    /// Example:
    /// ```rust
    /// use libset::project::Project;
    /// use anyhow::Result;
    ///
    /// fn main() -> Result<()> {
    ///     let config: Project = Project::new("com", "organization", "App");
    ///     Ok(())
    /// }
    /// ```
    pub fn open(qualifier: &str, organization: &str, application: &str) -> Result<Self> {
        if let Some(project) = ProjectDirs::from(qualifier, organization, application) {
            let mut file = std::fs::File::open(
                project
                    .data_dir()
                    .join(format!("{qualifier}.{organization}.{application}.toml")),
            )?;
            let mut buffer = String::new();

            file.read_to_string(&mut buffer)?;

            let project: Project = toml::from_str(buffer.as_str())?;
            Ok(project)
        } else {
            Err(anyhow!("Failed to open project"))
        }
    }

    /// Sets the author of the program.
    ///
    /// ```rust
    /// use libset::project::Project;
    ///
    /// let config: Project = Project::new("com", "organization", "App").author("Your Name");
    /// ```
    pub fn author(mut self, author: &str) -> Self {
        self.author = author.to_string();
        Project::update(&self).unwrap();
        self
    }

    /// Sets the version of the program.
    ///
    /// ```rust
    /// use libset::project::Project;
    ///
    /// let config: Project = Project::new("com", "organization", "App").version("0.1.1");
    /// ```
    pub fn version(mut self, version: &str) -> Self {
        self.version = version.to_string();
        Project::update(&self).unwrap();
        self
    }

    /// Sets the information about the program.
    ///
    /// ```rust
    /// use libset::project::Project;
    ///
    /// let config: Project = Project::new("com", "organization", "App")
    ///     .about("This app is just for demonstration.");
    /// ```
    pub fn about(mut self, about: &str) -> Self {
        self.about = about.to_string();
        Project::update(&self).unwrap();
        self
    }

    /// Returns the base path with the name property joined.
    /// ///
    /// ```rust
    /// use std::path::PathBuf;
    /// use libset::project::Project;
    ///
    /// let config: Option<PathBuf> = Project::new("com", "organization", "App").path();
    /// ```
    pub fn path(&self) -> Option<PathBuf> {
        if let Some(project) =
            directories::ProjectDirs::from(&self.qualifier, &self.organization, &self.application)
        {
            Some(project.data_dir().to_path_buf())
        } else {
            None
        }
    }

    /// Adds files to the project's config directory
    ///
    /// ```rust
    /// use libset::project::Project;
    /// use libset::new_file;
    ///
    /// Project::new("com", "organization", "App").add_files(&[new_file!("file")]).unwrap();
    /// ```
    pub fn add_files(self, files: &[File]) -> Result<Self> {
        let project =
            directories::ProjectDirs::from(&self.qualifier, &self.organization, &self.application)
                .context("Project directory doesn't exist.")?;

        let files: Vec<File> = files
            .iter()
            .map(|file| File {
                name: file.name.clone(),
                path: project.data_dir().join(match &file.format {
                    FileFormat::TOML => format!("{}.toml", file.name),
                    FileFormat::JSON => format!("{}.json", file.name),
                    FileFormat::Plain => file.name.clone(),
                }),
                format: file.format,
                content: file.content.clone(),
            })
            .collect();

        for file in files {
            let name = match &file.format {
                FileFormat::TOML => format!("{}.toml", file.name),
                FileFormat::JSON => format!("{}.json", file.name),
                FileFormat::Plain => file.name.clone(),
            };
            if !file.path.exists() {
                project.place_data_file(&name)?;
                file.write()?;
            }
        }
        Ok(self)
    }

    /// Clears any current configuration files and directories.
    /// ```
    /// use libset::project::Project;
    /// use anyhow::{Result, Context};
    ///
    /// fn main() -> Result<()> {
    ///     let project = Project::new("com", "organization", "App");
    ///     project.clear()
    /// }
    /// ```
    pub fn clear(&self) -> anyhow::Result<()> {
        if let Some(project) =
            directories::ProjectDirs::from(&self.qualifier, &self.organization, &self.application)
        {
            std::fs::remove_dir_all(&project.data_dir())?;
            Ok(())
        } else {
            Err(anyhow!("Project directory doesn't exist."))
        }
    }

    /// Get a file from a relative path to the app's configuration directory.
    /// ```
    /// use libset::project::Project;
    /// use libset::format::FileFormat;
    ///
    /// let project = Project::new("com", "organization", "App");
    /// let files = project.find("app");
    /// println!("{:?}", files);
    /// ```
    pub fn find(&self, name: &str) -> Result<Vec<File>> {
        let project =
            directories::ProjectDirs::from(&self.qualifier, &self.organization, &self.application)
                .context("Project directory doesn't exist")?;
        let mut files = vec![];
        for entry in walkdir::WalkDir::new(project.data_dir())
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|entry| entry.file_name().to_str().unwrap().contains(name))
        {
            let file_name = entry.file_name().to_string_lossy();
            if file_name.ends_with(".toml") {
                let file = File {
                    name: file_name.to_string(),
                    path: entry.path().to_path_buf(),
                    format: FileFormat::TOML,
                    content: std::fs::read_to_string(entry.path())?,
                };
                files.push(file)
            }
            if file_name.ends_with(".json") {
                let file = File {
                    name: file_name.to_string(),
                    path: entry.path().to_path_buf(),
                    format: FileFormat::JSON,
                    content: std::fs::read_to_string(entry.path())?,
                };
                files.push(file)
            }
        }
        Ok(files)
    }

    /// Find a file in the project's directory.
    /// 
    /// ```rust
    /// use libset::project::Project;
    /// use libset::format::FileFormat;
    /// use libset::new_file;
    ///
    /// let project = Project::new("com", "organization", "App").add_files(&[new_file!("testfile")]).unwrap();
    /// let file = project.get_file("testfile", FileFormat::TOML).unwrap();
    /// println!("{file:?}");
    /// ```
    pub fn get_file(&self, name: &str, format: FileFormat) -> Result<File> {
        let project =
            directories::ProjectDirs::from(&self.qualifier, &self.organization, &self.application)
                .context("Project directory doesn't exist")?;
        let files: Vec<File> = walkdir::WalkDir::new(project.data_dir())
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|entry| entry.file_name().to_string_lossy().contains(name) && entry.file_name().to_string_lossy().contains(format.extension()))
            .map(| file | File {
                name: file.file_name().to_string_lossy().to_string(),
                path: file.path().to_path_buf(),
                format,
                content: std::fs::read_to_string(file.path()).unwrap(),
            })
            .collect();
        if files.len() == 1 {
            return Ok(files[0].clone());
        } else {
            return Err(anyhow!("Could not find the file"));
        }
    }
}

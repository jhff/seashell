use std::fs;
use std::path::PathBuf;

use super::format::SupportedFormat;

#[derive(Debug, Default, Clone)]
pub struct DirContents(Vec<DirContent>);

impl DirContents {
    pub fn new(dir: &str) -> Self {
        if let Ok(entries) = fs::read_dir(dir) {
            let contents = entries
                .filter_map(|e| e.ok())
                .filter_map(|e| DirContent::try_from(&e.path()).ok())
                .collect();

            return Self(contents);
        }

        Self(vec![])
    }

    pub fn list(&self) -> &[DirContent] {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub enum DirContent {
    Dir(PathBuf),
    AudioFile(File),
}

impl DirContent {
    pub fn name(&self) -> Option<&str> {
        match self {
            DirContent::Dir(path) => path.file_name().and_then(|name| name.to_str()),
            DirContent::AudioFile(file) => Some(file.name()),
        }
    }

    pub fn path(&self) -> &PathBuf {
        match self {
            DirContent::Dir(path) => path,
            DirContent::AudioFile(file) => file.path(),
        }
    }
}

impl TryFrom<&PathBuf> for DirContent {
    type Error = ();

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        File::try_from(path)
            .map(DirContent::AudioFile)
            .or_else(|_| {
                path.is_dir()
                    .then(|| DirContent::Dir(path.clone()))
                    .ok_or(())
            })
    }
}

#[derive(Debug, Clone)]
pub struct File {
    name: String,
    #[allow(dead_code)]
    format: SupportedFormat,
    path: PathBuf,
}

impl File {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

impl TryFrom<&PathBuf> for File {
    type Error = ();

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        path.is_file()
            .then(|| {
                let name = path.file_name()?.to_os_string().into_string().ok()?;
                let format =
                    SupportedFormat::try_from(path.extension().unwrap_or_default()).ok()?;

                Some(File {
                    name,
                    format,
                    path: path.clone(),
                })
            })
            .unwrap_or_default()
            .ok_or(())
    }
}

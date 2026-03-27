use std::path::PathBuf;

#[derive(Default, Clone)]
pub struct Excaligrep {
    pub search_query: String,
    pub selected_folder: Option<PathBuf>,
    pub search_results: Vec<SearchResult>,
    pub files: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct SearchResult {
    pub file_name: String,
    pub line: String,
}

pub struct Config {
    pub last_folder: Option<PathBuf>,
}

impl Config {
    pub fn load() -> Self {
        let config_path = Self::config_path();
        if let Some(parent) = config_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(contents) = std::fs::read_to_string(&config_path) {
            let folder = contents.trim();
            if !folder.is_empty() {
                let path = PathBuf::from(folder);
                if path.exists() && path.is_dir() {
                    return Self {
                        last_folder: Some(path),
                    };
                }
            }
        }
        Self { last_folder: None }
    }

    pub fn save(&self) {
        if let Some(folder) = &self.last_folder
            && let Some(parent) = Self::config_path().parent()
        {
            let _ = std::fs::create_dir_all(parent);
            let _ = std::fs::write(Self::config_path(), folder.to_string_lossy().as_ref());
        }
    }

    fn config_path() -> PathBuf {
        dirs::config_dir()
            .or_else(dirs::home_dir)
            .unwrap_or_else(|| PathBuf::from("."))
            .join("excaligrep")
            .join("last_folder")
    }
}

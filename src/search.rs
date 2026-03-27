use crate::app::SearchResult;
use calamine::{Data as CellData, Range, Reader, open_workbook_auto};
use std::path::{Path, PathBuf};

pub struct Searcher;

impl Searcher {
    pub fn search(folder: &Path, query: &str) -> Vec<SearchResult> {
        let csv_folder = folder.join(".csv");

        if !csv_folder.exists() {
            return vec![];
        }

        let output = std::process::Command::new("rg")
            .arg("--no-config")
            .arg(query)
            .arg(&csv_folder)
            .output();

        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                stdout
                    .lines()
                    .filter_map(|line| {
                        line.split_once(':').map(|(file_part, rest)| {
                            let file_name = std::path::Path::new(file_part)
                                .file_name()
                                .map(|s| s.to_string_lossy().to_string())
                                .unwrap_or_else(|| file_part.to_string());
                            SearchResult {
                                file_name,
                                line: rest.to_string().replace(',', " | "),
                            }
                        })
                    })
                    .collect()
            }
            Err(_) => vec![],
        }
    }

    pub fn ensure_indexed(folder: &PathBuf) {
        let csv_folder = folder.join(".csv");
        let _ = std::fs::create_dir(&csv_folder);

        let excel_files: Vec<_> = std::fs::read_dir(folder)
            .into_iter()
            .flatten()
            .flatten()
            .filter_map(|entry| {
                let path = entry.path();
                if let Some(ext) = path.extension()
                    && (ext == "xlsx" || ext == "xls")
                {
                    return Some(path);
                }
                None
            })
            .collect();

        for excel_path in excel_files {
            Self::index_file(&excel_path, &csv_folder);
        }
    }

    pub fn get_excel_files(folder: &PathBuf) -> Vec<String> {
        std::fs::read_dir(folder)
            .into_iter()
            .flatten()
            .flatten()
            .filter_map(|entry| {
                let path = entry.path();
                if let Some(ext) = path.extension()
                    && (ext == "xlsx" || ext == "xls")
                {
                    return path.file_name().map(|s| s.to_string_lossy().to_string());
                }
                None
            })
            .collect()
    }

    fn index_file(excel_path: &PathBuf, csv_folder: &Path) {
        let excel_mtime = std::fs::metadata(excel_path)
            .ok()
            .and_then(|m| m.modified().ok());

        let excel_path_buf = excel_path.clone();
        let workbook = open_workbook_auto(excel_path);

        if let Ok(mut workbook) = workbook {
            let sheet_names = workbook.sheet_names().to_vec();
            for sheet_name in sheet_names.iter() {
                let csv_path = csv_folder.join(format!(
                    "{}_{}.csv",
                    excel_path_buf
                        .file_stem()
                        .unwrap_or_default()
                        .to_string_lossy(),
                    sheet_name.replace(' ', "_")
                ));

                let should_write = if csv_path.exists() {
                    match (
                        excel_mtime,
                        csv_path.metadata().ok().and_then(|m| m.modified().ok()),
                    ) {
                        (Some(em), Some(cm)) => em > cm,
                        _ => true,
                    }
                } else {
                    true
                };

                if should_write && let Ok(range) = workbook.worksheet_range(sheet_name) {
                    let csv_content = Self::range_to_csv(&range);
                    let _ = std::fs::write(&csv_path, csv_content);
                }
            }
        }
    }

    fn range_to_csv(range: &Range<CellData>) -> String {
        let mut csv_content = String::new();
        for row in range.rows() {
            let row_strs: Vec<String> = row
                .iter()
                .map(|cell| {
                    let s = match cell {
                        calamine::Data::Int(i) => i.to_string(),
                        calamine::Data::Float(f) => f.to_string(),
                        calamine::Data::String(s) => s.clone(),
                        calamine::Data::Bool(b) => b.to_string(),
                        calamine::Data::DateTime(dt) => format!("{}", dt),
                        calamine::Data::DateTimeIso(s) => s.clone(),
                        calamine::Data::DurationIso(s) => s.clone(),
                        calamine::Data::Error(e) => format!("{:?}", e),
                        calamine::Data::Empty => String::new(),
                    };
                    if s.contains(',') || s.contains('"') || s.contains('\n') {
                        format!("\"{}\"", s.replace('"', "\"\""))
                    } else {
                        s
                    }
                })
                .collect();
            csv_content.push_str(&row_strs.join(","));
            csv_content.push('\n');
        }
        csv_content
    }
}

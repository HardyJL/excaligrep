mod app;
mod search;

use app::{Config, Excaligrep, SearchResult};
use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::{Element, Length, Task, Theme};
use search::Searcher;
use std::path::PathBuf;

#[derive(Debug, Clone)]
enum Message {
    SearchInputChanged(String),
    SelectFolderPressed,
    FolderSelected(Option<PathBuf>),
    SearchPressed,
    LiveSearch,
    SearchFinished(Vec<SearchResult>),
}

pub fn main() -> iced::Result {
    let config = Config::load();
    let folder = config.last_folder.clone();
    let files = folder.as_ref().map(Searcher::get_excel_files).unwrap_or_default();
    let app = Excaligrep {
        selected_folder: folder,
        files,
        ..Default::default()
    };
    iced::application(move || app.clone(), Excaligrep::update, Excaligrep::view)
        .theme(Theme::Dark)
        .run()
}

impl Excaligrep {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SearchInputChanged(query) => {
                self.search_query = query.clone();
                if query.is_empty() {
                    self.search_results.clear();
                    return Task::none();
                }
                if self.selected_folder.is_none() {
                    return Task::none();
                }
                Task::perform(async move { query }, |_| Message::LiveSearch)
            }

            Message::SelectFolderPressed => Task::perform(
                async {
                    rfd::AsyncFileDialog::new()
                        .set_title("Select Folder")
                        .pick_folder()
                        .await
                        .map(|f| f.path().to_path_buf())
                },
                Message::FolderSelected,
            ),

            Message::FolderSelected(folder) => {
                if let Some(path) = folder {
                    self.selected_folder = Some(path.clone());
                    self.files = Searcher::get_excel_files(&path);
                    Config { last_folder: Some(path) }.save();
                }
                Task::none()
            }

            Message::SearchPressed => {
                if self.selected_folder.is_none() {
                    self.search_results = vec![SearchResult {
                        file_name: "Error".to_string(),
                        line: "Please select a folder first.".to_string(),
                    }];
                    return Task::none();
                }

                self.search_results = vec![SearchResult {
                    file_name: "Indexing".to_string(),
                    line: "Converting Excel files to CSV...".to_string(),
                }];

                let folder = self.selected_folder.clone().unwrap();
                let query = self.search_query.clone();

                Task::perform(
                    async move {
                        Searcher::ensure_indexed(&folder);
                        Searcher::search(&folder, &query)
                    },
                    Message::SearchFinished,
                )
            }

            Message::LiveSearch => {
                let folder = match &self.selected_folder {
                    Some(f) => f.clone(),
                    None => return Task::none(),
                };

                if !folder.join(".csv").exists() {
                    return Task::none();
                }

                let query = self.search_query.clone();
                Task::perform(
                    async move { Searcher::search(&folder, &query) },
                    Message::SearchFinished,
                )
            }

            Message::SearchFinished(results) => {
                self.search_results = if results.is_empty() {
                    vec![SearchResult {
                        file_name: "No results".to_string(),
                        line: "No matches found.".to_string(),
                    }]
                } else {
                    results
                };
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let top_bar = container(
            row![
                text_input("Search...", &self.search_query)
                    .on_input(Message::SearchInputChanged)
                    .on_submit(Message::SearchPressed)
                    .padding(10),
                button("Search")
                    .on_press(Message::SearchPressed)
                    .padding(10),
            ]
            .spacing(10),
        )
        .padding(10);

        let sidebar = container(
            column![
                text("Folder").size(18),
                button("Select Folder").on_press(Message::SelectFolderPressed),
                text(
                    self.selected_folder
                        .as_ref()
                        .map(|p| format!("{:?}", p))
                        .unwrap_or_else(|| "No folder".to_string())
                )
                .size(12),
                text("Files:").size(14),
                scrollable(
                    column(
                        self.files.iter().map(|f| text(f).size(11).into())
                    )
                    .spacing(2)
                )
                .height(Length::Fill),
            ]
            .spacing(15)
            .width(Length::Fixed(250.0)),
        )
        .padding(15);

        let results = scrollable(
            column(
                self.search_results.iter().map(|res| {
                    let segments = build_text_segments(&res.line, &self.search_query);
                    container(
                        column![
                            text(&res.file_name).size(12),
                            segments,
                        ]
                        .spacing(4),
                    )
                    .padding(12)
                    .into()
                }))
            .spacing(8)
            .padding(10),
        );

        column![top_bar, row![sidebar, results]].into()
    }
}

fn build_text_segments<'a>(line: &'a str, query: &str) -> iced::widget::Row<'a, Message> {
    if query.is_empty() {
        return row![text(line).size(14)];
    }

    let query_lower = query.to_lowercase();
    let line_lower = line.to_lowercase();
    
    let mut segments = iced::widget::Row::new();
    let mut last_pos = 0;
    let highlight_color = iced::Color::from_rgb(1.0, 0.3, 0.3);
    
    for (pos, _) in line_lower.match_indices(&query_lower) {
        if pos > last_pos {
            segments = segments.push(text(&line[last_pos..pos]).size(14));
        }
        segments = segments.push(
            text(&line[pos..pos + query.len()])
                .size(14)
                .color(highlight_color)
        );
        last_pos = pos + query.len();
    }
    
    if last_pos < line.len() {
        segments = segments.push(text(&line[last_pos..]).size(14));
    }
    
    segments
}

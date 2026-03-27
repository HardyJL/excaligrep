# Excaligrep

A desktop application for searching through Excel files. Built with Rust and Iced.

## Features

- Select a folder containing Excel files (.xlsx, .xls)
- Search across all sheets in all Excel files
- Real-time search with live results
- Highlights matching text in results
- Persists last selected folder

## Requirements

- Rust (latest stable)
- ripgrep (`rg`) must be installed and in PATH

## Build

```bash
cargo build --release
```

## Usage

1. Run the application
2. Click "Select Folder" to choose a directory containing Excel files
3. Type your search query in the search box
4. Press Enter or click "Search" to search

Excel files are automatically converted to CSV for indexing. The CSV files are stored in a hidden `.csv` folder within the selected directory.

## Tech Stack

- [Iced](https://iced.rs/) - Cross-platform GUI library
- [calamine](https://github.com/tafia/calamine) - Excel file reader
- [ripgrep](https://github.com/BurntSushi/ripgrep) - Search engine

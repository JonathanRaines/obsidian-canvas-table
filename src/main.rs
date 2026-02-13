use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(name = "obsidian-canvas-table")]
#[command(about = "Generate Obsidian canvas files for literature review comparison tables", long_about = None)]
struct Args {
    /// Path to the folder containing markdown files
    #[arg(short, long)]
    folder: PathBuf,

    /// List of headings to include as columns (e.g., "Model,Data,Metrics")
    #[arg(short = 'H', long, value_delimiter = ',')]
    headings: Vec<String>,

    /// Output canvas file path
    #[arg(short, long, default_value = "output.canvas")]
    output: PathBuf,

    /// Width of each node
    #[arg(short, long, default_value = "400")]
    width: i32,

    /// Height of each node
    #[arg(short = 'e', long, default_value = "420")]
    height: i32,

    /// Horizontal spacing between columns
    #[arg(short = 'x', long, default_value = "20")]
    spacing_x: i32,

    /// Vertical spacing between rows
    #[arg(short = 'y', long, default_value = "20")]
    spacing_y: i32,

    /// Base path for relative file paths in canvas (leave empty for absolute paths)
    #[arg(short, long)]
    base_path: Option<PathBuf>,
}

#[derive(Serialize, Deserialize, Debug)]
struct CanvasNode {
    id: String,
    #[serde(rename = "type")]
    node_type: String,
    file: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    subpath: Option<String>,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Canvas {
    nodes: Vec<CanvasNode>,
    edges: Vec<serde_json::Value>,
}

fn generate_id() -> String {
    // Generate a 16-character hex ID similar to Obsidian's format
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hash, Hasher};

    let random_state = RandomState::new();
    let mut hasher = random_state.build_hasher();
    uuid::Uuid::new_v4().hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

fn find_markdown_files(folder: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();

    for entry in WalkDir::new(folder)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "md" {
                    files.push(path.to_path_buf());
                }
            }
        }
    }

    files.sort();
    files
}

fn normalize_heading(heading: &str) -> String {
    // Ensure heading starts with #
    if heading.starts_with('#') {
        heading.to_string()
    } else {
        format!("#{}", heading)
    }
}

fn make_relative_path(file_path: &Path, base_path: Option<&PathBuf>) -> String {
    if let Some(base) = base_path {
        // Try to make path relative to base
        if let Ok(rel) = file_path.strip_prefix(base) {
            return rel.to_string_lossy().to_string();
        }
    }

    // Return absolute path without leading slash for Obsidian compatibility
    let path_str = file_path.to_string_lossy().to_string();
    if path_str.starts_with('/') {
        path_str[1..].to_string()
    } else {
        path_str
    }
}

fn generate_canvas(files: Vec<PathBuf>, headings: Vec<String>, args: &Args) -> Canvas {
    let mut nodes = Vec::new();

    // Calculate column positions
    let mut column_x_positions = Vec::new();
    let mut current_x = 0;

    for _ in &headings {
        column_x_positions.push(current_x);
        current_x += args.width + args.spacing_x;
    }

    // Generate nodes for each file and heading combination
    let mut current_y = 0;

    for file_path in &files {
        for (col_idx, heading) in headings.iter().enumerate() {
            let normalized_heading = normalize_heading(heading);
            let file_str = make_relative_path(file_path, args.base_path.as_ref());

            let node = CanvasNode {
                id: generate_id(),
                node_type: "file".to_string(),
                file: file_str,
                subpath: Some(normalized_heading),
                x: column_x_positions[col_idx],
                y: current_y,
                width: args.width,
                height: args.height,
            };

            nodes.push(node);
        }

        current_y += args.height + args.spacing_y;
    }

    Canvas {
        nodes,
        edges: Vec::new(),
    }
}

fn main() {
    let args = Args::parse();

    // Validate folder exists
    if !args.folder.exists() {
        eprintln!("Error: Folder '{}' does not exist", args.folder.display());
        std::process::exit(1);
    }

    if !args.folder.is_dir() {
        eprintln!("Error: '{}' is not a directory", args.folder.display());
        std::process::exit(1);
    }

    // Validate headings
    if args.headings.is_empty() {
        eprintln!("Error: At least one heading must be provided");
        std::process::exit(1);
    }

    // Find all markdown files
    let files = find_markdown_files(&args.folder);

    if files.is_empty() {
        eprintln!(
            "Warning: No markdown files found in '{}'",
            args.folder.display()
        );
    }

    println!("Found {} markdown file(s)", files.len());
    println!("Generating canvas with {} column(s)", args.headings.len());

    // Generate canvas
    let canvas = generate_canvas(files, args.headings.clone(), &args);

    println!("Created {} node(s)", canvas.nodes.len());

    // Write to output file
    let json = serde_json::to_string_pretty(&canvas).expect("Failed to serialize canvas");
    fs::write(&args.output, json).expect("Failed to write output file");

    println!("Canvas file written to: {}", args.output.display());
}

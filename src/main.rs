use arboard::Clipboard;
use clap::Parser;
use ignore::WalkBuilder;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input folder path (default: current directory)
    #[arg(short, long)]
    input: Option<PathBuf>,

    /// Output file path (default: print to terminal)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Copy output to clipboard
    #[arg(short, long)]
    clipboard: bool,

    /// Process hidden files and folders
    #[arg(short, long)]
    all_hidden_files_and_folders: bool,

    /// Ignore files and folders in .gitignore (default: true)
    #[arg(short = 'g', long, default_value_t = true)]
    respect_gitignore: bool,
}

fn main() {
    let args: Args = Args::parse();
    let mut result = String::new();

    // Determine input folder
    let input_folder_path = args
        .input
        .unwrap_or_else(|| std::env::current_dir().unwrap());
    if !input_folder_path.is_dir() {
        panic!("Input path must be a folder");
    }
    println!("Input folder: {}", input_folder_path.display());

    // process folder
    process_folder(
        &input_folder_path,
        &input_folder_path,
        args.all_hidden_files_and_folders,
        args.respect_gitignore,
        &mut result,
    );

    // Handle output
    if let Some(output_path) = args.output {
        std::fs::write(output_path, &result).expect("Failed to write output file");
    } else {
        println!("{}", result);
    }

    // Copy to clipboard if requested
    if args.clipboard {
        let mut clipboard = Clipboard::new().expect("Failed to initialize clipboard");
        clipboard
            .set_text(result)
            .expect("Failed to copy to clipboard");
        println!("Result copied to clipboard");
    }
}

fn is_utf8(file_path: &Path) -> bool {
    if let Ok(mut file) = fs::File::open(file_path) {
        let mut buffer = Vec::new();
        if file.read_to_end(&mut buffer).is_ok() {
            return String::from_utf8(buffer).is_ok();
        }
    }
    false
}

fn process_folder(
    path: &Path,
    starting_path: &Path,
    hidden_files_and_folders: bool,
    respect_gitignore: bool,
    result: &mut String,
) {
    // iterate over files and folders:
    // if file add shortend path to result, add """, add content to result, add """
    // if folder process folder
    let walker = WalkBuilder::new(path)
        .hidden(!hidden_files_and_folders)
        .git_ignore(respect_gitignore)
        .build();

    for entry in walker {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if path.is_file() && is_utf8(path) {
                    if let Ok(file_content) = fs::read_to_string(path) {
                        let relative_path = path.strip_prefix(starting_path).unwrap_or(path);
                        result.push_str(&format!("\"{}\"\n", relative_path.to_str().unwrap()));
                        result.push_str(&format!(
                            "\"\"\"\n{}\n\"\"\"\n\n",
                            file_content.replace("\"", "\\\"")
                        ));
                    }
                }
            }
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}

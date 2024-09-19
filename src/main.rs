use clap::Parser;
use std::path::{Path, PathBuf};
use std::fs;
use arboard::Clipboard;
use std::io::Read;

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
}

fn main() {
    let args: Args = Args::parse();
    let mut result = String::new();
    
    // Determine input folder
    let input_folder_path = args.input.unwrap_or_else(|| std::env::current_dir().unwrap());
    if !input_folder_path.is_dir() {
        panic!("Input path must be a folder");
    }
    println!("Input folder: {}", input_folder_path.display());

    // process folder
    process_folder(&input_folder_path, &input_folder_path, args.all_hidden_files_and_folders, &mut result);

    // Handle output
    if let Some(output_path) = args.output {
        std::fs::write(output_path, &result).expect("Failed to write output file");
    } else {
        println!("{}", result);
    }

    // Copy to clipboard if requested
    if args.clipboard {
        let mut clipboard = Clipboard::new().expect("Failed to initialize clipboard");
        clipboard.set_text(result).expect("Failed to copy to clipboard");
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


fn process_folder(path: &Path, starting_path: &Path, hidden_files_and_folders: bool, result: &mut String) {
    // iterate over files and folders:
    // if file add shortend path to result, add """, add content to result, add """
    // if folder process folder
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        // hidden files and folders
        if !path.file_name().unwrap().to_str().unwrap().starts_with('.') || hidden_files_and_folders {
            if path.is_file() && is_utf8(&path) {
                // println!("Processed file: {:?}", path);
                if let Ok(file_content) = fs::read_to_string(&path) {
                    let relative_path = path.strip_prefix(starting_path).unwrap_or(&path);
                    result.push_str(&format!("\"{}\"\n", relative_path.to_str().unwrap()));
                    result.push_str(&format!("\"\"\"\n{}\n\"\"\"\n\n", file_content.replace("\"", "\\\"")));
                }
            } else if path.is_dir() {
                process_folder(&path, starting_path, hidden_files_and_folders, result);
            } else {
                // println!("Processed some other file: {:?}\n", path);
            }
        }
    }
}

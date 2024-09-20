# Concat all Files in a Folder

Does what it says :)

## Usage
- Input folder path as argument.
- Default output is to terminal.
- `-c` for output to clipboard
- `-o` for output in file
- `-a`  for process all hidden files and folders
- `-h`  for help


## Example

```terminal
>>> concat_all_files_in_folder -i .\src
```
  
```terminal
src/main.rs
"""
fn main() {
  println!("Hello, World!");
}
"""

src/subfolder/helper.rs
"""
fn helper() {
  println!("Helping!");
}
"""
```

use std::fs;
use std::io;
use std::path::Path;

/*
fn visit_dirs(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                println!("Dir: {}", path.display());
                visit_dirs(&path)?;
            } else {
                println!("File: {}", path.display());
            }
        }
    }
    Ok(())
}
*/

fn main() {
    println!("Hello, world!");
    let root = Path::new("/Users/kiransaisubramanyamkalimisetty/Projects/rust-file-sorter");
    
    if root.is_dir() {
        println!()
    }
}

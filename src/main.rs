use std::cmp::Reverse;
use std::env;
use std::fs::{self, DirEntry};
use std::io::{self, ErrorKind};
use std::path::{Path};
use std::process;
use std::time::{SystemTime};

// walk a directory only visiting files
fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

struct Entry {
    path: String,
    mtime: u64,
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} [number]", args[0]);
        process::exit(1);
    }

    let number_of_entries_to_print: usize = args[1].parse().unwrap();
    let current_dir = env::current_dir()?;
    let parent = current_dir.parent().unwrap();
    let mut entries: Vec<Entry> = vec!();

    let mut callback = |entry: &DirEntry| {
        let path = entry.path();
        let metadata = fs::metadata(&path).unwrap();
        let mtime = metadata.modified().unwrap().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        // I don't understand lifetimes so let's make the map own the `path` by cloning it
        let entry = Entry { path: path.as_path().to_str().unwrap().to_string(), mtime: mtime };
        entries.push(entry);
    };

    if let Ok(_) = visit_dirs(&parent, &mut callback) {
        // Reverse sort so that highest (most recent) mtimes are first
        entries.sort_by_key(|e| Reverse(e.mtime));

        for e in &entries[..number_of_entries_to_print] {
            println!("{}", e.path);
        }

        Ok(())
    } else {
        eprintln!("Something went wrong when searching for files");
        Err(io::Error::new(ErrorKind::Other, "oh no!"))
    }
}

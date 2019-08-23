use std::cmp::Reverse;
use std::env;
use std::fs::{self, DirEntry};
use std::io::{self, ErrorKind};
use std::path::{Path};
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

fn build_entry(entry: &DirEntry) -> Entry {
    let path = entry.path();
    let metadata = fs::metadata(&path).unwrap();
    let mtime = metadata.modified().unwrap().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    // I don't understand lifetimes so let's make the map own the `path` by cloning it
    Entry { path: path.as_path().to_str().unwrap().to_string(), mtime: mtime }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let number_of_entries_to_print: usize = if args.len() < 2 {
        10
    } else {
        args[1].parse().unwrap()
    };

    let current_dir = env::current_dir()?;
    let mut entries: Vec<Entry> = vec!();

    let mut callback = |entry: &DirEntry| {
        entries.push(build_entry(entry));
    };

    if let Ok(_) = visit_dirs(&current_dir, &mut callback) {
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

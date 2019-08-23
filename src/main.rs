use std::cmp::Reverse;
use std::env;
use std::io::{self, ErrorKind};
use std::time::{SystemTime};
use walkdir::WalkDir;

struct Entry {
    path: String,
    mtime: u64,
}

fn build_entry(direntry: &walkdir::DirEntry) -> io::Result<Entry> {
    let path = direntry.path();
    let metadata = direntry.metadata()?;
    if let Ok(duration) = metadata.modified()?.duration_since(SystemTime::UNIX_EPOCH) {
        // I don't understand lifetimes so let's make the map own the `path` by cloning it
        Ok(Entry { path: path.to_str().unwrap().to_string(), mtime: duration.as_secs() })
    } else {
        Err(io::Error::new(ErrorKind::Other, "Could not convert duration"))
    }
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

    for direntry in WalkDir::new(&current_dir) {
        if let Ok(direntry) = direntry {
            if let Ok(entry) = build_entry(&direntry) {
                entries.push(entry);
            } else {
                eprintln!("Failed building entry");
            }
        } else {
            eprintln!("Failed finding entry");
        }
    }

    let leading_path = current_dir.to_str().unwrap();
    // Reverse sort so that highest (most recent) mtimes are first
    entries.sort_by_key(|e| Reverse(e.mtime));

    for e in &entries[..number_of_entries_to_print] {
        println!("{}", &e.path[leading_path.len() + 1..]);
    }

    Ok(())
}

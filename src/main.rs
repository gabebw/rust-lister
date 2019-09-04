use jwalk::{DirEntry, WalkDir};
use std::cmp::{self, Reverse};
use std::env;
use std::error::Error;
use std::fs::Metadata;
use std::io;
use std::path::{PathBuf};
use std::time::{SystemTime};

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn is_file(entry: &DirEntry) -> bool {
    entry.file_type.as_ref().map(|f| f.is_file()).unwrap_or(false)
}

fn mtime_result(e: &DirEntry) -> Result<u64, Box<dyn Error>> {
    let metadata: Option<&Metadata> = e.metadata.as_ref().unwrap().as_ref().ok();
    if let Some(metadata) = metadata {
        Ok(metadata.modified()?.duration_since(SystemTime::UNIX_EPOCH)?.as_secs())
    } else {
        Err(Box::new(io::Error::new(io::ErrorKind::Other, "Couldn't get metadata")))
    }
}

fn mtime(e: &DirEntry, default: u64) -> u64 {
    mtime_result(e).unwrap_or(default)
}

fn build_entries(current_dir: &PathBuf, n: usize) -> Vec<DirEntry> {
    let walker = WalkDir::new(&current_dir).skip_hidden(true).preload_metadata(true);
    let mut x: Vec<DirEntry> = walker.into_iter()
        // Skip items that we can't access
        .filter_map(Result::ok)
        // Skip directories
        .filter(is_file)
        .collect();

    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    x.sort_by_cached_key(|e| Reverse(mtime(e, now)));
    x.into_iter().take(n).collect()
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let maximum_number_of_entries_to_print: usize = if args.len() < 2 {
        10
    } else {
        args[1].parse().unwrap()
    };

    let current_dir = env::current_dir()?;
    let entries = build_entries(&current_dir, maximum_number_of_entries_to_print);

    let leading_path = current_dir.to_str().unwrap();
    let number_of_entries_to_print = cmp::min(maximum_number_of_entries_to_print, entries.len());

    for e in &entries[..number_of_entries_to_print] {
        let path = format!("{}", e.path().display());
        println!("{}", &path[leading_path.len() + 1..]);
    }

    Ok(())
}

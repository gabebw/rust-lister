use std::cmp::{self, Ordering};
use std::env;
use std::error::Error;
use std::io;
use std::path::{PathBuf};
use std::time::{SystemTime};
use walkdir::{DirEntry, WalkDir};

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn is_file(entry: &DirEntry) -> bool {
    entry.file_type().is_file()
}

fn compare_entries_result(a: &DirEntry, b: &DirEntry) -> Result<Ordering, Box<dyn Error>> {
    let b_time = b.metadata()?.modified()?.duration_since(SystemTime::UNIX_EPOCH)?.as_secs();
    let a_time = a.metadata()?.modified()?.duration_since(SystemTime::UNIX_EPOCH)?.as_secs();
    // Reverse sort so that highest (most recent) mtimes are first
    Ok(b_time.cmp(&a_time))
}

fn compare_entries(a: &DirEntry, b: &DirEntry) -> Ordering {
    compare_entries_result(a, b).unwrap_or(Ordering::Equal)
}

fn build_entries(current_dir: &PathBuf, n: usize) -> Vec<DirEntry> {
    let walker = WalkDir::new(&current_dir).sort_by(|a, b| compare_entries(a, b));
    walker.into_iter()
        // Don't descend into any hidden items.
        .filter_entry(|e| !is_hidden(e))
        // Skip items that we can't access
        .filter_map(Result::ok)
        // Skip directories
        .filter(is_file)
        .take(n)
        .collect()
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

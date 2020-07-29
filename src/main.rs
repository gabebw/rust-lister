use clap::{App, Arg};
use jwalk::{DirEntry, WalkDir};
use std::cmp::{min, Reverse};
use std::error::Error;
use std::fs::Metadata;
use std::io;
use std::path::PathBuf;
use std::process::exit;
use std::time::SystemTime;

#[derive(PartialEq, Eq)]
enum SortBy {
    Modified,
    Created,
}

fn is_file(entry: &DirEntry) -> bool {
    entry
        .file_type
        .as_ref()
        .map(|f| f.is_file())
        .unwrap_or(false)
}

fn metadata_result<F>(e: &DirEntry, process: F) -> Result<u64, Box<dyn Error>>
where
    F: Fn(&Metadata) -> io::Result<SystemTime>,
{
    let metadata: Option<&Metadata> = e.metadata.as_ref().unwrap().as_ref().ok();
    if let Some(metadata) = metadata {
        Ok(process(metadata)?
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs())
    } else {
        Err(Box::new(io::Error::new(
            io::ErrorKind::Other,
            "Couldn't get metadata",
        )))
    }
}

fn mtime(e: &DirEntry, default: u64) -> u64 {
    metadata_result(e, |metadata| metadata.modified()).unwrap_or(default)
}

fn ctime(e: &DirEntry, default: u64) -> u64 {
    metadata_result(e, |metadata| metadata.created()).unwrap_or(default)
}

fn build_entries(current_dir: &PathBuf, n: usize, sort_by: SortBy) -> Vec<DirEntry> {
    // Use a maximum of 4 threads. Never use more than half of the available CPU cores.
    let num_threads = min(4, num_cpus::get() / 2);
    let walker = WalkDir::new(&current_dir)
        .skip_hidden(true)
        .preload_metadata(true)
        .num_threads(num_threads);
    let mut x: Vec<DirEntry> = walker
        .into_iter()
        // Skip items that we can't access
        .filter_map(Result::ok)
        // Skip directories
        .filter(is_file)
        .collect();

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    x.sort_by_cached_key(|e| match sort_by {
        SortBy::Modified => Reverse(mtime(e, now)),
        SortBy::Created => Reverse(ctime(e, now)),
    });
    x.into_iter().take(n).collect()
}

fn maximum_number_of_entries_to_print(args: Option<&str>) -> usize {
    match args {
        None => 10,
        Some(s) => match s.parse() {
            Err(_) => {
                eprintln!("Please pass a number to -n/--number-of-results.");
                exit(1);
            }
            Ok(n) => n,
        },
    }
}

fn main() -> io::Result<()> {
    let matches = App::new("Lister")
        .version("0.2")
        .author("Gabe <gabebw@gabebw.com>")
        .about("Recursively list files by most-recently-modified or -created times")
        .arg(
            Arg::with_name("sort-by")
                .short("s")
                .long("sort-by")
                .value_name("SORT_BY")
                .help("Sort by an attribute (defaults to modified)")
                .takes_value(true)
                .possible_values(&["modified", "created"]),
        )
        .arg(
            Arg::with_name("NUMBER_OF_RESULTS")
                .short("n")
                .long("--number-of-results")
                .value_name("N")
                .help("Number of results to print (default 10)"),
        )
        .arg(
            Arg::with_name("DIRECTORY")
                .help("Directory to walk through (defaults to current directory)")
                .index(1),
        )
        .get_matches();
    let sort_by = match matches.value_of("sort-by").unwrap_or("modified") {
        "created" => SortBy::Created,
        "modified" => SortBy::Modified,
        _ => SortBy::Modified,
    };

    let max_num_entries = maximum_number_of_entries_to_print(matches.value_of("NUMBER_OF_RESULTS"));
    let dir = PathBuf::from(matches.value_of("DIRECTORY").unwrap_or("."));
    let entries = build_entries(&dir, max_num_entries, sort_by);
    let leading_path = dir.to_str().unwrap();
    let number_of_entries_to_print = min(max_num_entries, entries.len());

    for e in &entries[..number_of_entries_to_print] {
        let path = format!("{}", e.path().display());
        println!("{}", &path[leading_path.len() + 1..]);
    }

    Ok(())
}
